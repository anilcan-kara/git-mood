use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

pub fn install_hook() -> Result<(), String> {
    // Find git repo root using git rev-parse
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .map_err(|e| format!("Failed to run git: {}", e))?;

    if !output.status.success() {
        return Err("Not inside a git repository".to_string());
    }

    let repo_root_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let repo_root = Path::new(&repo_root_str);
    let hook_dir = repo_root.join(".git").join("hooks");

    if !hook_dir.exists() {
        fs::create_dir_all(&hook_dir)
            .map_err(|e| format!("Failed to create hooks directory: {}", e))?;
    }

    let hook_path = hook_dir.join("commit-msg");

    // Get current executable path
    let current_exe = std::env::current_exe()
        .map_err(|e| format!("Failed to get current executable path: {}", e))?;
    let exe_path_str = current_exe.to_string_lossy().replace('\\', "/");

    // Check if there is already a hook and back it up
    if hook_path.exists() {
        let existing_content = fs::read_to_string(&hook_path).unwrap_or_default();
        if existing_content.contains("git-mood") {
            println!("git-mood hook already installed in this repository.");
            return Ok(());
        }
        let backup_path = hook_path.with_extension("backup");
        fs::copy(&hook_path, &backup_path)
            .map_err(|e| format!("Failed to back up existing hook: {}", e))?;
        println!("Existing commit-msg hook backed up to {:?}", backup_path);
    }

    // Shell script template (works on Windows via Git Bash and Unix natively)
    let hook_content = format!(
        "#!/bin/sh\n\n# git-mood commit-msg hook\n\"{}\" commit-hook \"$1\"\n",
        exe_path_str
    );

    fs::write(&hook_path, hook_content)
        .map_err(|e| format!("Failed to write hook file: {}", e))?;

    // Make the hook executable on Unix
    #[cfg(unix)]
    {
        let mut perms = fs::metadata(&hook_path)
            .map_err(|e| format!("Failed to get hook metadata: {}", e))?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&hook_path, perms)
            .map_err(|e| format!("Failed to set hook permissions: {}", e))?;
    }

    println!("git-mood hook successfully installed at {:?}", hook_path);
    Ok(())
}
