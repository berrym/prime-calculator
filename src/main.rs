use clap::Parser;
use std::{
    error,
    io::{self, Write},
};

use crate::primes::calculate;

mod primes;

#[derive(Parser)]
#[command(arg_required_else_help(true))]
#[command(name="primes", version, long_about = "Calculate prime numbers up to limit or n'th number.")]
#[command(next_line_help = true)]
struct Cli {
    /// Calculate prime numbers up to limit
    #[arg(short, long, value_name = "NUMBER")]
    limit: Option<usize>,

    /// Calculate primes to the n'th prime
    #[arg(short, long, value_name = "NUMBER")]
    nth: Option<usize>,

    /// Interactive prime generator repl
    #[arg(short, long)]
    repl: bool,

    /// Show progress when calculating primes
    #[arg(short, long)]
    progress: bool,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    // Parse command line
    let cli = Cli::parse();

    // Calculate and print prime numbers up to N
    if let Some(limit) = cli.limit.as_ref() {
        println!(
            "List of prime numbers up to {}\n{:#?}",
            *limit,
            calculate::n_primes(*limit, cli.progress)
        );
        return Ok(());
    }

    // Calculate and print prime numbers up to the n'th number
    if let Some(nth) = cli.nth.as_ref() {
        if let Some(p) = calculate::nth_prime(*nth, cli.progress) {
            println!("Number {} of all primes is {:?}", *nth, p);
        }
        return Ok(());
    }

    // Interactive repl
    if cli.repl {
        interactive(cli.progress)?
    }
    Ok(())
}

// Interactive program
fn interactive(progress: bool) -> Result<(), Box<dyn error::Error>> {
    println!("Prime Number Calculator\n");
    println!("Calculate prime numbers up to N");
    println!("Type \"exit\" or \"quit\" when finished");

    loop {
        let mut input = String::new();

        print!("\nWhat number of all primes do you want? ");
        io::stdout().flush().unwrap();

        // Read a line of user input
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // If user input is quit, then break program loop
        if input.trim() == "exit" || input.trim() == "quit" {
            println!("Goodbye!");
            return Ok(());
        }

        // Parse input into a number
        let nth: usize = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Invalid input!");
                continue;
            }
        };

        // Calculate the n'th prime number
        if let Some(p) = calculate::nth_prime(nth, progress) {
            println!("Number {} of all primes is {:?}", nth, p);
        }
    }
}
