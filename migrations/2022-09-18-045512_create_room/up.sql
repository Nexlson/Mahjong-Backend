-- Your SQL goes here

CREATE TABLE room (
    id SERIAL PRIMARY KEY,
    number_of_participants INTEGER NOT NULL,
    player_1 VARCHAR,
    player_2 VARCHAR,
    player_3 VARCHAR,
    player_4 VARCHAR
)