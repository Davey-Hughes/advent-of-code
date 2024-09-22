use std::{cmp::max, env, error::Error, process::exit};

use itertools::Itertools;

async fn part_1(program_file: &str) -> Result<i64, Box<dyn Error + Send + Sync>> {
    async fn try_phase(
        program_file: &str,
        phase_settings: &[i64],
    ) -> Result<i64, Box<dyn Error + Send + Sync>> {
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
    for phase_settings in (0..=4).permutations(5) {
        res = max(res, try_phase(program_file, &phase_settings).await?);
    }

    Ok(res)
}

async fn part_2(program_file: &str) -> Result<i64, Box<dyn Error + Send + Sync>> {
    async fn try_phase(
        program_file: &str,
        phase_settings: &[i64],
    ) -> Result<i64, Box<dyn Error + Send + Sync>> {
        let mut forward = 0;
        let mut interpreters = vec![];

        for phase in phase_settings {
            let interpreter = intcode::Interpreter::from_file(program_file, vec![*phase]).await?;
            let interpreter_io = interpreter.exec_spawn()?;
            interpreters.push(interpreter_io);
        }

        let mut halted = vec![false; interpreters.len()];

        while halted.iter().any(|x| !(*x)) {
            for ((input_tx, output_rx), halted) in interpreters.iter_mut().zip(halted.iter_mut()) {
                match input_tx.send(forward).await {
                    Ok(()) => (),
                    Err(_) => *halted = true,
                }

                if let Some(output) = output_rx.recv().await {
                    forward = output;
                }
            }
        }
        Ok(forward)
    }

    let mut res = 0;
    for phase_settings in (5..=9).permutations(5) {
        res = max(res, try_phase(program_file, &phase_settings).await?);
    }

    Ok(res)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    println!("Part 1: {:?}", part_1(&args[1]).await?);
    println!("Part 2: {:?}", part_2(&args[1]).await?);

    Ok(())
}
