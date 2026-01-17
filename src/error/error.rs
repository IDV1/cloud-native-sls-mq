pub mod server_error{
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum ServerError {
        #[error("{0}")]
        CommonError(String),
        #[allow(unused)]
        #[error("{0}")]
        LoggerError(String),
    }
}