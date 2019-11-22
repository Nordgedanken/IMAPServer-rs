use std::fs;

use log::error;
use rand::distributions::Alphanumeric;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub shared_secret: String,
    pub mailbox_root: String,
}

impl Config {
    pub fn new() -> Option<Self> {
        let mut rng = thread_rng();

        // This does need to stay mutable even when the compiler says otherwise. If it is not mut it fails to generate random numbers
        #[allow(unused_mut)]
        let mut random_string: String;

        if rng.gen() {
            random_string = rng.sample_iter(&Alphanumeric).take(100).collect::<String>();
        } else {
            return None;
        }

        let config = Self {
            shared_secret: random_string,
            mailbox_root: "./mailbox_root".to_string(),
        };

        // TODO consider using /etc/ImapServer/Config.yml instead
        let c = serde_yaml::to_string(&config).expect("unable to convert config to string");
        let wrote = fs::write("./Config.yml", c);

        match wrote {
            Ok(_) => {
                return Some(config);
            }
            Err(e) => {
                error!("{}", e);
                return None;
            }
        }
    }

    pub fn load() -> Option<Self> {
        let metadata = fs::metadata("./Config.yml");
        match metadata {
            Ok(metadata) => {
                if metadata.is_file() {
                    let data = fs::read_to_string("./Config.yml").expect("Unable to read config");

                    let deserialized_config: Self = serde_yaml::from_str(&data)
                        .expect("unable to make struct from config content");

                    return Some(deserialized_config);
                } else {
                    return Config::new();
                }
            }

            Err(e) => {
                error!("{}", e);
                return Config::new();
            }
        }
    }
}
