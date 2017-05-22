use std;
use config::Config;
pub fn start_config_dir() -> std::path::PathBuf {
    use app_dirs::*;
    const APP_INFO: AppInfo = AppInfo { name: "IMAPServer", author: "MTRNord" };
    let config_root = app_root(AppDataType::UserConfig, &APP_INFO).unwrap();
    println!("{:?}", config_root);
    config_root
}

pub fn get_config() -> Config{
    use config::{File, FileFormat};
    let mut c = Config::new();
    // Add 'Main.json'
    c.merge(File::new("Main", FileFormat::Json).required(false)).unwrap();
    c
}