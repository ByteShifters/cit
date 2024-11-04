use std::process::Command;
use log::info;

pub fn upload_changes(msg: &str, branch: &str) -> Result<(), String> {
    info!("(CIT): {} - Running upload with message: {}, branch: {}", chrono::Utc::now(), msg, branch);
    
    // Run `git add .`
    let add_status = Command::new("git")
        .args(&["add", "."])
        .status()
        .map_err(|e| format!("Failed to add changes: {}", e))?;
    
    if !add_status.success() {
        return Err("Add command failed".into());
    }
    
    // Run `git commit -m "commit: {msg}"`
    let commit_status = Command::new("git")
        .args(&["commit", "-m", &format!("commit: {}", msg)])
        .status()
        .map_err(|e| format!("Failed to commit changes: {}", e))?;
    
    if !commit_status.success() {
        return Err("Commit command failed".into());
    }

    // Run `git push {branch}`
    let push_status = Command::new("git")
        .args(&["push", "origin", branch])
        .status()
        .map_err(|e| format!("Failed to push changes: {}", e))?;
    
    if push_status.success() {
        info!("(INF): {} - Upload successful", chrono::Utc::now());
        Ok(())
    } else {
        Err("Push command failed".into())
    }
}
