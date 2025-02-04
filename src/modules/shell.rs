use std::env;
use std::process::Command;

#[cfg(unix)]
pub fn get_shell() -> String {
    match env::var("SHELL") {
        Ok(shell_path) => shell_path.split('/').last().unwrap_or("unknown").to_string(),
        Err(_) => {
            let output = Command::new("sh")
                .arg("-c")
                .arg("echo $0")
                .output()
                .expect("Ошибка выполнения команды");

            if output.status.success() {
                String::from_utf8_lossy(&output.stdout).trim().to_string()
            } else {
                "unknown".to_string()
            }
        }
    }
}

#[cfg(windows)]
pub fn get_shell() -> String {
    if let Ok(pwsh) = env::var("PSModulePath") {
        if !pwsh.is_empty() {
            return "PowerShell".to_string();
        }
    }
    match env::var("ComSpec") {
        Ok(comspec) => comspec.split('\\').last().unwrap_or("cmd").to_string(),
        Err(_) => "unknown".to_string(),
    }
}