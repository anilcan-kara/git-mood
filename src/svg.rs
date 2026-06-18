use std::collections::HashMap;
use chrono::{Duration, Local, Datelike};
use serde::{Deserialize, Serialize};
use crate::sentiment::Mood;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DayStats {
    pub commits_count: usize,
    pub positive: usize,
    pub negative: usize,
    pub neutral: usize,
    pub refactor: usize,
    pub weekend: usize,
}

impl DayStats {
    pub fn determine_mood(&self) -> Option<Mood> {
        if self.commits_count == 0 {
            return None;
        }

        // 1. If 30% or more commits are negative, mark the day negative (Red)
        if self.negative > 0 && (self.negative as f64 / self.commits_count as f64) >= 0.3 {
            return Some(Mood::Negative);
        }

        // 2. If 50% or more are refactoring, mark refactor (Yellow)
        if self.refactor > 0 && (self.refactor as f64 / self.commits_count as f64) >= 0.5 {
            return Some(Mood::Refactor);
        }

        // 3. If 50% or more are weekend commits, mark weekend (Purple)
        if self.weekend > 0 && (self.weekend as f64 / self.commits_count as f64) >= 0.5 {
            return Some(Mood::Weekend);
        }

        // 4. If positive >= neutral, mark positive (Turquoise)
        if self.positive > 0 && self.positive >= self.neutral {
            return Some(Mood::Positive);
        }

        // 5. Default is Neutral (Green)
        Some(Mood::Neutral)
    }
}

pub fn generate_svg(log_data: &HashMap<String, DayStats>) -> String {
    let today = Local::now().date_naive();
    let weekday = today.weekday().num_days_from_sunday() as i64; // Sunday = 0, Saturday = 6
    
    // Find Sunday of 52 weeks ago
    let start_date = today - Duration::days(weekday + 52 * 7);

    let box_size = 10;
    let gap = 2;
    let margin_left = 35;
    let margin_top = 45;

    let mut grid_svg = String::new();
    let mut current_date = start_date;
    
    let mut total_commits = 0;
    let mut mood_counts = HashMap::new();

    // Iterate through all days from start_date to today
    while current_date <= today {
        let date_str = current_date.format("%Y-%m-%d").to_string();
        
        let stats = log_data.get(&date_str);
        let mood = stats.and_then(|s| {
            total_commits += s.commits_count;
            s.determine_mood()
        });

        if let Some(m) = mood {
            *mood_counts.entry(m).or_insert(0) += 1;
        }

        let days_since_start = (current_date - start_date).num_days();
        let col = days_since_start / 7;
        let row = days_since_start % 7;

        let x = margin_left + col * (box_size + gap);
        let y = margin_top + row * (box_size + gap);

        let color = match mood {
            Some(Mood::Positive) => Mood::Positive.to_color_code(),
            Some(Mood::Negative) => Mood::Negative.to_color_code(),
            Some(Mood::Refactor) => Mood::Refactor.to_color_code(),
            Some(Mood::Weekend) => Mood::Weekend.to_color_code(),
            Some(Mood::Neutral) => Mood::Neutral.to_color_code(),
            None => "#161b22", // Gray
        };

        grid_svg.push_str(&format!(
            r#"<rect class="day" x="{}" y="{}" width="{}" height="{}" rx="2" fill="{}" data-date="{}" />"#,
            x, y, box_size, box_size, color, date_str
        ));
        grid_svg.push('\n');

        current_date += Duration::days(1);
    }

    // Determine dominant mood
    let dominant_mood = mood_counts
        .iter()
        .max_by_key(|&(_, count)| count)
        .map(|(mood, _)| *mood)
        .unwrap_or(Mood::Neutral);

    let dominant_mood_str = match dominant_mood {
        Mood::Positive => "Turquoise (Highly Productive)",
        Mood::Negative => "Red (Bug Squashing / Stressed)",
        Mood::Refactor => "Yellow (Cleanups & Refactoring)",
        Mood::Weekend => "Purple (Weekend Warrior)",
        Mood::Neutral => "Green (Steady Progress)",
    };

    let dominant_mood_color = dominant_mood.to_color_code();

    // Build the final SVG with a beautiful widget wrapper
    format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 710 170" width="100%" height="100%">
  <style>
    .bg {{ fill: #0d1117; rx: 8px; }}
    .title {{ font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif; font-size: 14px; fill: #adbac7; font-weight: 600; }}
    .subtitle {{ font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif; font-size: 11px; fill: #768390; }}
    .legend-text {{ font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif; font-size: 10px; fill: #768390; }}
    .day {{ transition: transform 0.1s ease; cursor: pointer; }}
    .day:hover {{ transform: scale(1.2); transform-origin: center; stroke: #539bf5; stroke-width: 1px; }}
  </style>
  <rect class="bg" width="710" height="170" />
  
  <!-- Header Info -->
  <text x="20" y="25" class="title">git-mood // Contribution Analytics</text>
  <text x="500" y="25" class="subtitle" text-anchor="end">Dominant Mood: <tspan fill="{}">{}</tspan></text>
  <text x="700" y="25" class="subtitle" text-anchor="end">Total commits tracked: {}</text>

  <!-- Contribution Grid -->
  {}

  <!-- Legend -->
  <g transform="translate(480, 145)">
    <rect x="0" y="0" width="8" height="8" rx="1" fill="#161b22" />
    <rect x="25" y="0" width="8" height="8" rx="1" fill="#00cdac" />
    <rect x="50" y="0" width="8" height="8" rx="1" fill="#00f2fe" />
    <rect x="75" y="0" width="8" height="8" rx="1" fill="#ff0844" />
    <rect x="100" y="0" width="8" height="8" rx="1" fill="#f6d365" />
    <rect x="125" y="0" width="8" height="8" rx="1" fill="#7f00ff" />
    
    <text x="140" y="8" class="legend-text">Mood Map</text>
  </g>
  
  <!-- Day labels (S M T W T F S) -->
  <g class="legend-text" transform="translate(15, 54)">
    <text x="0" y="0">S</text>
    <text x="0" y="24">T</text>
    <text x="0" y="48">T</text>
    <text x="0" y="72">S</text>
  </g>
</svg>"##,
        dominant_mood_color, dominant_mood_str, total_commits, grid_svg
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_svg_generation() {
        let mut log_data = HashMap::new();
        log_data.insert(
            "2026-06-18".to_string(),
            DayStats {
                commits_count: 5,
                positive: 4,
                negative: 0,
                neutral: 1,
                refactor: 0,
                weekend: 0,
            },
        );

        let svg = generate_svg(&log_data);
        assert!(svg.contains("git-mood // Contribution Analytics"));
        assert!(svg.contains("Dominant Mood"));
        assert!(svg.contains("2026-06-18"));
    }
}
