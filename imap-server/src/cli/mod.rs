use crate::database::Database;
use crate::passwords;

pub async fn add(db: Database, username: String, pass: String) -> color_eyre::Result<()> {
    let hash = passwords::hash(pass.as_str());
    Ok(())
}
