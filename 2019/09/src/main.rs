use std::{env, error::Error, process::exit};

async fn run_boost(
    program_file: &str,
    input: Vec<i64>,
) -> Result<i64, Box<dyn Error + Send + Sync>> {
    let mut interpreter = intcode::Interpreter::from_file(program_file, input).await?;
    interpreter.exec().await?;
    Ok(*interpreter
        .output_history()
        .first()
        .ok_or("Program had no output")?)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    println!("Part 1: {:?}", run_boost(&args[1], vec![1]).await?);
    println!("Part 2: {:?}", run_boost(&args[1], vec![2]).await?);

    Ok(())
}
