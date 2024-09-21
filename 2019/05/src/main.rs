use std::{env, error::Error, process::exit};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    let mut interpreter = intcode::Interpreter::from_file(&args[1], vec![1]).await?;

    interpreter.exec().await?;
    let mut outputs = vec![];
    while let Some(message) = interpreter.output().await {
        outputs.push(message);
    }
    println!("Part 1: {:?}", outputs.last().ok_or("Empty output")?);

    let mut interpreter = intcode::Interpreter::from_file(&args[1], vec![5]).await?;

    interpreter.exec().await?;
    println!(
        "Part 2: {:?}",
        interpreter.output().await.ok_or("Empty output")?
    );

    Ok(())
}
