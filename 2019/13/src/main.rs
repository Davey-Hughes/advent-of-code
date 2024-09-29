use core::time;
use std::{cmp::Ordering, collections::HashMap, env, error::Error, process::exit, thread};

use console::Term;

fn draw_screen(screen: &HashMap<(i64, i64), i64>, score: i64) -> String {
    let mut min_x = i64::MAX;
    let mut min_y = i64::MAX;
    let mut max_x = i64::MIN;
    let mut max_y = i64::MIN;

    for (x, y) in screen.keys() {
        min_x = min_x.min(*x);
        min_y = min_y.min(*y);
        max_x = max_x.max(*x);
        max_y = max_y.max(*y);
    }

    let mut output = String::new();
    output.push_str(format!("Score: {score}\n").as_str());

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let tile = match screen.get(&(x, y)) {
                Some(1) => '█',
                Some(2) => '▒',
                Some(3) => '▬',
                Some(4) => 'o',
                _ => ' ',
            };
            output.push(tile);
        }
        output.push('\n');
    }

    output
}

async fn part_1(program_file: &str) -> Result<usize, Box<dyn Error + Send + Sync>> {
    let mut interpreter = intcode::Interpreter::from_file(program_file, vec![]).await?;
    interpreter.exec().await?;

    let output = interpreter.output_history();
    Ok(output.chunks(3).filter(|chunk| chunk[2] == 2).count())
}

async fn part_2(
    program_file: &str,
    draw_game: bool,
) -> Result<usize, Box<dyn Error + Send + Sync>> {
    let mut interpreter = intcode::Interpreter::from_file(program_file, vec![]).await?;
    interpreter.set_memory(0, 2);
    let (input_tx, mut output_rx) = interpreter.exec_spawn()?;

    let mut term = if draw_game {
        Some(Term::stdout())
    } else {
        None
    };

    let mut screen: HashMap<(i64, i64), i64> = HashMap::new();
    let mut score = 0;
    let mut paddle_x = 0;
    let mut ball_x;

    loop {
        let mut output = [0; 3];
        for i in &mut output {
            if let Some(val) = output_rx.recv().await {
                *i = val;
            }
        }

        if output[0] == -1 && output[1] == 0 {
            score = output[2];
        } else {
            screen.insert((output[0], output[1]), output[2]);
        }

        if output[2] == 3 {
            paddle_x = output[0];
        }

        if output[2] == 4 {
            ball_x = output[0];
            break;
        }
    }

    loop {
        if let Some(term) = &mut term {
            term.clear_screen()?;
            term.write_line(&draw_screen(&screen, score))?;
            thread::sleep(time::Duration::from_millis(20));
        }

        let paddle_dir = match paddle_x.cmp(&ball_x) {
            Ordering::Less => 1,
            Ordering::Greater => -1,
            Ordering::Equal => 0,
        };

        match input_tx.send(paddle_dir).await {
            Ok(()) => {}
            Err(_) => break,
        }

        loop {
            let mut output = [0; 3];
            for i in &mut output {
                if let Some(val) = output_rx.recv().await {
                    *i = val;
                } else {
                    if let Some(term) = &mut term {
                        term.clear_screen()?;
                        term.write_line(&draw_screen(&screen, score))?;
                    }
                    return Ok(score.try_into()?);
                }
            }

            if output[0] == -1 && output[1] == 0 {
                score = output[2];
            } else {
                screen.insert((output[0], output[1]), output[2]);
            }

            if output[2] == 3 {
                paddle_x = output[0];
            }

            if output[2] == 4 {
                ball_x = output[0];
                break;
            }
        }
    }

    Ok(score.try_into()?)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    println!("Part 1: {:?}", part_1(&args[1]).await?);
    println!("Part 2: {}", part_2(&args[1], false).await?);

    Ok(())
}
