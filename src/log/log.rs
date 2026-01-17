pub mod logging {
    use log::info;
    use serde::Deserialize;
    use crate::config_process::config::config::MetadataServerConfig;
    use crate::config_process::config::config_handle;


    #[derive(Clone, Debug, Deserialize)]
    #[allow(unused)]
    pub struct Log{
        pub config: String,
        pub path: String,
    }

    #[allow(unused)]
    pub fn get_config() -> &'static MetadataServerConfig {
        config_handle::config_check()
    }

    pub fn init_logger(path: &str){
        let config = log4rs::config::load_config_file(path, Default::default())
            .unwrap();
        // 初始化日志系统
        let _handle = log4rs::init_config(config).unwrap();
        info!("✓ The log initialization was successful !!")
    }
}