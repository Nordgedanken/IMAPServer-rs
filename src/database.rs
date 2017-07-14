#[derive(Debug, PartialEq, Eq)]
struct User {
    name: String,
    passwd: String
}

#[derive(Debug, PartialEq, Eq)]
struct Mailbox {
    id: i32,
    name: String,
    unread: i32,
    messeages: i32,
}
