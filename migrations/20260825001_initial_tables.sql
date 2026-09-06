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

CREATE TABLE IF NOT EXISTS "inventories" (
    "id" TEXT PRIMARY KEY NOT NULL,
    "name" TEXT NOT NULL,
    "uid" TEXT NOT NULL,
    "id_type" TEXT NOT NULL,
    "created_at" DATETIME DEFAULT CURRENT_TIMESTAMP,
    "updated_at" DATETIME DEFAULT CURRENT_TIMESTAMP,
    "quantity" INTEGER DEFAULT 0
);

CREATE INDEX IF NOT EXISTS "idx_inventories_uid" ON "inventories" ("uid");
