use std::process::Command;
use log::info;

pub fn commit_all(msg: &str) -> Result<(), String> {
    info!("(CIT): {} - Running commit with message: {}", chrono::Utc::now(), msg);
    let status = Command::new("git")
        .args(&["commit", "-m", msg])
        .status()
        .map_err(|e| format!("Failed to run commit: {}", e))?;
    
    if status.success() {
        info!("(INF): {} - Commit successful", chrono::Utc::now());
        Ok(())
    } else {
        Err("Commit command failed".into())
    }
}
