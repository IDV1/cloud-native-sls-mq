pub mod tools{
    use std::fs::{create_dir_all, read_to_string};
    use std::io;
    use std::path::Path;
    use crate::error::error::server_error::ServerError;

    // 文件读取
    pub fn read_file(path: impl AsRef<Path>) -> Result<String, ServerError> {
        read_to_string(path)
            .map_err(|e|
                ServerError::CommonError(
                    format!("read file failed, No such file or directory: {e}")))
    }

    // 文件是否存在
    pub fn file_exists(path: impl AsRef<Path>) -> Result<bool, ServerError> {
        let path = path.as_ref();
        if path.exists() {
            Ok(true)
        }else {
            Err(ServerError::CommonError("not found file".to_string()))
        }
    }

    // 目录创建
    pub fn create_folder(fold: impl AsRef<Path>) -> io::Result<()> {
        let path = fold.as_ref();
        if path.exists() {
            create_dir_all(path)
        }else {
            Err(io::Error::new(io::ErrorKind::NotFound, "folder not found"))
        }
    }
}