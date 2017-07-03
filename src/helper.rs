use std;
use std::path::Path;
use std::io;
use mysql as my;
use simplelog;
use simplelog::{TermLogger, WriteLogger, CombinedLogger, LogLevelFilter, SharedLogger, SimpleLogger};
use std::fs::File;

pub fn init_log() {
    let mut config_dir = get_config_dir();
    config_dir.push("IMAP.log");
    let mut logger: Vec<Box<SharedLogger>> = vec![];
    logger.push(match TermLogger::new(LogLevelFilter::Info, simplelog::Config::default()) {
        Some(termlogger) => termlogger,
        None => SimpleLogger::new(LogLevelFilter::Info, simplelog::Config::default()),
    });
    logger.push(WriteLogger::new(LogLevelFilter::Debug, simplelog::Config::default(), File::create(config_dir.to_str().unwrap()).expect("Could not create logfile")));
    CombinedLogger::init(logger).expect("Could not initialize logger");
}

fn connect_to_db() -> my::Pool {
    let pool = my::Pool::new("mysql://root:password@localhost:3307").unwrap();
    pool
}

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
    const APP_INFO: AppInfo = AppInfo {
        name: "IMAPServer",
        author: "MTRNord",
    };
    let config_root = app_root(AppDataType::UserConfig, &APP_INFO).unwrap();
    config_root
}

pub fn get_config() -> super::config::Config {
    use toml;
    use std::io::prelude::*;

    let mut config_dir = get_config_dir();
    config_dir.push("Main.yml");
    let config: super::config::Config;

    if config_dir.exists() {
        let mut file = File::open(config_dir).expect("Unable to open the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read the file");
        config = toml::from_str(contents.as_str()).unwrap();
    }else {
        use std::process;
        touch(config_dir.as_path()).expect("The Server wasn't able to save the default config. Is the dir writeable?");
        let mut f = File::create(&config_dir).expect("The Server wasn't able to save the default config. Is the dir writeable?");
        f.write_all(b"ip = '127.0.0.1'\n").unwrap();
        f.write_all(b"[db]'\n").unwrap();
        f.write_all(b"ip = '127.0.0.1'\n").unwrap();
        f.write_all(b"username = 'root'\n").unwrap();
        f.write_all(b"password = 'yyyyyyyyyyyyyyyyy'\n").unwrap();
        println!("Default config saved please edit it and restart the server");
        process::abort();
    }
    config
}
