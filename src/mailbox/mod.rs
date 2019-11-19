use std::path::Path;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use futures::StreamExt;
use log::{debug, warn};
use rand::{thread_rng, Rng};
use tokio::fs::{create_dir_all, metadata, read_dir};

use crate::database::establish_connection;
use crate::models::{NewUser, User};
use crate::schema::users;

pub(crate) struct Mailbox {
    pub(crate) mailbox_root: String,
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

impl Mailbox {
    pub fn new(email: String, password: String) -> Option<Self> {
        // TODO define secure key in config
        let key = b"secure_key";

        let password_hash = easy_password::bcrypt::hash_password(&password, key, 12)
            .expect("unable to hash password");

        let mut rng = thread_rng();

        if rng.gen() {
            let random_number: i32 = rng.gen_range(0, 2 ^ 32 - 1);

            let new_user = NewUser {
                email: &email,
                password_hash: &password_hash,
                uid_validity_identifier: random_number,
            };

            let connection = establish_connection();
            diesel::insert_into(users::table)
                .values(&new_user)
                .execute(&connection)
                .expect("Failed to add new User");

            // TODO Define in config
            let mailbox_root = format!("./mailbox_root/{}", email);

            return Some(Mailbox { mailbox_root });
        } else {
            return None;
        };
    }

    pub fn load(user: String) -> Option<Self> {
        use crate::schema::users::dsl::*;
        let connection = establish_connection();

        let results: Result<User, diesel::result::Error> = users
            .filter(email.eq(user))
            .limit(1)
            .get_result::<User>(&connection);

        match results {
            Ok(results) => {
                let mailbox_root = format!("./mailbox_root/{}", results.email);
                Some(Mailbox { mailbox_root })
            }
            _ => None,
        }
    }

    pub fn load_all() -> Option<Vec<Self>> {
        use crate::schema::users::dsl::*;
        let connection = establish_connection();

        let results: Vec<User> = users
            .load::<User>(&connection)
            .expect("Error getting Mailboxes");

        let mut returns: Vec<Self> = Vec::new();

        for entry in &results {
            let mailbox_root = format!("./mailbox_root/{}", entry.email);
            returns.push(Mailbox { mailbox_root })
        }

        Some(returns)
    }

    // TODO get filtered if wanted by the client
    pub async fn get_lsub(&self) -> Option<Vec<String>> {
        let mut dirs = read_dir(self.mailbox_root.to_owned())
            .await
            .expect("unable to read dir");

        let mut dirs_lsub: Vec<&str> = Vec::new();

        while let Some(dir) = dirs.next().await {
            let dir = dir.expect("unable to get dir");
            if dir.file_name() == "INBOX" {
                dirs_lsub.push("* LSUB (\\HasNoChildren) \".\" INBOX\r\n");
                continue;
            } else if dir.file_name() == "Trash" {
                dirs_lsub.push("* LSUB  (\\Subscribed \\Noinferiors) \".\" \"Trash\"\r\n");
                continue;
            } else {
                let path_string =
                    format!("* LSUB  (\\Subscribed) \".\" {:?}\r\n", dir.file_name());
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

    // TODO get filtered if wanted by the client
    pub async fn get_list(&self) -> Option<Vec<String>> {
        let mut dirs = read_dir(self.mailbox_root.to_owned())
            .await
            .expect("unable to read dir");

        let mut dirs_list: Vec<&str> = Vec::new();

        while let Some(dir) = dirs.next().await {
            let dir = dir.expect("unable to get dir");
            if dir.file_name() == "INBOX" {
                dirs_list
                    .push("* LIST (\\Marked \\HasNoChildren \\Subscribed) \".\" \"INBOX\"\r\n");
                continue;
            } else if dir.file_name() == "Trash" {
                dirs_list.push("* LIST  (\\Subscribed \\Noinferiors) \".\" \"Trash\"\r\n");
                continue;
            } else {
                // TODO actually check if subscribed or not.
                let path_string =
                    format!("* LIST (\\Subscribed) \".\" {:?}\r\n", dir.file_name());
                debug!("DEBUG PATH: {}", path_string);
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
