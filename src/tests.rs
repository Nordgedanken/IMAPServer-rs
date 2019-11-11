use diesel;
use diesel::result::Error;
use diesel::{Connection, RunQueryDsl, SqliteConnection};

use crate::models::NewUser;
use crate::schema::users;

pub fn with_db<F>(f: F) -> ()
where
    F: Fn(&SqliteConnection) -> (),
{
    let conn = crate::database::establish_connection();

    conn.test_transaction::<_, Error, _>(|| {
        f(&conn);
        Ok(())
    });
}

#[test]
fn create_mailbox() {
    with_db(|s| {
        let new_user = NewUser {
            email: "test@localhost",
            password_hash: "test",
            uid_validity_identifier: "0000",
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(s)
            .expect("Failed to add new User");
    })
}
