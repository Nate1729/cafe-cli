use std::{env, io};
use io::Write;

mod pour_over;
use pour_over::PourOver;


enum BrewType {
    PourOver
}

fn main() {
    let cli_args: Vec<String> = env::args().collect();
    if cli_args.len() < 2 {
        println!("Hey you didn't pass me an argument! I don't know what to do!"); 
    } else {
    
        let command = &cli_args[1];

        if command == "brew" {
            println!("Starting brewing process!");
            configure_brew();
        }
    }
}

fn configure_brew() {
    let brew_type = get_brew_type().expect("Error getting brew Type");

    // TODO Make grind_size configureable
    let grind_size = 14;

    match brew_type {
        BrewType::PourOver => PourOver::configure_brew(grind_size),
    };
}

fn get_brew_type() -> io::Result<BrewType> { 
    let brew_type = loop {
        let mut buffer = String::new();
        print!("\nPlease select brewing process\n");
        print!("(1) Pour Over (Chemx, V60, etc.)");
        io::stdout().flush()?;
        io::stdin().read_line(&mut buffer)?;

        // Attempt to convert input to number
        let num: u32 = match buffer.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        if num == 1 {
            break BrewType::PourOver;
        }
    };  
    Ok(brew_type)
}
