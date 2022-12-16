-- Your SQL goes here
CREATE TABLE round (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    winning_player INTEGER,
    losing_player INTEGER,
    room_id INTEGER,
    foreign key (winning_player) REFERENCES player(id),
    foreign key (losing_player) REFERENCES player(id),
    foreign key (room_id) REFERENCES room(id)
)