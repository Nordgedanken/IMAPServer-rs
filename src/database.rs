#[derive(Debug, PartialEq, Eq)]
pub struct Users {
    pub id: i32,
    pub name: String,
    pub passwd: String,
    pub email: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Mailbox {
    pub id: i32,
    pub name: String,
    pub unread: i32,
    pub messeages: i32,
}
