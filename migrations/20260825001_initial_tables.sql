-- SQLite3 flavor SQL syntax
CREATE TABLE IF NOT EXISTS "users" (
    "id" TEXT PRIMARY KEY NOT NULL,
    "username" TEXT NOT NULL,
    "email" TEXT NOT NULL,
    "password_hash" TEXT NOT NULL,
    "token_seed" TEXT NOT NULL,
    "ref_token_seed" TEXT NOT NULL,
    "created_at" DATETIME DEFAULT CURRENT_TIMESTAMP,
    "updated_at" DATETIME DEFAULT CURRENT_TIMESTAMP,
    "should_reset_password" INTEGER DEFAULT 0
);
