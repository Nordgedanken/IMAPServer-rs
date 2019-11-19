CREATE TABLE users (
  id INTEGER NOT NULL PRIMARY KEY,
  email TEXT NOT NULL,
  password_hash TEXT NOT NULL,
  uid_validity_identifier INTEGER NOT NULL
)
