#[derive(Debug, PartialEq, Eq)]
struct User {
    name: String,
    passwd: String,
    mailbox: i32,
};

#[derive(Debug, PartialEq, Eq)]
struct Mailbox {
    id: i32,
    name: String,
    Unread: i32,
    messeages: i32,
}