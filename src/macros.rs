macro_rules! mail {
    ($msg:expr) => ({
        println!("log({}): {}", state, $msg);
    })
}
