use std::{
    collections::{HashSet, VecDeque},
    env,
    error::Error,
    fs,
};

struct Garden {
    map: Vec<Vec<char>>,
}

impl Garden {
    fn plot_perimeter_iter(&self, start: (usize, usize)) -> PlotPerimeterIter<'_> {
        PlotPerimeterIter {
            garden: self,
            plant: self.map[start.1][start.0],
            queue: VecDeque::from(vec![start]),
            seen: HashSet::from([start]),
        }
    }

    /// Returns the up, down, left, and right neighbors of a coordinate, as an iterator by
    /// performing bounds checking
    fn neighbors(
        &self,
        coords: (usize, usize),
    ) -> impl Iterator<Item = (char, (usize, usize))> + Clone + '_ {
        [
            (coords.0.checked_sub(1), Some(coords.1)),
            (coords.0.checked_add(1), Some(coords.1)),
            (Some(coords.0), coords.1.checked_sub(1)),
            (Some(coords.0), coords.1.checked_add(1)),
        ]
        .into_iter()
        .filter_map(|(x, y)| Some((x?, y?)))
        .filter(|(x, y)| *x < self.map[0].len() && *y < self.map.len())
        .map(|(x, y)| (self.map[y][x], (x, y)))
    }

    /// counts the number of corners this spot in a plot has by checking 3 spots around the input
    /// coordinates ([left, up-left, up], [up, up-right, right], ...)
    ///
    /// converts any coordinates that are out of bounds of the garden or not part of the same plot
    /// as the input coordinates to None
    fn corners(&self, coords: (usize, usize), plot_spots: &HashSet<(usize, usize)>) -> usize {
        let all_neighbors = [
            (coords.0.checked_sub(1), Some(coords.1)),
            (coords.0.checked_sub(1), coords.1.checked_sub(1)),
            (Some(coords.0), coords.1.checked_sub(1)),
            (coords.0.checked_add(1), coords.1.checked_sub(1)),
            (coords.0.checked_add(1), Some(coords.1)),
            (coords.0.checked_add(1), coords.1.checked_add(1)),
            (Some(coords.0), coords.1.checked_add(1)),
            (coords.0.checked_sub(1), coords.1.checked_add(1)),
            (coords.0.checked_sub(1), Some(coords.1)),
        ]
        .into_iter()
        .map(|(x, y)| Some((x?, y?)))
        .map(|neighbor| {
            if let Some((x, y)) = neighbor {
                if x < self.map[0].len() && y < self.map.len() && plot_spots.contains(&(x, y)) {
                    return neighbor;
                }
            }

            None
        })
        .collect::<Vec<_>>();

        // if a triplet corresponding to the corners ([left, up-left, up], [up, up-right, right], ...)
        // matches one of the patterns below, there is a fence corner at this spot
        #[allow(clippy::unnested_or_patterns)]
        all_neighbors
            .windows(3)
            .step_by(2)
            .map(|triplet| match triplet {
                [None, None, None] | [Some(_), None, Some(_)] | [None, Some(_), None] => 1,
                _ => 0,
            })
            .sum::<usize>()
    }
}

struct PlotPerimeterIter<'a> {
    garden: &'a Garden,
    plant: char,
    queue: VecDeque<(usize, usize)>,
    seen: HashSet<(usize, usize)>,
}

impl Iterator for PlotPerimeterIter<'_> {
    type Item = (usize, (usize, usize));

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.queue.pop_front()?;
        let mut perimeter = 4;

        for (_, neighbor) in self
            .garden
            .neighbors(n)
            .filter(|(plant, _)| *plant == self.plant)
        {
            perimeter -= 1;

            if !self.seen.contains(&neighbor) {
                self.seen.insert(neighbor);
                self.queue.push_back(neighbor);
            }
        }

        Some((perimeter, n))
    }
}

fn parse_input(file_string: &str) -> Garden {
    Garden {
        map: file_string.lines().map(|l| l.chars().collect()).collect(),
    }
}

fn part_1(garden: &Garden) -> usize {
    let mut seen = HashSet::new();

    (0..garden.map[0].len())
        .flat_map(|x| (0..garden.map.len()).map(move |y| (x, y)))
        .map(|(x, y)| {
            if seen.contains(&(x, y)) {
                return 0;
            }

            let mut perimeter = 0;
            let mut area = 0;

            for spot in garden.plot_perimeter_iter((x, y)) {
                seen.insert(spot.1);

                perimeter += spot.0;
                area += 1;
            }

            area * perimeter
        })
        .sum()
}

fn part_2(garden: &Garden) -> usize {
    let mut seen = HashSet::new();

    (0..garden.map[0].len())
        .flat_map(|x| (0..garden.map.len()).map(move |y| (x, y)))
        .map(|(x, y)| {
            if seen.contains(&(x, y)) {
                return 0;
            }

            let mut area = 0;
            let mut plot_seen = HashSet::new();

            for spot in garden.plot_perimeter_iter((x, y)) {
                seen.insert(spot.1);
                plot_seen.insert(spot.1);

                area += 1;
            }

            // the number of straight sides on a plot corresponds to the number of corners the plot
            // fencing has
            let sides = plot_seen
                .iter()
                .map(|spot| garden.corners(*spot, &plot_seen))
                .sum::<usize>();

            area * sides
        })
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(env::args().nth(1).expect("Input file expected as argument"))?;

    let garden = parse_input(&contents);

    println!("Part 1: {:?}", part_1(&garden));
    println!("Part 2: {:?}", part_2(&garden));

    Ok(())
}
