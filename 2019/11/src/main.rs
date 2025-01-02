use core::fmt;
use std::{env, error::Error, fmt::Formatter, process::exit};

static DEFAULT_CHAR: char = '⠀';

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Panel {
    Black,
    White,
    Default,
}

impl fmt::Display for Panel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Black => '.',
                Self::White => '#',
                Self::Default => DEFAULT_CHAR,
            }
        )
    }
}

impl From<&Panel> for char {
    fn from(val: &Panel) -> Self {
        match val {
            Panel::Black => '.',
            Panel::White => '#',
            Panel::Default => DEFAULT_CHAR,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(&self, direction: i64) -> Self {
        match (self, direction) {
            (Self::Up, 0) | (Self::Down, 1) => Self::Left,
            (Self::Up, 1) | (Self::Down, 0) => Self::Right,
            (Self::Left, 0) | (Self::Right, 1) => Self::Down,
            (Self::Left, 1) | (Self::Right, 0) => Self::Up,
            _ => panic!("Invalid turn"),
        }
    }
}

struct Robot {
    x: usize,
    y: usize,
    direction: Direction,
}

impl fmt::Display for Robot {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.direction.clone() {
                Direction::Up => '^',
                Direction::Down => 'v',
                Direction::Left => '<',
                Direction::Right => '>',
            }
        )
    }
}

struct Hull {
    panels: Vec<Vec<Panel>>,
    robot: Robot,
}

impl Hull {
    fn new() -> Self {
        Self {
            panels: vec![vec![Panel::Default; 1]; 1],
            robot: Robot {
                x: 0,
                y: 0,
                direction: Direction::Up,
            },
        }
    }

    fn paint(&mut self, color: Panel) {
        self.panels[self.robot.y][self.robot.x] = color;
    }

    fn move_robot(&mut self, direction: i64) {
        self.robot.direction = self.robot.direction.turn(direction);

        match self.robot.direction {
            Direction::Up => self.robot.y = self.robot.y.wrapping_sub(1),
            Direction::Down => self.robot.y = self.robot.y.wrapping_add(1),
            Direction::Left => self.robot.x = self.robot.x.wrapping_sub(1),
            Direction::Right => self.robot.x = self.robot.x.wrapping_add(1),
        }

        if self.panels.get(self.robot.y).is_none() {
            self.panels.push(vec![Panel::Default; self.panels[0].len()]);

            if self.robot.direction == Direction::Up {
                self.panels.rotate_right(1);
                self.robot.y = self.robot.y.wrapping_add(1);
            }
        }

        if self.panels[0].get(self.robot.x).is_none() {
            for row in &mut self.panels {
                row.push(Panel::Default);
                if self.robot.direction == Direction::Left {
                    row.rotate_right(1);
                }
            }
            if self.robot.direction == Direction::Left {
                self.robot.x = self.robot.x.wrapping_add(1);
            }
        }
    }
}

impl fmt::Display for Hull {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut rows = vec![];
        for (i, row) in self.panels.iter().enumerate() {
            let mut row_str = String::new();
            for (k, panel) in row.iter().enumerate() {
                if i == self.robot.y && k == self.robot.x {
                    row_str.push('^');
                } else {
                    row_str.push(panel.into());
                }
            }
            rows.push(row_str);
        }

        write!(f, "{}", rows.join("\n"))?;

        Ok(())
    }
}

#[allow(clippy::match_on_vec_items)]
async fn paint(
    program_file: &str,
    start_color: Panel,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let interpreter = intcode::Interpreter::from_file(program_file, vec![]).await?;
    let (input_tx, mut output_rx) = interpreter.exec_spawn()?;

    let mut hull = Hull::new();
    hull.paint(start_color);

    loop {
        let panel = match hull.panels[hull.robot.y][hull.robot.x] {
            Panel::Black | Panel::Default => 0,
            Panel::White => 1,
        };

        match input_tx.send(panel).await {
            Ok(()) => (),
            Err(_) => break,
        }

        match output_rx.recv().await {
            Some(0) => hull.paint(Panel::Black),
            Some(1) => hull.paint(Panel::White),
            Some(n) => {
                println!("{n}");
                panic!("Invalid color");
            }
            None => break,
        }

        match output_rx.recv().await {
            Some(output) => hull.move_robot(output),
            None => break,
        }
    }

    Ok(hull.to_string())
}

async fn part_1(program_file: &str) -> Result<usize, Box<dyn Error + Send + Sync>> {
    let painted = paint(program_file, Panel::Default).await?;
    let panels_painted = painted
        .chars()
        .filter(|c| *c != DEFAULT_CHAR && *c != '\n')
        .count();

    Ok(panels_painted)
}

async fn part_2(program_file: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    paint(program_file, Panel::White).await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    println!("Part 1: {:?}", part_1(&args[1]).await?);
    println!("Part 2: \n{}", part_2(&args[1]).await?);

    Ok(())
}
