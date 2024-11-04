use std::process::Command;
use log::info;

pub fn add_all() -> Result<(), String> {
    info!("(CIT): {} - Adding all changes", chrono::Utc::now());
    let status = Command::new("git")
        .args(&["add", "."])
        .status()
        .map_err(|e| format!("Failed to add changes: {}", e))?;
    
    if status.success() {
        info!("(INF): {} - Add command successful", chrono::Utc::now());
        Ok(())
    } else {
        Err("Add command failed".into())
    }
}
