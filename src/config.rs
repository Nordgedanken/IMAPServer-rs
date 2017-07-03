#[derive(Serialize, Deserialize)]
pub struct Config {
    ip: String,
    db: DB,
}

#[derive(Serialize, Deserialize)]
struct DB {
    ip: String,
    port: Option<u16>,
    username: String,
    password: String,
}