#[allow(unused_imports)]  // FIXME

use clap::Parser;
use std::sync::{Arc, Mutex};
use num_cpus;
use sha2::{Sha256, Digest};
use hex::encode;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// число нулей в конце хеша
    #[arg(short, long)]
    zeros: u8,

    /// число значений хеша
    #[arg(short, long, default_value_t = 1)]
    lines: u8,
}

fn calculate_sha256_hash(value: u64) -> String {
    let mut hasher = Sha256::new();
    hasher.update(&value.to_be_bytes());
    encode(&hasher.finalize())
}

fn main() {
    let args = Args::parse();
    println!("zeros = {}, lines = {}", args.zeros, args.lines);

    // Находим количество доступных ядер CPU
    let cpu_threads = num_cpus::get();
    println!("cpu_threads = {}", cpu_threads);

    // получить хеш числа
    let value = 4;
    let hash_hex = calculate_sha256_hash(value);
    use colored::Colorize;
    println!("{}, \"{}\"", value.to_string().yellow(), hash_hex.green());
 
    

}