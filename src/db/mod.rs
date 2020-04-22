pub mod migration;
pub mod storage;

use std::path::PathBuf;

pub fn database_file_path() -> PathBuf {
    let mut path = dirs::data_dir().unwrap();
    path.push("rtd");
    path.push("data");
    path.set_extension("db");
    path
}
