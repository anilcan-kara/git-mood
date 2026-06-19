mod sentiment;
mod hook;
mod svg;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use chrono::{Local, Datelike};
use colored::Colorize;
use sentiment::Mood;
use svg::DayStats;

fn get_log_path() -> PathBuf {
    let home = env::var("USERPROFILE")
        .or_else(|_| env::var("HOME"))
        .unwrap_or_else(|_| ".".to_string());
    Path::new(&home).join(".git-mood.json")
}

fn load_db(path: &Path) -> HashMap<String, DayStats> {
    if !path.exists() {
        return HashMap::new();
    }
    let content = fs::read_to_string(path).unwrap_or_default();
    serde_json::from_str(&content).unwrap_or_default()
}

fn save_db(path: &Path, db: &HashMap<String, DayStats>) -> Result<(), String> {
    let content = serde_json::to_string_pretty(db).map_err(|e| e.to_string())?;
    fs::write(path, content).map_err(|e| e.to_string())
}

fn get_diff_stats() -> (usize, usize) {
    let output = Command::new("git")
        .args(["diff", "--cached", "--numstat"])
        .output();

    let mut insertions = 0;
    let mut deletions = 0;

    if let Ok(out) = output {
        if out.status.success() {
            let stdout_str = String::from_utf8_lossy(&out.stdout);
            for line in stdout_str.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let ins: usize = parts[0].parse().unwrap_or(0);
                    let del: usize = parts[1].parse().unwrap_or(0);
                    insertions += ins;
                    deletions += del;
                }
            }
        }
    }

    (insertions, deletions)
}

