#[derive(Serialize, Deserialize)]
pub struct Config {
    pub ip: String,
    pub db: DB,
}

#[derive(Serialize, Deserialize)]
pub struct DB {
    pub ip: String,
    pub port: Option<u16>,
    pub username: String,
    pub password: String,
}
