use std::path::Path;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use log::warn;
use rand::{thread_rng, Rng};
use tokio::fs::{create_dir_all, metadata};

use crate::database::establish_connection;
use crate::models::{NewUser, User};
use crate::schema::users;

pub(crate) struct Mailbox {
    pub(crate) mailbox_root: String,
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
                Some(Mailbox {
                    mailbox_root: mailbox_root,
                })
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
