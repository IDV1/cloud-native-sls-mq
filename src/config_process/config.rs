// 文件配置
pub mod config{
    use serde::Deserialize;
    use crate::log::log::logging::Log;
    #[derive(Debug, Deserialize, Clone)]
    pub struct MetadataServerConfig {
        #[allow(unused)]
        #[serde(default = "default_node_id")]
        pub node_id: u32,
        #[serde(default = "default_grpc_port")]
        pub grpc_port: usize,
        pub log: Log
    }

    fn default_node_id() -> u32{
        1
    }

    fn default_grpc_port() -> usize{
        9090
    }
}

// 文件处理
pub mod config_handle{
    use std::sync::OnceLock;
    use crate::config_process::config::config::MetadataServerConfig;
    use crate::tools::utils::tools::read_file;

    // 只执行一次
    pub static CONFIG: OnceLock<MetadataServerConfig> = OnceLock::new();

    // 读取toml
    pub fn try_init_path(config_path: &str) -> &'static MetadataServerConfig {
        CONFIG.get_or_init(|| {
            let toml = read_file(config_path)
                .unwrap_or_else(|e| panic!("read config_process file failed: {e}"));
            toml::from_str(&toml)
                .unwrap_or_else(|e| panic!("parse toml failed: {e}"))
        })
    }

    // 读取检查
    pub fn config_check() -> &'static MetadataServerConfig {
        match CONFIG.get() {
            Some(config) => config,
            None => panic!("metadata-server config_process is not initialized")}
    }
}