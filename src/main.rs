extern crate app_dirs;
extern crate config;
fn main() {
    helper::start_config_dir();
    helper::get_config().set_default("RFC", "3501");
    println!("Hello, world!");
}

mod helper;