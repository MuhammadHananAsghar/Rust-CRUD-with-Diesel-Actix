-- Your SQL goes here
CREATE TABLE "users" (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    address TEXT NOT NULL,
    created_at TEXT NOT NULL
);

INSERT INTO "users" (name, address, created_at) VALUES ('John Doe', '123 Main St', '2024-05-12 10:00:37');
INSERT INTO "users" (name, address, created_at) VALUES ('Jane Doe', '456 Elm St', '2024-05-12 10:00:37');
INSERT INTO "users" (name, address, created_at) VALUES ('Alice Smith', '789 Oak St', '2024-05-12 10:00:37');
INSERT INTO "users" (name, address, created_at) VALUES ('Bob Smith', '1011 Pine St', '2024-05-12 10:00:37');
INSERT INTO "users" (name, address, created_at) VALUES ('Charlie Brown', '1213 Birch St', '2024-05-12 10:00:37');
