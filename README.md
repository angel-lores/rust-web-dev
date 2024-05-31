# Rust Web Example
Angel Lores 2024

This is a homework repository for CS-410P Rust Web Development taken at PSU in Spring 2024.

## Requirements
- Rust
- PostgreSQL

## Database Setup
After installing PostgreSQL run the following commands to create the Database & Table:
```console
psql postgres
```
```SQL
CREATE DATABASE qa_db;
```
```SQL
quit
```
```console
psql qa_db
```
```SQL
CREATE TABLE question (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    content VARCHAR(255) NOT NULL,
    tags TEXT[] DEFAULT NULL
);
```
```SQL
CREATE TABLE answer (
    id SERIAL PRIMARY KEY,
    content VARCHAR(255) NOT NULL,
    q_id INTEGER REFERENCES question(id) NOT NULL
);
```
```SQL
quit
```

## Run & Test (http://127.0.0.1:3000/q/)
```console
cargo run
```
POST a question
```
curl -X POST http://localhost:3000/q/ -H "Content-Type: application/json" -d '{"title": "US President", "content": "Who was the first president of the USA?", "tags": ["USA", "trivia"]}'
```
POST an answer to a question (q_id must match an existing question id)
```
curl -X POST http://localhost:3000/a/ -H "Content-Type: application/json" -d '{"content": "George Washington", "q_id": 1}'
```
GET all questions
```
curl http://localhost:3000/q/
```
GET a question and all associated answers by question ID
```
curl http://localhost:3000/qa/1
```
PUT a question by ID
```
curl -X PUT http://localhost:3000/qa/1 -H "Content-Type: application/json" -d '{"title": "USA", "content": "Who was the third president?", "tags": ["trivia", "usa"]}'
```
DELETE a question and all associated answers by ID
```
curl -X DELETE http://localhost:3000/qa/1
```