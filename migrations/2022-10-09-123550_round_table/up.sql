-- Your SQL goes here

CREATE TABLE round (
    id varchar PRIMARY KEY,
    gameid varchar,
    winning_player text[],
    losing_player text[]
)