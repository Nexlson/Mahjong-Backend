-- Your SQL goes here
CREATE TABLE player (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nickname VARCHAR NOT NULL,
    host INTEGER NOT NULL,
    score INTEGER
)