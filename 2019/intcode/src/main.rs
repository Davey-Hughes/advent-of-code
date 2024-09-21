use std::{env, error::Error, process::exit};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    let mut interpreter = intcode::Interpreter::from_file(&args[1], vec![5]).await?;

    let term = console::Term::stdout();

    loop {
        term.clear_screen()?;
        println!("\n{}", &interpreter);
        term.read_key()?;

        if interpreter.exec_one().await?.is_some() {
            break;
        }
    }

    // println!("\nOutput: {:?}", interpreter.output());

    Ok(())
}
