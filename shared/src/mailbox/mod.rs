use std::path::Path;

use argonautica::{Hasher, Verifier};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use futures::compat::Future01CompatExt;
use futures::StreamExt;
use log::debug;
use log::warn;
use rand::prelude::*;
use tokio::fs::{create_dir_all, metadata, read_dir};

use crate::config::Config;
use crate::database::establish_connection;
use crate::models::{NewUser, User};
use crate::schema::users;
use crate::schema::users::dsl::*;

#[derive(Clone)]
pub struct Mailbox {
    pub user: String,
    pub mailbox_root: String,
    password_hash: String,
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

impl Mailbox {
    pub async fn new(user: String, password: String) -> Option<Self> {
        let config = Config::load().await.expect("unable to load config");

        let mut hasher = Hasher::default();
        let password_hash_new = hasher
            .with_password(password)
            .with_secret_key(config.shared_secret)
            .hash_non_blocking()
            .compat()
            .await
            .expect("unable to hash password");

        let mut rng = StdRng::from_entropy();

        // This does need to stay mutable even when the compiler says otherwise. If it is not mut it fails to generate random numbers
        #[allow(unused_mut)]
            let mut random_number: String;

        if rng.gen() {
            let random_number_int: i32 = rng.gen_range(0, 2 ^ 32 - 1);

            random_number = format!("{:?}", random_number_int);
        } else {
            warn!("Unable to generate random number");
            return None;
        }

        let connection = establish_connection();
        let user_local = user.clone();
        let results: Result<User, diesel::result::Error> = users
            .filter(email.eq(&user))
            .limit(1)
            .get_result::<User>(&connection);

        match results {
            Ok(m) => {
                let mailbox_root = format!("{}/{}", config.mailbox_root, m.email);
                return Some(Mailbox {
                    mailbox_root,
                    user,
                    password_hash: m.password_hash,
                });
            }
            Err(_) => {
                let new_user = NewUser {
                    email: &user,
                    password_hash: &password_hash_new,
                    uid_validity_identifier: &random_number,
                };

                diesel::insert_into(users::table)
                    .values(&new_user)
                    .execute(&connection)
                    .expect("Failed to add new User");

                let mailbox_root = format!("{}/{}", config.mailbox_root, user);

                return Some(Mailbox {
                    mailbox_root,
                    user: user_local,
                    password_hash: password_hash_new,
                });
            }
        }
    }

    pub async fn load(user: String) -> Option<Self> {
        let connection = establish_connection();
        let config = Config::load().await.expect("unable to load config");
        let user_local = user.clone();

        let results: Result<User, diesel::result::Error> = users
            .filter(email.eq(user))
            .limit(1)
            .get_result::<User>(&connection);

        match results {
            Ok(results) => {
                let mailbox_root = format!("{}/{}", config.mailbox_root, results.email);
                Some(Mailbox {
                    mailbox_root,
                    user: user_local,
                    password_hash: results.password_hash,
                })
            }
            _ => None,
        }
    }

    pub async fn load_all() -> Option<Vec<Self>> {
        let connection = establish_connection();
        let config = Config::load().await.expect("unable to load config");

        let results: Vec<User> = users
            .load::<User>(&connection)
            .expect("Error getting Mailboxes");

        let mut returns: Vec<Self> = Vec::new();

        for entry in &results {
            let mailbox_root = format!("{}/{}", config.mailbox_root, entry.email);
            returns.push(Mailbox {
                mailbox_root,
                user: entry.email.clone(),
                password_hash: entry.password_hash.clone(),
            })
        }

        Some(returns)
    }

    pub async fn check_password_plain(&self, password: String) -> Result<(), ()> {
        let config = Config::load().await.expect("unable to load config");
        let local_hash = self.password_hash.clone();

        let mut verifier = Verifier::default();
        let verified = verifier
            .with_hash(local_hash)
            .with_password(password)
            .with_secret_key(config.shared_secret)
            .verify_non_blocking()
            .compat()
            .await;
        match verified {
            Ok(v) => {
                if v {
                    return Ok(());
                } else {
                    return Err(());
                }
            }
            Err(_) => {
                return Err(());
            }
        }
    }

