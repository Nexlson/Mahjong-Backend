-- Your SQL goes here

CREATE TABLE player (
    id varchar PRIMARY KEY,
    name varchar NOT NULL,
    gameId varchar,
    score integer NOT NULL,
    FOREIGN KEY (gameId) REFERENCES game(id)
)