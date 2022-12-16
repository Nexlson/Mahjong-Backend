#!/bin/sh

# install diesel cli
cargo install diesel_cli --no-default-features --features sqlite

# set .env file
echo DATABASE_URL=mahjong_game.sqlite > .env

# run diesel setup (create new database)
diesel setup 

# run all migration
diesel migration run 
