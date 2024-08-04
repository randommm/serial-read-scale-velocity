-- Add migration script here
CREATE TABLE "sessions" (
    "id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,
    "created_at" integer NOT NULL
);
CREATE TABLE "readings" (
    "id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,
    "session_id" integer NOT NULL,
    "value" float NOT NULL,
    "read_at" integer NOT NULL
);
PRAGMA journal_mode=WAL;
