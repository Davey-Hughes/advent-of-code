use std::{env, error::Error, process::exit};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    let mut interpreter = intcode::Interpreter::from_file(&args[1], vec![1])?;

    interpreter.exec()?;
    println!(
        "Part 1: {:?}",
        interpreter.output().last().ok_or("Empty output")?
    );

    let mut interpreter = intcode::Interpreter::from_file(&args[1], vec![5])?;

    interpreter.exec()?;
    println!(
        "Part 2: {:?}",
        interpreter.output().last().ok_or("Empty output")?
    );

    // let mut interpreter = intcode::Interpreter::from_file(&args[1], vec![5])?;
    //
    // let term = console::Term::stdout();
    //
    // loop {
    //     term.clear_screen()?;
    //     println!("\n{}", &interpreter);
    //     term.read_key()?;
    //
    //     if interpreter.exec_one()?.is_some() {
    //         break;
    //     }
    // }
    //
    // println!("\nOutput: {:?}", interpreter.output());

    Ok(())
}
