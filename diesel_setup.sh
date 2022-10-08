#!/bin/sh

# create database and empty migration directory
diesel setup 

# create specific migration folder, up and down sql (up is for creation, down is for revert)
diesel migration generate create_room
diesel migration generate create_round

# run migration 
diesel migration run

# revert changes
diesel migration revert


