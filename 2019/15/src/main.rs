use std::{
    collections::{HashSet, VecDeque},
    env,
    error::Error,
};
use tokio::sync::mpsc;

use intcode::Interpreter;

#[derive(Debug)]
struct Robot {
    input_tx: mpsc::Sender<i64>,
    output_rx: mpsc::UnboundedReceiver<i64>,
    position: (i64, i64),
}

#[derive(PartialEq)]
enum SearchType {
    OxygenSystem,
    FillOxygen,
}

impl Robot {
    fn input_to_coords(&self, input: i64) -> (i64, i64) {
        match input {
            1 => (self.position.0, self.position.1 - 1),
            2 => (self.position.0, self.position.1 + 1),
            3 => (self.position.0 - 1, self.position.1),
            4 => (self.position.0 + 1, self.position.1),
            _ => panic!("Invalid input to input_to_coords"),
        }
    }

    fn move_robot(&mut self, dir: i64) {
        self.position = self.input_to_coords(dir);
    }

    fn invert(input: i64) -> i64 {
        match input {
            1 => 2,
            2 => 1,
            3 => 4,
            4 => 3,
            _ => panic!("Invalid input to invert"),
        }
    }

    async fn search(
        &mut self,
        searchtype: SearchType,
    ) -> Result<usize, Box<dyn Error + Send + Sync>> {
        let mut seen = HashSet::new();
        let mut queue: VecDeque<((i64, i64), Vec<i64>)> = VecDeque::from([(self.position, vec![])]);
        let mut prev_path = vec![];
        let mut longest = 0;

        while let Some((pos, path)) = queue.pop_front() {
            if seen.insert(pos) {
                if path.len() > longest {
                    longest = path.len();
                }

                let diff_index = diff(&path, &prev_path);

                // move self back to last last shared position
                for dir in prev_path[diff_index..].iter().rev() {
                    self.move_robot(Robot::invert(*dir));
                    self.input_tx.send(Robot::invert(*dir)).await?;
                    self.output_rx.recv().await;
                }

                // apply the remaining path for current position
                for dir in &path[diff_index..] {
                    self.move_robot(*dir);
                    self.input_tx.send(*dir).await?;
                    self.output_rx.recv().await;
                }

                prev_path.clone_from(&path);

                for dir in 1..=4i64 {
                    self.input_tx.send(dir).await?;
                    match self.output_rx.recv().await {
                        Some(0) => {}
                        Some(1) => {
                            let mut path = path.clone();
                            path.push(dir);
                            queue.push_back((self.input_to_coords(dir), path));

                            self.input_tx.send(Robot::invert(dir)).await?;
                            self.output_rx.recv().await;
                        }
                        Some(2) => {
                            if searchtype == SearchType::OxygenSystem {
                                return Ok(path.len() + 1);
                            }
                        }
                        Some(o) => {
                            return Err(format!("Unexpected output from program: {o}").into())
                        }
                        None => return Err("Output channel closed unexpectedly".into()),
                    }
                }
            }
        }

        match searchtype {
            SearchType::OxygenSystem => Err("Cannot find oxygen system".into()),
            SearchType::FillOxygen => Ok(longest),
        }
    }

    async fn find_oxygen_system(&mut self) -> Result<usize, Box<dyn Error + Send + Sync>> {
        self.search(SearchType::OxygenSystem).await
    }

    async fn fill_oxygen(&mut self) -> Result<usize, Box<dyn Error + Send + Sync>> {
        self.search(SearchType::FillOxygen).await
    }
}

fn diff<T>(a: impl IntoIterator<Item = T>, b: impl IntoIterator<Item = T>) -> usize
where
    T: PartialEq,
{
    for (i, (a, b)) in a.into_iter().zip(b.into_iter()).enumerate() {
        if a != b {
            return i;
        }
    }

    0
}

async fn part_1(robot: &mut Robot) -> Result<usize, Box<dyn Error + Send + Sync>> {
    robot.find_oxygen_system().await
}

async fn part_2(robot: &mut Robot) -> Result<usize, Box<dyn Error + Send + Sync>> {
    robot.fill_oxygen().await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let program_file = env::args().nth(1).expect("Input file expected as argument");

    let (input_tx, output_rx) = Interpreter::from_file(&program_file, vec![])
        .await?
        .exec_spawn()?;

    let mut robot = Robot {
        input_tx,
        output_rx,
        position: (0, 0),
    };

    println!("Part 1: {:?}", part_1(&mut robot).await?);
    println!("Part 2: {:?}", part_2(&mut robot).await?);

    Ok(())
}
