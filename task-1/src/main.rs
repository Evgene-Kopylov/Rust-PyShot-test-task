use rand::Rng;


fn main() {
    let game_stamps = generate_game();
    let offset = 1_000_000; // Замените на желаемый момент времени

    let (home_score, away_score) = get_score(&game_stamps, offset);

    println!("Счет на момент времени {}: Дома {} - В гостях {}", offset, home_score, away_score);
}


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
    let score_changed: bool = rand::thread_rng().gen_bool(PROBABILITY_SCORE_CHANGED);
    let home_score_change: bool = rand::thread_rng().gen_bool(PROBABILITY_HOME_SCORE);
    let offset_change: i32 = rand::thread_rng().gen_range(1..=OFFSET_MAX_STEP);

    Stamp {
        offset: previous_value.offset + offset_change,
        score: Score {
            home: previous_value.score.home + if score_changed && home_score_change { 1 } else { 0 },
            away: previous_value.score.away + if score_changed && !home_score_change { 1 } else { 0 },
        },
    }
}

fn generate_game() -> Vec<Stamp> {
    let mut stamps = vec![INITIAL_STAMP];
    let mut current_stamp = INITIAL_STAMP;

    for _ in 0..TIMESTAMPS_COUNT {
        current_stamp = generate_stamp(current_stamp);
        stamps.push(current_stamp);
    }

    stamps
}


fn get_score(game_stamps: &[Stamp], offset: i32) -> (i32, i32) {
    let mut home_score = 0;
    let mut away_score = 0;

    for stamp in game_stamps {
        if stamp.offset <= offset {
            home_score = stamp.score.home;
            away_score = stamp.score.away;
        } else {
            break;
        }
    }

    (home_score, away_score)
}
