use std::process::Command;
use log::info;

pub fn show_log() -> Result<(), String> {
    info!("(CIT): {} - Showing git log", chrono::Utc::now());
    let status = Command::new("git")
        .args(&["log"])
        .status()
        .map_err(|e| format!("Failed to show log: {}", e))?;
    
    if status.success() {
        info!("(INF): {} - Log command successful", chrono::Utc::now());
        Ok(())
    } else {
        Err("Log command failed".into())
    }
}