fn print_help() {
    println!("{}", "==============================================".cyan());
    println!("        {} - GitHub Graph Colorizer", "git-mood".green().bold());
    println!("{}", "==============================================".cyan());
    println!("Usage:");
    println!("  git-mood init           Install git hook in the current repository");
    println!("  git-mood status         Display terminal dashboard of your moods");
    println!("  git-mood sync [file]    Generate SVG (defaults to git-mood.svg) and push it");
    println!();
    println!("Hook Command (Internal):");
    println!("  git-mood commit-hook <file>  Analyze commit message file");
    println!("{}", "==============================================".cyan());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "init" => {
            println!("Initializing git-mood hook...");
            if let Err(e) = hook::install_hook() {
                eprintln!("{}: {}", "Error".red().bold(), e);
            }
        }
        "commit-hook" => {
            if args.len() < 3 {
                eprintln!("{}: Missing commit message file path", "Error".red().bold());
                return;
            }
            let msg_file = &args[2];
            let commit_msg = match fs::read_to_string(msg_file) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("{}: Failed to read commit msg: {}", "Error".red().bold(), e);
                    return;
                }
            };

            // Analyze
            let (insertions, deletions) = get_diff_stats();
            let today = Local::now().date_naive();
            let weekday = today.weekday();
            let is_weekend = weekday == chrono::Weekday::Sat || weekday == chrono::Weekday::Sun;

            let mood = sentiment::analyze_commit(&commit_msg, insertions, deletions, is_weekend);

            // Update database
            let db_path = get_log_path();
            let mut db = load_db(&db_path);
            let date_key = today.format("%Y-%m-%d").to_string();

            let mut stats = db.entry(date_key).or_insert_with(DayStats::default);
            stats.commits_count += 1;
            match mood {
                Mood::Positive => stats.positive += 1,
                Mood::Negative => stats.negative += 1,
                Mood::Neutral => stats.neutral += 1,
                Mood::Refactor => stats.refactor += 1,
                Mood::Weekend => stats.weekend += 1,
            }

            if let Err(e) = save_db(&db_path, &db) {
                eprintln!("{}: Failed to save mood: {}", "Error".red().bold(), e);
                return;
            }

            // Print feedback to the developer during their git workflow!
            let colored_mood = match mood {
                Mood::Positive => "Positive (Turquoise) 🚀".cyan().bold(),
                Mood::Negative => "Stressed / Negative (Red) ⚠️".red().bold(),
                Mood::Neutral => "Steady / Neutral (Green) 🟩".green().bold(),
                Mood::Refactor => "Refactor / Cleanup (Yellow) 🟨".yellow().bold(),
                Mood::Weekend => "Weekend Coding (Purple) 🟪".magenta().bold(),
            };

            println!();
            println!(" [git-mood] Sentiment Analyzed: {}", colored_mood);
            println!("            Diff: +{} / -{} lines", insertions, deletions);
            println!();
        }
        "status" => {
            let db_path = get_log_path();
            let db = load_db(&db_path);

            println!("{}", "\n--- Your git-mood Dashboard ---".cyan().bold());
            if db.is_empty() {
                println!("No commits tracked yet. Run `git-mood init` in a repo to start logging!");
                return;
            }

            // Print summary of last 7 days
            let today = Local::now().date_naive();
            println!("\nRecent Days:");
            for i in (0..7).rev() {
                let day = today - chrono::Duration::days(i);
                let date_str = day.format("%Y-%m-%d").to_string();
                let day_name = day.format("%A").to_string();

                if let Some(stats) = db.get(&date_str) {
                    if let Some(m) = stats.determine_mood() {
                        let block = match m {
                            Mood::Positive => "■".cyan(),
                            Mood::Negative => "■".red(),
                            Mood::Neutral => "■".green(),
                            Mood::Refactor => "■".yellow(),
                            Mood::Weekend => "■".magenta(),
                        };
                        println!("  {} ({}): {} [{} commits]", day_name, date_str, block, stats.commits_count);
                        continue;
                    }
                }
                // If no commits
                println!("  {} ({}): {} [0 commits]", day_name, date_str, "■".truecolor(80, 80, 80));
            }

            // Dominant overall mood breakdown
            let mut mood_counts = HashMap::new();
            let mut total_commits = 0;
            for stats in db.values() {
                total_commits += stats.commits_count;
                if let Some(m) = stats.determine_mood() {
                    *mood_counts.entry(m).or_insert(0) += 1;
                }
            }

            println!("\nOverall Stats:");
            println!("  Total Commits: {}", total_commits);
            for (m, count) in &mood_counts {
                let name = match m {
                    Mood::Positive => "Turquoise (Positive)".cyan(),
                    Mood::Negative => "Red (Negative)".red(),
                    Mood::Neutral => "Green (Neutral)".green(),
                    Mood::Refactor => "Yellow (Refactoring)".yellow(),
                    Mood::Weekend => "Purple (Weekend)".magenta(),
                };
                println!("  {}: {} days", name, count);
            }
            println!();
        }
        "sync" => {
            let svg_filename = if args.len() >= 3 {
                &args[2]
            } else {
                "git-mood.svg"
            };
            println!("{} Generating {}...", "[git-mood]".green(), svg_filename);
            let db_path = get_log_path();
            let db = load_db(&db_path);

            let svg_content = svg::generate_svg(&db);
            let svg_path = Path::new(svg_filename);

            if let Err(e) = fs::write(svg_path, svg_content) {
                eprintln!("{}: Failed to write SVG file: {}", "Error".red().bold(), e);
                return;
            }
            println!("{} Saved SVG to {:?}", "[git-mood]".green(), svg_path);

            // Check if git repository to perform auto-commit/push
            if Path::new(".git").exists() {
                println!("{} Detected local git repository. Committing and pushing SVG...", "[git-mood]".green());
                
                // git add
                let add_status = Command::new("git")
                    .args(["add", svg_filename])
                    .status();

                if let Ok(status) = add_status {
                    if status.success() {
                        // git commit
                        let commit_status = Command::new("git")
                            .args(["commit", "-m", "docs: update git-mood contribution graph [skip ci]", "--no-verify"])
                            .status();

                        if let Ok(c_status) = commit_status {
                            if c_status.success() {
                                println!("{} Committed changes.", "[git-mood]".green());
                                
                                // git push
                                let push_status = Command::new("git")
                                    .args(["push"])
                                    .status();

                                if let Ok(p_status) = push_status {
                                    if p_status.success() {
                                        println!("{} Successfully pushed to remote!", "[git-mood]".green());
                                    } else {
                                        println!("{} Push failed. You may need to run 'git push' manually.", "[git-mood]".yellow());
                                    }
                                }
                            } else {
                                println!("{} No new changes to commit or commit failed.", "[git-mood]".yellow());
                            }
                        }
                    }
                }
            } else {
                println!("{} Not a git repository or no .git folder found locally. Skipping git push.", "[git-mood]".yellow());
            }
        }
        _ => {
            print_help();
        }
    }
}
