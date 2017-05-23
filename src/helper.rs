use std;
use config::Config;
use std::path::Path;
use std::io;

// A simple implementation of `% touch path` (ignores existing files)
// From http://rustbyexample.com/std_misc/fs.html
fn touch(path: &Path) -> io::Result<()> {
    use std::fs::OpenOptions;
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn get_config_dir() -> std::path::PathBuf {
    use app_dirs::*;
    const APP_INFO: AppInfo = AppInfo { name: "IMAPServer", author: "MTRNord" };
    let config_root = app_root(AppDataType::UserConfig, &APP_INFO).unwrap();
    config_root
}

pub fn get_config() -> Config{
    use config::{File, FileFormat};
    let mut config_dir = get_config_dir();
    config_dir.push("Main.yml");
    touch(config_dir.as_path());
    let mut c = Config::new();
    // Add 'Main.yaml'
    c.merge(File::new(config_dir.to_str().unwrap(), FileFormat::Yaml).required(true)).unwrap();
    c
}