pub(crate) fn get_hostname() -> String {
    match hostname::get() {
        Ok(hostname) => hostname.to_string_lossy().to_string(),
        Err(_) => "N/A".to_string(),
    }
}