#[allow(unused_imports)]  // FIXME

use colored::Colorize;

use clap::Parser;
use std::sync::{Arc, Mutex};
use num_cpus;
use sha2::{Sha256, Digest};
use hex::encode;
use std::thread;

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

fn calculate_sha256_hash(value: u64) -> String {
    let mut hasher = Sha256::new();
    hasher.update(value.to_be_bytes());
    encode(hasher.finalize())
}

fn ends_with_zeros(hash_string: &str, n: usize) -> bool {
    hash_string.ends_with(&"0".repeat(n))
}

fn main() { 
    let args = Args::parse();
    println!("zeros = {}, lines = {}", args.zeros, args.lines);

    let t = vec![];
    let results = Arc::new(Mutex::new(t));
    let mut handles = vec![];

    // Находим количество доступных ядер CPU
    let num_threads = num_cpus::get();
    
    for thread in 0..num_threads {
        let results = Arc::clone(&results);
        let handle = thread::spawn(move || {
            let line = "737363636700000";
            let mut val = results.lock().unwrap();
            val.push(line);
        });
        handles.push(handle);

    }

    for handle in handles {
        handle.join().unwrap();
    }

    let res = results.lock().unwrap();
    for line in res.iter() {
        println!("{}", line);
    }

    // // Находим количество доступных ядер CPU
    // let cpu_threads = num_cpus::get();
    // println!("cpu_threads = {}", cpu_threads);

    // for value in 0..1_000_000 {
    //     let hash_string = calculate_sha256_hash(value);
    //     if ends_with_zeros(&hash_string, args.zeros) {
    //         println!("{}, \"{}\"", value.to_string().yellow(), hash_string.green());
    //         break;
    //     }

    // }


    // let num_values_per_thread = args.zeros / num_threads;

    // // Создаем вектор для хранения результатов
    // let results = Arc::new(Mutex::new(Vec::new()));

    // // Создаем вектор потоков
    // let mut handles = vec![];

    // for _ in 0..num_threads {
    //     let results = Arc::clone(&results);

    //     let handle = std::thread::spawn(move || {
    //         let mut count = 0;
    //         let mut value: u64 = 1; // Используем u64 для избегания переполнения

    //         while count < num_values_per_thread {
    //             // Рассчитываем хеш SHA-256 для числа
    //             let hash_hex = calculate_sha256_hash(value);

    //             // Проверяем, оканчивается ли хеш на нули
    //             if ends_with_zeros(&hash_hex, args.zeros) {
    //                 let result_string = format!("{}, \"{}\"", value, hash_hex);
    //                 let mut results = results.lock().unwrap();
    //                 results.push(result_string);
    //                 dbg!(results);
    //             }

    //             value += 1;
    //             count += 1;
    //         }
    //     });

    //     handles.push(handle);
    // }

    // // Дожидаемся завершения всех потоков
    // for handle in handles {
    //     handle.join().unwrap();
    //     println!("op");
    // }

    // // Выводим результаты
    // let results = results.lock().unwrap();
    // println!("fin");
    // dbg!(results);
    // // for result in results.iter() {
    // //     println!("i");
    // //     println!("{}", result);
    // // }

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
        assert!(ends_with_zeros(hash_string, 5)); // Хеш должен оканчиваться на 5 нулей
        assert!(!ends_with_zeros(hash_string, 7)); // Хеш не должен оканчиваться на 6 нулей
    }
}