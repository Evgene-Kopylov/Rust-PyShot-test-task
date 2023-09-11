### Задание 3. Поиск хешей.
Пример консольной команды для запуска
```console
cargo run -- -z 3 -l 5
```
`-z` - число нулей в конце строки

`-l` - число строк

`--help` - меню приложения

![help](Screenshot%202023-09-11%20114529.png)

Пример вывода:
![Скриншот](./Screenshot%202023-09-08%20203045.png)

### cargo watch + tests
Отслеживать изменения и при сохранении запускать тесты.
```console
cargo install cargo-watch
cd hash_finder
cargo watch -x "run -- -z 2 -l 3"
```