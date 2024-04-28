# Rust Web Example
Angel Lores 2024

This is a homework repository for CS-410P Rust Web Development taken at PSU in Spring 2024.

## Currently implementing CRUD operations on Questions Server in rest_up, C & R are working
Questions Server heavily based on Bart Massey's knock-knock Repo, and Rust Web Development Book Repo:
https://github.com/pdx-cs-rust-web/knock-knock
https://github.com/Rust-Web-Development/code/tree/main/ch_04/final

###Run & Test:
1. cargo run
2. http://127.0.0.1/3000/
3. Use following curl commands to test (only get and post work for now):
    - curl http://localhost:3000/
    - curl -X POST -H "Content-Type: application/json" -d '{"id": "4", "title": "What", "content": "What?", "tags": ["fyi"]}' http://localhost:3000/
    - curl -X PUT -H "Content-Type: application/json" -d '{"id": "1", "title": "Updated Question Title", "content": "Updated question content", "tags": ["swap"]}' http://localhost:3000/4
    - curl -X DELETE http://localhost:3000/1