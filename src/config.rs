use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Config {
    mailbox_root: String,
    server_ip: String,
    log_level: String,
}
