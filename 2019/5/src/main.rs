use std::{env, error::Error, process::exit};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    let interpreter = intcode::Interpreter::from_file(&args[1])?;

    println!("{interpreter}");

    Ok(())
}
