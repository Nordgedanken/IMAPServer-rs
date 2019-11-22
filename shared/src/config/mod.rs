use log::error;
use rand::distributions::Alphanumeric;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::fs::metadata;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub shared_secret: String,
    pub mailbox_root: String,
}

impl Config {
    pub async fn new() -> Option<Self> {
        let mut rng = StdRng::from_entropy();
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
        let mut file = File::create("./Config.yml")
            .await
            .expect("unable to create file");
        let wrote = file.write_all(c.as_bytes()).await;

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

    pub async fn load() -> Option<Self> {
        let metadata = metadata("./Config.yml").await;
        match metadata {
            Ok(metadata) => {
                if metadata.is_file() {
                    let mut file = File::open("./Config.yml")
                        .await
                        .expect("unable to open file");
                    let mut data = String::new();
                    file.read_to_string(&mut data)
                        .await
                        .expect("unable to read config file");

                    let deserialized_config: Self = serde_yaml::from_str(&data)
                        .expect("unable to make struct from config content");

                    return Some(deserialized_config);
                } else {
                    return Config::new().await;
                }
            }

            Err(e) => {
                error!("{}", e);
                return Config::new().await;
            }
        }
    }
}
