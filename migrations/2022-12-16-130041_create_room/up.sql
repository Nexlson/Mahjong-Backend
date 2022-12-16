-- Your SQL goes here
CREATE TABLE room (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    player_one INTEGER,
    player_two INTEGER,
    player_three INTEGER,
    player_four INTEGER,
    foreign key (player_one) REFERENCES player(id),
    foreign key (player_two) REFERENCES player(id),
    foreign key (player_three) REFERENCES player(id),
    foreign key (player_four) REFERENCES player(id)
)