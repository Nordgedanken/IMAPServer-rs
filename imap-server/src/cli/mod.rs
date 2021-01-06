use crate::database::Database;
use crate::passwords;

pub async fn add(db: Database, username: String, pass: String) -> color_eyre::Result<()> {
    let hash = passwords::hash(pass.as_str());
    db.add_user(username, hash).await?;
    tracing::info!("Successfully added User!");
    Ok(())
}

pub async fn remove(db: Database, username: String) -> color_eyre::Result<()> {
    db.remove_user(username).await?;
    tracing::info!("Successfully removed User!");
    Ok(())
}

pub async fn change_password(
    db: Database,
    username: String,
    pass: String,
) -> color_eyre::Result<()> {
    db.add_user(username, pass).await?;
    tracing::info!("Successfully changed user's password!");
    Ok(())
}
