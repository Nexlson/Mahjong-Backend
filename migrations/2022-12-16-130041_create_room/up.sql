-- Your SQL goes here
CREATE TABLE room (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    player_one INTEGER NOT NULL,
    player_two INTEGER,
    player_three INTEGER,
    player_four INTEGER,
    players INTEGER NOT NULL DEFAULT 0,
    foreign key (player_one) REFERENCES player(id),
    foreign key (player_two) REFERENCES player(id),
    foreign key (player_three) REFERENCES player(id),
    foreign key (player_four) REFERENCES player(id)
)