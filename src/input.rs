use std::{collections::HashSet, fmt::Display};

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use crate::Position;

#[derive(Debug)]
pub struct Input {
    pub map: Vec<char>,
    pub width: usize,
    pub height: usize,
}

impl Input {
    pub fn new(width: usize, height: usize) -> Self {
        let len = width * height;
        Input {
            map: vec!['.'; len],
            width,
            height,
            // starting_points: HashSet::new(),
        }
    }

    fn position(&self, coord: usize) -> Position {
        Position {
            x: (coord % self.width),
            y: (coord / self.width),
        }
    }

    fn coord(&self, pos: &Position) -> usize {
        (pos.y * self.width) + pos.x
    }

    fn neighbours(&self, pos: &Position, target: char) -> Vec<Position> {
        let mut neighbours = vec![];

        if pos.y > 0 {
            let neighbour = Position {
                x: pos.x,
                y: pos.y - 1,
            };
            if self.map[self.coord(&neighbour)] == '.' || self.map[self.coord(&neighbour)] == target
            {
                neighbours.push(neighbour);
            }
        }
        if pos.y < self.height - 1 {
            let neighbour = Position {
                x: pos.x,
                y: pos.y + 1,
            };
            if self.map[self.coord(&neighbour)] == '.' || self.map[self.coord(&neighbour)] == target
            {
                neighbours.push(neighbour);
            }
        }

        if pos.x < self.width - 1 {
            let neighbour = Position {
                x: pos.x + 1,
                y: pos.y,
            };
            if self.map[self.coord(&neighbour)] == '.' || self.map[self.coord(&neighbour)] == target
            {
                neighbours.push(neighbour);
            }
        }
        if pos.x > 0 {
            let neighbour = Position {
                x: pos.x - 1,
                y: pos.y,
            };
            if self.map[self.coord(&neighbour)] == '.' || self.map[self.coord(&neighbour)] == target
            {
                neighbours.push(neighbour);
            }
        }
        neighbours
    }

    fn pick_direction(
        &self,
        start: &Position,
        rng: &mut ChaCha8Rng,
        target: char,
    ) -> Option<Position> {
        let options = self.neighbours(start, target);
        if options.len() == 0 {
            dbg!(&options, start);
            return None;
        }

        options.choose(rng).copied()
    }

    pub fn depth_first(
        &self,
        visited: &mut Vec<Position>,
        trail: &mut Vec<Position>,
        current: &Position,
        mut rng: &mut ChaCha8Rng,
    ) -> Vec<Position> {
        let current_val = trail.len();
        trail.push(*current);
        visited.push(*current);

        if trail.len() == 10 {
            return trail.to_vec();
        }
        let target = char::from_digit(current_val as u32 + 1, 10).expect("failed to parse num");
        let mut neighbours = self.neighbours(current, target);
        neighbours.shuffle(&mut rng);

        for pos in neighbours {
            if !visited.contains(&pos) && current_val < 9 && trail.len() < 10 {
                // if current_val == 9 {
                // return trail.to_vec();
                // } else {
                // let neighbour_val = self.map[self.coord(&pos)];
                self.depth_first(visited, trail, &pos, rng);
                // }
            }
        }

        trail.to_vec()
    }

    pub fn add_trails(&mut self, starting_points: &HashSet<Position>, rng: &mut ChaCha8Rng) {
        let mut trails = vec![];

        for pos in starting_points.iter() {
            let mut visited = vec![];
            let mut trail = vec![];

            trails.push(self.depth_first(&mut visited, &mut trail, pos, rng));
            // update map
            for (i, point) in trail.iter().enumerate() {
                let coord = self.coord(point);
                self.map[coord] = char::from_digit(i as u32, 10).expect("Couldn't parse index");
            }
            trail.truncate(0);
        }
        // iter visited and update map
        dbg!(trails);
    }

    // TODO this should be depth first
    pub fn random_walk(&mut self, starting_points: &HashSet<Position>, rng: &mut ChaCha8Rng) {
        // each start point
        // do a random walk to 9
        // if already set and not next num
        // pick another direction
        // starting_points.iter().for_each(|start| {
        let mut iter = starting_points.iter();
        for _ in 0..3 {
            let start = iter.next().unwrap();
            // let mut current = Position { x: 2, y: 4 };
            let mut current = *start;
            let current_coord = self.coord(&current);
            self.map[current_coord] = '0';

            for i in 0..9 {
                let target = char::from_digit(i + 1, 10).expect("Failed to parse number");
                let pos = self.pick_direction(&current, rng, target);
                // dbg!(&pos);

                match pos {
                    None => {
                        dbg!("no spaces", i);

                        // TODO start again from 0 and pick different dir
                        return;
                    }
                    Some(pos) => {
                        let coord = (pos.y * self.width) + pos.x;
                        current = pos;

                        // let val = self.map[coord];
                        // if val == '.' {
                        // dbg!(val);

                        // update map - Rust unhappy with multiple ref to self
                        let mut map = std::mem::take(&mut self.map);
                        map[coord] =
                            char::from_digit(i as u32 + 1, 10).expect("Failed to convert to char");
                        self.map = map;
                        // } else if val.to_digit(10).expect("Failed to parse current value")
                        //     != i as u32 + 1
                        // {
                        //     dbg!("not adding");
                        //     return;
                        // }
                    }
                }
            }
        }
        // })
    }

    pub fn fill(&mut self, rng: &mut ChaCha8Rng) {
        self.map = self
            .map
            .iter()
            .map(|i| {
                if *i == '.' {
                    return char::from_digit(rng.gen_range(0..=9), 10)
                        .expect("Couldn't parse number");
                }
                *i
            })
            .collect()
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.map.iter().enumerate().for_each(|(index, item)| {
            let pos = self.position(index);
            // dbg!(&pos, self.starting_points.get(&pos));
            // if self.starting_points.get(&pos).is_some() && item == &'.' {
            // write!(f, "0").expect("Failed to write antinode");
            // } else {
            write!(f, "{}", item).expect("Failed to write item");
            // }
            if (index + 1) % self.width == 0 {
                write!(f, "\n").expect("Failed to add new line");
            }
        });
        Ok(())
    }
}
