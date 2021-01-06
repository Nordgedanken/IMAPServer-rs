#[derive(Clone)]
pub struct Database {
    pub users: sled::Tree,
}

impl Database {
    pub async fn open() -> color_eyre::Result<Self> {
        let db: sled::Db = sled::open("db")?;

        let users_tree: sled::Tree = db.open_tree(b"users")?;
        Ok(Database { users: users_tree })
    }

    pub async fn add_user(&self, username: String, password: String) -> color_eyre::Result<()> {
        self.users
            .insert(username.as_bytes(), password.as_bytes())?;
        Ok(())
    }

    pub async fn remove_user(&self, username: String) -> color_eyre::Result<()> {
        self.users.remove(username)?;
        Ok(())
    }

    pub async fn get_user_hash(&self, username: &str) -> color_eyre::Result<[u8; 128]> {
        let result = self.users.get(username)?;
        match result {
            Some(hash_ivec) => {
                let hash: &[u8] = &hash_ivec;
                let mut padded = [0u8; 128];
                hash.iter().enumerate().for_each(|(i, val)| {
                    padded[i] = val.clone();
                });
                Ok(padded)
            }
            None => Err(color_eyre::eyre::eyre!("Hash missing!")),
        }
    }
}
