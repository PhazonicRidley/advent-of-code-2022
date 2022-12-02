use std::env;
mod day_1;

fn main() {
    println!("{}", env::current_dir().unwrap().display());
    day_1::main();
}
