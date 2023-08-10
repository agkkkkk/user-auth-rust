-- Your SQL goes here

CREATE TABLE Users (
    firstname VARCHAR(20) NOT NULL,
    lastname VARCHAR(20),
    dateofbirth VARCHAR(10),
    email VARCHAR(50) PRIMARY KEY,
    password TEXT NOT NULL 
);