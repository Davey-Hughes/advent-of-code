use std::{cmp::max, env, error::Error, process::exit};

use itertools::Itertools;

async fn part_1(program_file: &str) -> Result<i64, Box<dyn Error>> {
    async fn try_phase(program_file: &str, phase_settings: &[i64]) -> Result<i64, Box<dyn Error>> {
        let mut forward = 0;
        for phase in phase_settings {
            let mut interpreter =
                intcode::Interpreter::from_file(program_file, vec![*phase, forward]).await?;

            interpreter.exec().await?;

            forward = interpreter
                .output()
                .await
                .ok_or("Program exited without output")?;
        }

        Ok(forward)
    }

    let mut res = 0;
    for phase_settings in (0..5).permutations(5) {
        res = max(res, try_phase(program_file, &phase_settings).await?);
    }

    Ok(res)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    println!("Part 1: {:?}", part_1(&args[1]).await?);

    // let mut interpreter = intcode::Interpreter::from_file(&args[1], vec![5])?;
    //
    // interpreter.exec()?;
    // println!(
    //     "Part 2: {:?}",
    //     interpreter.output().last().ok_or("Empty output")?
    // );

    // let mut interpreter = intcode::Interpreter::from_file(&args[1], vec![2, 43])?;
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
