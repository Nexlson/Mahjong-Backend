-- Your SQL goes here

CREATE TABLE game (
    id varchar PRIMARY KEY,
    duration time,
    players text[] NOT NULL,
    joined integer NOT NULL,
    number_of_player integer NOT NULL
)