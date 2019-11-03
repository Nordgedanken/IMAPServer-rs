use log::warn;
use std::path::Path;
use tokio::fs::{create_dir_all, metadata};

pub(crate) struct Mailbox {
    pub(crate) mailbox_root: &'static str,
}

impl Mailbox {
    pub fn new() -> Self {
        Mailbox {
            // TODO Define in config
            mailbox_root: "./mailbox_root",
        }
    }

    pub async fn check_mailbox_root(&self) -> Result<(), std::io::Error> {
        self.check_mailbox_folder(self.mailbox_root).await?;
        Ok(())
    }

    pub async fn check_mailbox_folder<P>(&self, path: P) -> Result<(), std::io::Error>
    where
        P: AsRef<Path>,
    {
        let path = &path.as_ref().to_owned();
        let metadata = metadata(path).await;
        match metadata {
            Err(_) => {
                warn!("Mailbox folder {} was missing. Recreating", path.display());

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