    pub async fn get_lsub(&self, args: Vec<&str>) -> Option<Vec<String>> {
        if args.len() == 4 {
            debug!("get_lsub 4");
        } else if args.len() == 5 {
            debug!("get_lsub 5");
        }

        let searched = args.get(args.len() - 1).expect("unable to get searched path").replace("\"", "");

        let mut dirs = read_dir(self.mailbox_root.to_owned())
            .await
            .expect("unable to read dir");

        let mut dirs_lsub: Vec<&str> = Vec::new();

        while let Some(dir) = dirs.next().await {
            let dir = dir.expect("unable to get dir");
            // FIXME this will break with subdirs
            if dir.file_name().into_string().unwrap() != searched && searched != "*" { continue; }
            let subscribed_str: &str = "(subscribed)";
            if args.contains(&subscribed_str) {
                let path_string = format!(
                    "* LSUB (\\Subscribed) \".\" {}\r\n",
                    dir.file_name()
                        .into_string()
                        .expect("unable to get filename")
                );
                let path_string = string_to_static_str(path_string);
                dirs_lsub.push(path_string);
                continue;
            } else {
                let path_string = format!(
                    "* LSUB (\\HasNoChildren) \".\" {}\r\n",
                    dir.file_name()
                        .into_string()
                        .expect("unable to get filename")
                );
                let path_string = string_to_static_str(path_string);
                dirs_lsub.push(path_string);
                continue;
            }
        }

        if dirs_lsub.len() > 0 {
            let dirs_lsub: Vec<String> = dirs_lsub.iter().map(|s| (*s).to_string()).collect();
            Some(dirs_lsub)
        } else {
            None
        }
    }

    pub async fn get_list(&self, args: Vec<&str>) -> Option<Vec<String>> {
        if args.len() == 4 {
            debug!("get_list 4");
        } else if args.len() == 5 {
            debug!("get_list 5");
        }

        let searched = args.get(args.len() - 1).expect("unable to get searched path").replace("\"", "");

        let mut dirs = read_dir(self.mailbox_root.to_owned())
            .await
            .expect("unable to read dir");

        let mut dirs_list: Vec<&str> = Vec::new();

        while let Some(dir) = dirs.next().await {
            let dir = dir.expect("unable to get dir");
            // FIXME this will break with subdirs
            if dir.file_name().into_string().unwrap() != searched && searched != "*" { continue; }
            let subscribed_str: &str = "(subscribed)";
            if args.contains(&subscribed_str) {
                // TODO actually check if subscribed or not.
                let path_string = format!(
                    "* LIST (\\Subscribed) \".\" {}\r\n",
                    dir.file_name()
                        .into_string()
                        .expect("unable to get filename")
                );
                let path_string = string_to_static_str(path_string);
                dirs_list.push(path_string);
                continue;
            } else {
                // TODO actually check if subscribed or not.
                let path_string = format!(
                    "* LIST (\\HasNoChildren) \".\" {}\r\n",
                    dir.file_name()
                        .into_string()
                        .expect("unable to get filename")
                );
                let path_string = string_to_static_str(path_string);
                dirs_list.push(path_string);
                continue;
            }
        }

        if dirs_list.len() > 0 {
            let dirs_list: Vec<String> = dirs_list.iter().map(|s| (*s).to_string()).collect();
            Some(dirs_list)
        } else {
            None
        }
    }

    pub async fn check_mailbox_root(&self) -> Result<(), std::io::Error> {
        self.check_mailbox_folder("").await?;
        Ok(())
    }

    pub async fn check_mailbox_folder<P>(&self, path_part: P) -> Result<(), std::io::Error>
        where
            P: AsRef<Path>,
    {
        let path = Path::new(&self.mailbox_root);
        let path = path.join(&path_part.as_ref().to_owned());
        let metadata = metadata(&path).await;
        match metadata {
            Err(_) => {
                warn!("Mailbox folder {:?} was missing. Recreating", &path);

                create_dir_all(path).await?
            }
            Ok(_) => {}
        }
        Ok(())
    }

    pub async fn create_folder<P>(&self, path: P) -> Result<(), std::io::Error>
        where
            P: AsRef<Path>,
    {
        let path = &path.as_ref().to_owned();
        self.check_mailbox_folder(path).await?;

        Ok(())
    }
}
