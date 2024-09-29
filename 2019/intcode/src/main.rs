use std::{env, process::exit};

use debugger::DebuggerResult;

mod debugger;
mod utils;

#[tokio::main]
async fn main() -> DebuggerResult<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    // let mut interpreter = intcode::Interpreter::from_file(&args[1], vec![]).await?;
    // interpreter.exec().await?;
    // println!("{:?}", interpreter.output_history());
    // Ok(())

    debugger::start(&args[1], vec![]).await
}
