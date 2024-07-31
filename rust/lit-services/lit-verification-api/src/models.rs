#[derive(Debug, Clone)]
pub struct CacheEntry {
    pub service_callback: String,
    pub client_ip: String,
    pub code_attempts: u32,
}
