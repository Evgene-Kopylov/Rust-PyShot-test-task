use colored::Colorize;
use clap::Parser;
use sha2::{Sha256, Digest};
use hex::encode;
use std::sync::{Arc, Mutex};
use num_cpus;
use std::thread;

const ITERATIONS_LIMIT: u64 = 5_000_000;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// число нулей в конце хеша
    #[arg(short, long)]
    zeros: usize,

    /// число значений хеша
    #[arg(short, long, default_value_t = 1)]
    lines: usize,
}

/// Возвращает строку хеша для числа.
fn calculate_sha256_hash(value: u64) -> String {
    let mut hasher = Sha256::new();
    hasher.update(value.to_be_bytes());
    encode(hasher.finalize())
}

/// Проверяет точное число нулей на конце строки.
fn ends_with_zeros(hash_string: &str, n: usize) -> bool {
    hash_string.ends_with(&"0".repeat(n))
}

fn main() { 
    let args = Args::parse();
    println!("zeros = {}, lines = {}", args.zeros, args.lines);

    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    // Находим количество доступных ядер CPU
    let num_threads = if num_cpus::get() > args.lines {
        args.lines
    } else {
        num_cpus::get()
    };

    // Сколько значений должна получить каждый поток.
    let num_values_per_thread = args.lines / num_threads + 1;

    for i in 0..num_threads {        
        let results = Arc::clone(&results);
        let handle = thread::spawn(move || {
            let mut count = 0;
            let mut val = results.lock().unwrap();            
            for j in (0..ITERATIONS_LIMIT).step_by(num_threads) {
                let hash_string = calculate_sha256_hash(j + i as u64);
                if ends_with_zeros(&hash_string, args.zeros) {
                    let line = format!("{} '{}'", j.to_string().yellow(), hash_string.green());
                    val.push((j, line));
                    count += 1;
                }
                if count >= num_values_per_thread {
                    break;
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mut result = results.lock().unwrap();
    result.sort_by_key(|item| item.0);

    for (i, (_, line)) in result.iter().enumerate().take(args.lines) {
        println!("{}) {}", i + 1, line);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_hash() {
        let value: u64 = 12345;
        let hash_string = calculate_sha256_hash(value);
        assert_eq!(hash_string.len(), 64); // Длина хеша должна быть 64 символа
    }

    #[test]
    fn test_ends_with_zeros() {
        let hash_string = "12345abcde000000";
        assert!(ends_with_zeros(hash_string, 6)); // Хеш должен оканчиваться на 6 нулей
        assert!(!ends_with_zeros(hash_string, 7)); // Хеш не должен оканчиваться на 7 нулей
    }
}