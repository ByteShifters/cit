use std::process::Command;
use log::info;

pub fn switch_branch(branch: &str) -> Result<(), String> {
    info!("(CIT): {} - Switching to branch {}", chrono::Utc::now(), branch);
    let status = Command::new("git")
        .args(&["checkout", branch])
        .status()
        .or_else(|_| {
            info!("(CIT): {} - Creating and switching to new branch {}", chrono::Utc::now(), branch);
            Command::new("git")
                .args(&["checkout", "-b", branch])
                .status()
                .map_err(|e| format!("Failed to create and switch branch: {}", e))
        })?;
    
    if status.success() {
        info!("(INF): {} - Switch successful", chrono::Utc::now());
        Ok(())
    } else {
        Err("Switch command failed".into())
    }
}
