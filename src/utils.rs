use std::process::{Command, Output};
use log::{info, error};

pub fn execute_command(cmd: &str, args: &[&str]) -> Result<Output, String> {
    info!("(CIT): {} - Executing command: {} {:?}", chrono::Utc::now(), cmd, args);
    
    let output = Command::new(cmd)
        .args(args)
        .output()
        .map_err(|e| format!("Failed to execute command {}: {}", cmd, e))?;
    
    if output.status.success() {
        info!("(INF): {} - Command {} executed successfully", chrono::Utc::now(), cmd);
        Ok(output)
    } else {
        error!("(ERR): {} - Command {} failed", chrono::Utc::now(), cmd);
        Err(format!("Command {} failed with status: {:?}", cmd, output.status))
    }
}
