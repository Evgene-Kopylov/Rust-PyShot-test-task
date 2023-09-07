use rand::Rng;

const TIMESTAMPS_COUNT: usize = 50000;
const PROBABILITY_SCORE_CHANGED: f64 = 0.0001;
const PROBABILITY_HOME_SCORE: f64 = 0.45;
const OFFSET_MAX_STEP: i32 = 3;

const INITIAL_STAMP: Stamp = Stamp {
    offset: 0,
    score: Score { home: 0, away: 0 },
};

#[derive(Debug, Clone, Copy)]
struct Score {
    home: i32,
    away: i32,
}

#[derive(Debug, Clone, Copy)]
struct Stamp {
    offset: i32,
    score: Score,
}

fn generate_stamp(previous_value: Stamp) -> Stamp {
    // Генерация новой фиксации состояния счета
    let score_changed: bool = rand::thread_rng().gen_bool(PROBABILITY_SCORE_CHANGED);
    let home_score_change: bool = rand::thread_rng().gen_bool(PROBABILITY_HOME_SCORE);
    let offset_change: i32 = rand::thread_rng().gen_range(1..=OFFSET_MAX_STEP);

    // Обновление счета на основе случайных изменений
    Stamp {
        offset: previous_value.offset + offset_change,
        score: Score {
            home: previous_value.score.home + if score_changed && home_score_change { 1 } else { 0 },
            away: previous_value.score.away + if score_changed && !home_score_change { 1 } else { 0 },
        },
    }
}

fn generate_game() -> Vec<Stamp> {
    // Генерация игры с фиксациями состояния счета
    let mut stamps = vec![INITIAL_STAMP];
    let mut current_stamp = INITIAL_STAMP;

    for _ in 0..TIMESTAMPS_COUNT {
        current_stamp = generate_stamp(current_stamp);
        stamps.push(current_stamp);
    }

    stamps
}

fn get_score(game_stamps: &[Stamp], offset: i32) -> (i32, i32) {
    // Вычисление счета на момент времени offset
    let mut home_score = 0;
    let mut away_score = 0;

    for stamp in game_stamps {
        if stamp.offset <= offset {
            home_score = stamp.score.home;
            away_score = stamp.score.away;
        } else {
            break; // Достигнут момент времени offset, выходим из цикла
        }
    }

    (home_score, away_score)
}

/// Игра происходит между двумя командами, "Хозяева" и "Гости".
/// Имеет TIMESTAMP_COUNT моментов времени, в каждый из которых 
/// счет может измениться с вероятностью PROBABILITY_SCORE_CHANGED.
/// PROBABILITY_HOME_SCORE - вероятность успеха "Хозяев",
/// иначе очко достается "Гостям", таким образом, только одна команда 
/// может получить очко в один момент игры.
fn main() {
    

    let game_stamps = generate_game();
    let offset = 40000; // Замените на желаемый момент времени

    // Получение и вывод счета на указанный момент времени
    let (home_score, away_score) = get_score(&game_stamps, offset);

    println!("Счет на момент времени {}: 'Хозяева' {} - 'Гости' {}", offset, home_score, away_score);
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_score_initial_offset() {
        // Проверка начального счета на момент времени 0
        let game_stamps = generate_game();
        let (home_score, away_score) = get_score(&game_stamps, 0);
        assert_eq!(home_score, 0);
        assert_eq!(away_score, 0);
    }

    #[test]
    fn test_get_score_exact_offset() {
        // Проверка счета на момент времени равном offset
        let game_stamps = generate_game();
        let offset = 1000;
        let (home_score, away_score) = get_score(&game_stamps, offset);
        assert!(home_score >= 0);
        assert!(away_score >= 0);
    }

    #[test]
    fn test_get_score_offset_greater_than_max() {
        // Проверка счета на момент времени большем, чем максимальный момент в игре
        let game_stamps = generate_game();
        let offset = TIMESTAMPS_COUNT as i32 + 100; // Больше, чем максимальный момент
        let (home_score, away_score) = get_score(&game_stamps, offset);
        assert!(home_score >= 0);
        assert!(away_score >= 0);
    }

    #[test]
    fn test_get_score_offset_less_than_min() {
        // Проверка счета на момент времени меньшем, чем минимальный момент в игре
        let game_stamps = generate_game();
        let offset = -100; // Меньше, чем минимальный момент
        let (home_score, away_score) = get_score(&game_stamps, offset);
        assert_eq!(home_score, 0);
        assert_eq!(away_score, 0);
    }
}