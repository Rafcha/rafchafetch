#[cfg(windows)]
use winapi::um::sysinfoapi::GetTickCount64;

#[cfg(unix)]
pub fn get_uptime() -> Result<u64, String> {
    // Читаем /proc/uptime на Linux
    let content = std::fs::read_to_string("/proc/uptime")
        .map_err(|e| format!("Ошибка чтения /proc/uptime: {}", e))?;
    
    let uptime_seconds = content.split_whitespace()
        .next()
        .and_then(|s| s.parse::<f64>().ok())
        .map(|t| t as u64)
        .ok_or("Неверный формат /proc/uptime")?;

    Ok(uptime_seconds)
}

#[cfg(windows)]
pub(crate) fn get_uptime() -> Result<u64, String> {
    unsafe {
        Ok(GetTickCount64() / 1000)
    }
}

pub fn format_uptime(seconds: u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;
    format!("{}d {}h {}m", days, hours, minutes)
}