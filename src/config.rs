#[derive(Deserialize)]
struct Config {
    ip: String,
    db: DB,
}

#[derive(Deserialize)]
struct DB {
    ip: String,
    port: Option<u16>,
    username: String,
    password: String,
}