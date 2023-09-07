# PyShot test task
 Тестовые задания на практику по Rust в PyShop.

### cargo watch + tests
Отслеживать изменения и при сохранении запускать тесты.
```console
cargo install cargo-watch
cd hash_finder
cargo watch -x "run -- -z 2 -l 3"
```