-- Your SQL goes here
CREATE TABLE users(
    id SERIAL PRIMARY KEY,
    username VARCHAR(200) NOT NULL,
    password VARCHAR(200) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)