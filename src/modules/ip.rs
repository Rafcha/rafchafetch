use get_if_addrs::{get_if_addrs, IfAddr};

pub fn get_ip_address() -> String {
    match get_if_addrs() {
        Ok(interfaces) => {
            for interface in interfaces {
                // Пропускаем loopback-интерфейсы
                if interface.is_loopback() {
                    continue;
                }

                // Берем первый IPv4 адрес
                if let IfAddr::V4(addr) = interface.addr {
                    return addr.ip.to_string();
                }
            }
            "N/A".to_string()
        }
        Err(_) => "N/A".to_string(),
    }
}