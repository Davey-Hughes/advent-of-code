use std::{cmp, env, error::Error, fs, process::exit};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Vertex {
    x: i32,
    y: i32,
}

impl Vertex {
    #[allow(dead_code)]
    const fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    const fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    const fn cross(self, other: Self) -> i32 {
        self.x * other.y - self.y * other.x
    }
}

#[derive(Debug)]
struct InterDetails {
    intersection: Vertex,
    wire_a_index: usize,
    wire_b_index: usize,
}

#[derive(Debug, Clone)]
struct Wire {
    vertices: Vec<Vertex>,
}

impl Wire {
    fn segment_iter(&self) -> impl Iterator<Item = (&Vertex, &Vertex)> + '_ {
        self.vertices.iter().zip(self.vertices.iter().skip(1))
    }
}

fn parse_input<S: AsRef<str>>(file_string: &S) -> Result<Vec<Wire>, Box<dyn Error>> {
    let re = regex::Regex::new(r"([RLUD])(\d+)[,|\n]?")?;

    let mut wires = vec![];

    for hay in file_string.as_ref().lines() {
        let mut wire = vec![];
        let mut cur_x = 0;
        let mut cur_y = 0;

        wire.push(Vertex { x: cur_x, y: cur_y });

        for (_, [direction, dist]) in re.captures_iter(hay).map(|c| c.extract()) {
            let distance = dist.parse::<i32>()?;

            match direction {
                "U" => cur_y += distance,
                "D" => cur_y -= distance,
                "R" => cur_x += distance,
                "L" => cur_x -= distance,
                _ => {
                    println!("Error: unexpected direction: {direction}");
                    break;
                }
            };

            wire.push(Vertex { x: cur_x, y: cur_y });
        }

        wires.push(Wire { vertices: wire });
    }

    Ok(wires)
}

#[allow(clippy::many_single_char_names)]
fn wire_intersections(wires: &[Wire]) -> Vec<InterDetails> {
    fn segment_intersection(p0: Vertex, p1: Vertex, q0: Vertex, q1: Vertex) -> Option<Vertex> {
        let r = p1.sub(p0);
        let s = q1.sub(q0);

        let numerator = q0.sub(p0).cross(r);
        let denominator = r.cross(s);

        if denominator == 0 {
            return None;
        }

        let u = f64::from(numerator) / f64::from(denominator);
        let t = f64::from(q0.sub(p0).cross(s)) / f64::from(denominator);

        if (0.0..=1.0).contains(&u) && (0.0..=1.0).contains(&t) {
            if p0.x == p1.x {
                return Some(Vertex { x: p0.x, y: q0.y });
            }

            return Some(Vertex { x: q0.x, y: p0.y });
        }

        None
    }

    let mut intersections = vec![];

    for (wire, rest) in wires[..wires.len() - 1]
        .iter()
        .enumerate()
        .map(|(i, w)| (w, &wires[i + 1..]))
    {
        for (i, (p0, p1)) in wire.segment_iter().enumerate() {
            for other_wire in rest {
                for (k, (q0, q1)) in other_wire.segment_iter().enumerate() {
                    if let Some(intersection) = segment_intersection(*p0, *p1, *q0, *q1) {
                        if intersection != (Vertex { x: 0, y: 0 }) {
                            intersections.push(InterDetails {
                                intersection,
                                wire_a_index: i,
                                wire_b_index: k,
                            });
                        }
                    }
                }
            }
        }
    }

    intersections
}

fn min_taxicab_distance(intersections: &[Vertex]) -> Option<i32> {
    intersections.iter().map(|v| v.x.abs() + v.y.abs()).min()
}

#[allow(clippy::similar_names)]
fn min_wire_distance(intersections: &[InterDetails], wires: &[Wire]) -> i32 {
    fn wire_distance(wire: &Wire, cumulative_dist: &mut Vec<i32>, point_index: usize) -> i32 {
        if let Some(distance) = cumulative_dist.get(point_index - 1) {
            return *distance;
        }

        if cumulative_dist.is_empty() {
            let (p0, p1) = (wire.vertices[0], wire.vertices[1]);
            cumulative_dist.push((p1.x - p0.x).abs() + (p1.y - p0.y).abs());
        }

        for i in (cumulative_dist.len() - 1)..point_index {
            let (p0, p1) = (wire.vertices[i + 1], wire.vertices[i + 2]);
            cumulative_dist.push(cumulative_dist[i] + (p1.x - p0.x).abs() + (p1.y - p0.y).abs());
        }

        cumulative_dist[point_index - 1]
    }

    let mut ret = i32::MAX;
    let mut cumulative_dists = [vec![], vec![]];

    for intersection in intersections {
        let wire_a_point = wires[0].vertices[intersection.wire_a_index];
        let wire_a_dist = wire_distance(
            &wires[0],
            &mut cumulative_dists[0],
            intersection.wire_a_index,
        ) + (intersection.intersection.x - wire_a_point.x).abs()
            + (intersection.intersection.y - wire_a_point.y).abs();

        let wire_b_point = wires[1].vertices[intersection.wire_b_index];
        let wire_b_dist = wire_distance(
            &wires[1],
            &mut cumulative_dists[1],
            intersection.wire_b_index,
        ) + (intersection.intersection.x - wire_b_point.x).abs()
            + (intersection.intersection.y - wire_b_point.y).abs();

        ret = cmp::min(ret, wire_a_dist + wire_b_dist);
    }

    ret
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input file expected as argument");
        exit(1);
    }

    let contents = fs::read_to_string(&args[1])?;

    let wires = parse_input(&contents)?;

    println!(
        "Part 1: {}",
        min_taxicab_distance(
            &wire_intersections(&wires)
                .iter()
                .map(|x| x.intersection)
                .collect::<Vec<_>>()
        )
        .unwrap()
    );

    println!(
        "Part 2: {}",
        min_wire_distance(&wire_intersections(&wires), &wires)
    );

    Ok(())
}
