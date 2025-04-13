-- Cleo by Alyx Shang.
-- Licensed under the FSL v1.

CREATE TABLE instance_info(
    instance_id TEXT NOT NULL PRIMARY KEY,
    hostname TEXT NOT NULL,
    instance_name TEXT NOT NULL,
    smtp_server TEXT NOT NULL,
    smtp_username TEXT NOT NULL,
    smtp_pass TEXT NOT NULL,
    file_dir TEXT NOT NULL
);

CREATE TABLE cleo_users(
    user_id TEXT NOT NULL PRIMARY KEY,
    display_name TEXT NOT NULL,
    is_verified BOOLEAN NOT NULL,
    username TEXT NOT NULL,
    pwd TEXT NOT NULL,
    email_addr TEXT NOT NULL,
    pfp_url TEXT NOT NULL,
    is_admin BOOLEAN NOT NULL
);

CREATE TABLE user_files(
    file_id TEXT NOT NULL PRIMARY KEY,
    user_id TEXT NOT NULL,
    file_path TEXT NOT NULL,
    file_url TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES cleo_users(user_id) ON DELETE CASCADE
);

CREATE TABLE user_posts(
    content_id TEXT NOT NULL PRIMARY KEY,
    user_id TEXT NOT NULL,
    content_type TEXT NOT NULL,
    content_text TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES cleo_users(user_id) ON DELETE CASCADE
);

CREATE TABLE extra_content_fields(
    field_id TEXT NOT NULL PRIMARY KEY,
    content_id TEXT NOT NULL,
    field_key TEXT NOT NULL,
    field_value TEXT NOT NULL,
    FOREIGN KEY (content_id) REFERENCES user_posts(content_id) ON DELETE CASCADE
);

CREATE TABLE user_api_tokens(
    token_id TEXT NOT NULL PRIMARY KEY,
    user_id TEXT NOT NULL,
    token TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES cleo_users(user_id) ON DELETE CASCADE
);

CREATE TABLE user_keys(
    key_id TEXT NOT NULL PRIMARY KEY,
    user_id TEXT NOT NULL,
    user_key TEXT NOT NULL,
    key_type TEXT NOT NULL,
    key_used BOOLEAN NOT NULL,
    username TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES cleo_users(user_id) ON DELETE CASCADE
);

CREATE TABLE email_tokens(
  etoken_id TEXT NOT NULL PRIMARY KEY,
  email_token TEXT NOT NULL,
  user_id TEXT NOT NULL,
  FOREIGN KEY (user_id) REFERENCES cleo_users(user_id) ON DELETE CASCADE
);
