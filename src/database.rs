#[derive(Debug, PartialEq, Eq)]
pub struct User {
    pub name: String,
    pub passwd: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Mailbox {
    pub id: i32,
    pub name: String,
    pub unread: i32,
    pub messeages: i32,
}
