use std::process::Command;
use log::info;

pub fn undo_last() -> Result<(), String> {
    info!("(CIT): {} - Undoing last commit", chrono::Utc::now());
    let status = Command::new("git")
        .args(&["reset", "--soft", "HEAD~1"])
        .status()
        .map_err(|e| format!("Failed to undo last commit: {}", e))?;
    
    if status.success() {
        info!("(INF): {} - Undo successful", chrono::Utc::now());
        Ok(())
    } else {
        Err("Undo command failed".into())
    }
}
