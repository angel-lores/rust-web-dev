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
quit
```

## Run & Test
```console
cargo run
```
Use URL http://127.0.0.1/3000/ to view the questions while you run the following commands on a terminal separate from where you used cargo run:
POST
```
curl -X POST http://localhost:3000/ -H "Content-Type: application/json" -d '{"title": "US President", "content": "Who was the first president of the USA?", "tags": ["USA", "trivia"]}'
```
GET All
```
curl http://localhost:3000/
```
GET By ID
```
curl http://localhost:3000/1
```
PUT By ID
```
curl -X PUT http://localhost:3000/1 -H "Content-Type: application/json" -d '{"title": "USA", "content": "Who was the third president?", "tags": ["trivia", "usa"]}'
```
DELETE By ID
```
curl -X DELETE http://localhost:3000/1
```