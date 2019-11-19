use super::schema::users;

#[derive(Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password_hash: String,
    pub uid_validity_identifier: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub password_hash: &'a str,
    pub uid_validity_identifier: i32,
}
