Чтобы принять участие в стажировке вам нужно заполнить анкету откликнувшись по ссылке [https://forms.gle/bmZufK76KvoRrVue9](https://forms.gle/bmZufK76KvoRrVue9)

Последняя часть вопросов в анкете отведена для ответов на задачи, которые описаны ниже на этой странице. Чаще всего в поле ответа нужно указать ссылку на gist с текстом ответа, либо ссылку на репозиторий, обращайте внимание на то, что нужно оставить в анкете в качестве результата. Если вместо требуемой ссылки вы приведете что-то другое, например текст ответа, ваш ответ с высокой вероятностью не будет засчитан.

## Обязанности [](https://jl.pyshop.ru/tasks/rust-dev/#%D0%BE%D0%B1%D1%8F%D0%B7%D0%B0%D0%BD%D0%BD%D0%BE%D1%81%D1%82%D0%B8)

- Разработка серверной части приложений (backend) на Rust.

## Требования [](https://jl.pyshop.ru/tasks/rust-dev/#%D1%82%D1%80%D0%B5%D0%B1%D0%BE%D0%B2%D0%B0%D0%BD%D0%B8%D1%8F)

- Базовые знания Rust. Достаточно на уровне использования в своих pet-проектах.
- Готовность к изучению множества технологий разработки одновременно в свое личное время.
- Приветствуется владение чем-нибудь из перечисленного: SQL, HTML, CSS (также lesscss, sass), Linux CLI, Docker, git, VS Code (или другие IDE/редакторы).
- Приветствуется базовая грамотность в Computer Science, включая базы данных, сетевые технологии, технологические стеки построения веб-приложений, устройства операционных систем (в первую очередь семейства Linux). Ориентиры: [https://yollection.ru/road/backend](https://yollection.ru/road/backend), [https://yollection.ru/road/frontend](https://yollection.ru/road/frontend).

## Задание 1. Разработать функцию определения счета в игре [](https://jl.pyshop.ru/tasks/rust-dev/#%D0%B7%D0%B0%D0%B4%D0%B0%D0%BD%D0%B8%D0%B5-1.-%D1%80%D0%B0%D0%B7%D1%80%D0%B0%D0%B1%D0%BE%D1%82%D0%B0%D1%82%D1%8C-%D1%84%D1%83%D0%BD%D0%BA%D1%86%D0%B8%D1%8E-%D0%BE%D0%BF%D1%80%D0%B5%D0%B4%D0%B5%D0%BB%D0%B5%D0%BD%D0%B8%D1%8F-%D1%81%D1%87%D0%B5%D1%82%D0%B0-%D0%B2-%D0%B8%D0%B3%D1%80%D0%B5)

### Задача [](https://jl.pyshop.ru/tasks/rust-dev/#%D0%B7%D0%B0%D0%B4%D0%B0%D1%87%D0%B0)

В примере кода ниже генерируется список фиксаций состояния счета игры в течение матча.  
Разработайте функцию getScore(gameStamps, offset), которая вернет счет на момент offset в списке gameStamps.  
Нужно суметь понять суть написанного кода, заметить нюансы, разработать функцию вписывающуюся стилем в существующий код, желательно адекватной алгоритмической сложности.

```rust
use rand::Rng;const TIMESTAMPS_COUNT: usize = 50000;const PROBABILITY_SCORE_CHANGED: f64 = 0.0001;const PROBABILITY_HOME_SCORE: f64 = 0.45;const OFFSET_MAX_STEP: i32 = 3;const INITIAL_STAMP: Stamp = Stamp {    offset: 0,    score: Score { home: 0, away: 0 },};#[derive(Debug, Clone, Copy)]struct Score {    home: i32,    away: i32,}#[derive(Debug, Clone, Copy)]struct Stamp {    offset: i32,    score: Score,}fn generate_stamp(previous_value: Stamp) -> Stamp {    let score_changed: bool = rand::thread_rng().gen_bool(PROBABILITY_SCORE_CHANGED);    let home_score_change: bool = rand::thread_rng().gen_bool(PROBABILITY_HOME_SCORE);    let offset_change: i32 = rand::thread_rng().gen_range(1..=OFFSET_MAX_STEP);    Stamp {        offset: previous_value.offset + offset_change,        score: Score {            home: previous_value.score.home + if score_changed && home_score_change { 1 } else { 0 },            away: previous_value.score.away + if score_changed && !home_score_change { 1 } else { 0 },        },    }}fn generate_game() -> Vec<Stamp> {    let mut stamps = vec![INITIAL_STAMP];    let mut current_stamp = INITIAL_STAMP;    for _ in 0..TIMESTAMPS_COUNT {        current_stamp = generate_stamp(current_stamp);        stamps.push(current_stamp);    }    stamps}fn get_score(game_stamps: &[Stamp], offset: i32) -> (i32, i32) {  // Implement this function}
```

### Результат [](https://jl.pyshop.ru/tasks/rust-dev/#%D1%80%D0%B5%D0%B7%D1%83%D0%BB%D1%8C%D1%82%D0%B0%D1%82)

1. Ссылка на [gist](https://gist.github.com/) с исходным кодом функции.

## Задание 2. Разработать тесты для функции определения счета в игре [](https://jl.pyshop.ru/tasks/rust-dev/#%D0%B7%D0%B0%D0%B4%D0%B0%D0%BD%D0%B8%D0%B5-2.-%D1%80%D0%B0%D0%B7%D1%80%D0%B0%D0%B1%D0%BE%D1%82%D0%B0%D1%82%D1%8C-%D1%82%D0%B5%D1%81%D1%82%D1%8B-%D0%B4%D0%BB%D1%8F-%D1%84%D1%83%D0%BD%D0%BA%D1%86%D0%B8%D0%B8-%D0%BE%D0%BF%D1%80%D0%B5%D0%B4%D0%B5%D0%BB%D0%B5%D0%BD%D0%B8%D1%8F-%D1%81%D1%87%D0%B5%D1%82%D0%B0-%D0%B2-%D0%B8%D0%B3%D1%80%D0%B5)

### Задача [](https://jl.pyshop.ru/tasks/rust-dev/#%D0%B7%D0%B0%D0%B4%D0%B0%D1%87%D0%B0-1)

Для разработанной в предыдущем задании функции getScore(game_stamps, offset) разработайте unit-тесты [How to Write Tests](https://doc.rust-lang.org/book/ch11-01-writing-tests.html).  
Тесты должны учитывать все возможные случаи использования функции, концентрироваться на проверке одного случая, не повторяться, название тестов должно отражать суть выполняемой проверки.

### Результат [](https://jl.pyshop.ru/tasks/rust-dev/#%D1%80%D0%B5%D0%B7%D1%83%D0%BB%D1%8C%D1%82%D0%B0%D1%82-1)

1. Ссылка на [gist](https://gist.github.com/) с исходным кодом тестов.

## Задание 3. Разработать консольное приложение для подбора хеша [](https://jl.pyshop.ru/tasks/rust-dev/#%D0%B7%D0%B0%D0%B4%D0%B0%D0%BD%D0%B8%D0%B5-3.-%D1%80%D0%B0%D0%B7%D1%80%D0%B0%D0%B1%D0%BE%D1%82%D0%B0%D1%82%D1%8C-%D0%BA%D0%BE%D0%BD%D1%81%D0%BE%D0%BB%D1%8C%D0%BD%D0%BE%D0%B5-%D0%BF%D1%80%D0%B8%D0%BB%D0%BE%D0%B6%D0%B5%D0%BD%D0%B8%D0%B5-%D0%B4%D0%BB%D1%8F-%D0%BF%D0%BE%D0%B4%D0%B1%D0%BE%D1%80%D0%B0-%D1%85%D0%B5%D1%88%D0%B0)

### Задача [](https://jl.pyshop.ru/tasks/rust-dev/#%D0%B7%D0%B0%D0%B4%D0%B0%D1%87%D0%B0-2)

Требуется разработать консольное приложение, которое будет перебирать целые числа начиная с 1, для каждого из чисел рассчитывать хеш sha256, и выводить в консоль хеш и исходное число, если дайджест хеша (символьное представление хеша) оканчивается N-символами нуля. N задается пользователем при запуске приложения. Параметр F определяет сколько значений хеша следует найти команде.

Например:

```bash
$ ./hash_finder -N 3 -F 64163, "95d4362bd3cd4315d0bbe38dfa5d7fb8f0aed5f1a31d98d510907279194e3000"11848, "cb58074fd7620cd0ff471922fd9df8812f29f302904b15e389fc14570a66f000"12843, "bb90ff93a3ee9e93c123ebfcd2ca1894e8994fef147ad81f7989eccf83f64000"13467, "42254207576dd1cfb7d0e4ceb1afded40b5a46c501e738159d8ac10b36039000"20215, "1f463eb31d6fa7f3a7b37a80f9808814fc05bf10f01a3f653bf369d7603c8000"28892, "dab12874ecae90c0f05d7d87ed09921b051a586c7321850f6bb5e110bc6e2000"
```

```bash
$ ./hash_finder -N 5 -F 3828028, "d95f19b5269418c0d4479fa61b8e7696aa8df197082b431a65ff37595c100000"2513638, "862d4525b0b60779d257be2b3920b90e3dbcd60825b86cfc6cffa49a63c00000"3063274, "277430daee71c67b356dbb81bb0a39b6d53efd19d14177a173f2e05358a00000"
```

### Замечания к реализации [](https://jl.pyshop.ru/tasks/rust-dev/#%D0%B7%D0%B0%D0%BC%D0%B5%D1%87%D0%B0%D0%BD%D0%B8%D1%8F-%D0%BA-%D1%80%D0%B5%D0%B0%D0%BB%D0%B8%D0%B7%D0%B0%D1%86%D0%B8%D0%B8)

Требуется разработать консольное приложение Rust.  
Код должен сопровождаться Cargo.toml, чтобы приложение было легко собрать и запустить для проверки.  
Архитектура приложения должна быть ориентирована на максимальную утилизацию вычислительных мощностей (concurrency, parallelism).  
При оценке будут учитываться архитектура, декомпозиция, документация, тесты.

### Результат [](https://jl.pyshop.ru/tasks/rust-dev/#%D1%80%D0%B5%D0%B7%D1%83%D0%BB%D1%8C%D1%82%D0%B0%D1%82-2)

1. Ссылки на git-репозитории с исходными кодами решения.