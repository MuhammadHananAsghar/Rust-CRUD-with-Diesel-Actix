-- Your SQL goes here
CREATE TABLE user_profile_image (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    image_url TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

INSERT INTO user_profile_image (user_id, image_url, created_at) VALUES (3, 'https://random.imagecdn.app/500/500', '2024-05-12 14:57:31');
INSERT INTO user_profile_image (user_id, image_url, created_at) VALUES (4, 'https://random.imagecdn.app/500/500', '2024-05-12 14:57:31');
INSERT INTO user_profile_image (user_id, image_url, created_at) VALUES (5, 'https://random.imagecdn.app/500/500', '2024-05-12 14:57:31');

-- diesel migration generate migration_name # generate migration
-- diesel migration run # run
-- diesel migration redo # rollback and run