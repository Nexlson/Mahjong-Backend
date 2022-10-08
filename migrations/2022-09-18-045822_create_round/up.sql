-- Your SQL goes here

CREATE TABLE round (
    id SERIAL PRIMARY KEY,
    room_id int,
    player_1 int DEFAULT 0,
    player_2 int DEFAULT 0,
    player_3 int DEFAULT 0,
    player_4 int DEFAULT 0,
    FOREIGN KEY (room_id)
    REFERENCES room(id)
)