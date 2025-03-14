use rand::prelude::*;
use std::fmt::Display;

use crate::map::Position;

#[derive(Debug, PartialEq)]
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

    fn depth_first(
        &self,
        visited: &mut Vec<Position>,
        trail: &mut Vec<Position>,
        current: &Position,
        mut rng: &mut SmallRng,
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
                self.depth_first(visited, trail, &pos, rng);
            }
        }

        trail.to_vec()
    }

    pub fn add_trails(&mut self, starting_points: &[Position], rng: &mut SmallRng) {
        let mut trails = vec![];

        for pos in starting_points.iter() {
            let mut visited = vec![];
            let mut trail = vec![];

            trails.push(self.depth_first(&mut visited, &mut trail, pos, rng));
            if trail.len() == 10 {
                // update map
                for (i, point) in trail.iter().enumerate() {
                    let coord = self.coord(point);
                    self.map[coord] = char::from_digit(i as u32, 10).expect("Couldn't parse index");
                }
            }
            trail.truncate(0);
        }
    }

    pub fn fill(&mut self, rng: &mut SmallRng) {
        self.map = self
            .map
            .iter()
            .map(|i| {
                if *i == '.' {
                    return char::from_digit(rng.random_range(0..=9), 10)
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
            write!(f, "{}", item).expect("Failed to write item");
            if (index + 1) % self.width == 0 {
                writeln!(f).expect("Failed to add new line");
            }
        });
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use rand::{SeedableRng, rngs::SmallRng};

    use crate::{input::Input, map::Position};

    #[test]
    fn it_should_format_correctly() {
        let input = Input::new(6, 6);
        assert_eq!(
            format!("{}", input),
            "......
......
......
......
......
......
"
        );
    }

    #[test]
    fn it_should_add_trails() {
        let mut input = Input::new(6, 6);
        let starting_points = vec![
            Position { x: 1, y: 0 },
            Position { x: 2, y: 4 },
            Position { x: 3, y: 0 },
            Position { x: 5, y: 5 },
        ];

        let seed = 123;
        let mut rng = SmallRng::seed_from_u64(seed);

        input.add_trails(&starting_points, &mut rng);

        assert_eq!(
            input,
            Input {
                width: 6,
                height: 6,
                map: vec![
                    '1', '0', '.', '0', '1', '.', '2', '.', '9', '.', '2', '.', '3', '4', '8', '4',
                    '3', '9', '8', '5', '7', '5', '6', '8', '7', '6', '0', '1', '6', '7', '9', '4',
                    '3', '2', '5', '.'
                ]
            }
        );
    }

    #[test]
    fn it_should_fill_blank_spaces() {
        let mut input = Input::new(10, 10);
        let starting_points = vec![
            Position { x: 1, y: 0 },
            Position { x: 2, y: 4 },
            Position { x: 3, y: 0 },
            Position { x: 5, y: 5 },
        ];

        let seed = 123;
        let mut rng = SmallRng::seed_from_u64(seed);

        input.add_trails(&starting_points, &mut rng);
        input.fill(&mut rng);

        assert_eq!(
            input,
            Input {
                width: 10,
                height: 10,
                map: vec![
                    '1', '0', '5', '0', '7', '8', '9', '2', '4', '4', '2', '6', '0', '1', '6', '5',
                    '9', '1', '6', '2', '3', '4', '0', '2', '3', '4', '8', '4', '7', '3', '8', '5',
                    '9', '8', '5', '6', '7', '6', '1', '1', '7', '6', '0', '1', '4', '9', '8', '5',
                    '1', '8', '9', '2', '1', '2', '3', '0', '1', '4', '6', '5', '6', '7', '7', '5',
                    '9', '6', '2', '3', '7', '0', '8', '1', '4', '9', '6', '8', '1', '4', '7', '5',
                    '2', '4', '1', '1', '2', '9', '2', '1', '3', '4', '6', '4', '2', '2', '4', '5',
                    '1', '1', '4', '3'
                ]
            }
        );
    }
}
