use std::process::Command;
use log::info;

pub fn push_all() -> Result<(), String> {
    info!("(CIT): {} - Running push", chrono::Utc::now());
    let status = Command::new("git")
        .args(&["push"])
        .status()
        .map_err(|e| format!("Failed to run push: {}", e))?;
    
    if status.success() {
        info!("(INF): {} - Push successful", chrono::Utc::now());
        Ok(())
    } else {
        Err("Push command failed".into())
    }
}
