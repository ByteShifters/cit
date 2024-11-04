use std::process::Command;
use log::info;

pub fn show_diff() -> Result<(), String> {
    info!("(CIT): {} - Showing git diff", chrono::Utc::now());
    let status = Command::new("git")
        .args(&["diff"])
        .status()
        .map_err(|e| format!("Failed to show diff: {}", e))?;
    
    if status.success() {
        info!("(INF): {} - Diff command successful", chrono::Utc::now());
        Ok(())
    } else {
        Err("Diff command failed".into())
    }
}
