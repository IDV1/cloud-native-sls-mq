// 命令行入口
pub mod server {
    use::clap;
    use clap::Parser;

    pub const DEFAULT_METADATA_DIR: &str = "config_process/metadata-server.toml";

    /// This is a high-performance, high-concurrency cloud-native queue.
    #[derive(Debug,Parser)]
    #[command(author = "IDV1", version, about, long_about = None)]
    pub struct Args{
        #[arg(short, long, default_value = DEFAULT_METADATA_DIR)]
        pub config: String,
    }
}