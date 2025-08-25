use std::{
  env,
  io::{self, Write},
};

fn main() {
    let args: Vec<String> = env::args().collect();

    for arg in args.iter() {
      println!("Received arg: {}", arg);
    }

    println!("Please enter an argument:");

    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let input = input.trim();
    println!("Received stdin input: {}", input);
}
