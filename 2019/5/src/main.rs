use std::{env, error::Error, process::exit};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    let mut interpreter = intcode::Interpreter::from_file(&args[1], vec![1])?;

    // let term = console::Term::stdout();
    //
    // while (interpreter.exec_one()?).is_none() {
    //     term.read_key()?;
    //     println!("\n{}", &interpreter);
    // }

    // println!("\nOutput: {:?}", interpreter.output());

    interpreter.exec()?;
    println!(
        "Part 1: {:?}",
        interpreter.output().last().ok_or("Empty output")?
    );

    Ok(())
}
