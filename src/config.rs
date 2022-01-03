
pub struct Config {
    pub baud: u32,
    pub timeout: u64,
    pub port: Option<String>,
    pub input: Option<std::path::PathBuf>,
}
