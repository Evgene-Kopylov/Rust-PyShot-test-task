use clap::Parser;

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

fn main() {
    let args = Args::parse();
    println!("zeros = {}, lines = {}", args.zeros, args.lines);

}