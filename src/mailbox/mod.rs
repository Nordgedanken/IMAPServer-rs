use log::warn;
use std::path::Path;
use tokio::fs::{create_dir_all, metadata};

pub async fn check_mailbox_root() -> Result<(), std::io::Error> {
    // TODO Define in config
    check_mailbox_folder("./mailbox_root").await?;
    Ok(())
}

pub async fn check_mailbox_folder<P>(path: P) -> Result<(), std::io::Error>
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
