use std::{
    collections::{HashSet, VecDeque},
    env::args,
    fmt::Display,
    fs::{read_to_string, write},
    io,
};

use rand::{distributions::Standard, prelude::*};
use rand_chacha::ChaCha8Rng;

mod test;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Map {
    map: Vec<u8>,
    width: usize,
    height: usize,
    trailheads: Vec<Position>,
    paths: Vec<Vec<Position>>,
}
impl Map {
    fn parse(input: &str) -> Self {
        let mut height = 0;
        let mut trailheads = vec![];

        let map: Vec<u8> = input
            .lines()
            .flat_map(|line| {
                let row = line
                    .trim()
                    .chars()
                    .enumerate()
                    .map(|(index, n)| {
                        let val = n.to_digit(10).expect("Not a number") as u8;
                        if val == 0 {
                            trailheads.push(Position {
                                x: index,
                                y: height,
                            });
                        }
                        val
                    })
                    .collect::<Vec<_>>();
                height += 1;
                row
            })
            .collect();
        let width = map.len() / height;

        Map {
            width,
            height,
            map,
            trailheads,
            paths: vec![],
        }
    }

    // fn position(&self, coord: usize) -> Position {
    //     Position {
    //         x: (coord % self.width) as usize,
    //         y: (coord / self.width) as usize,
    //     }
    // }

    fn coord(&self, pos: &Position) -> usize {
        (pos.y * self.width) + pos.x
    }

    fn neighbours(&self, pos: &Position) -> Vec<Position> {
        let mut neighbours = vec![];

        if pos.y > 0 {
            neighbours.push(Position {
                x: pos.x,
                y: pos.y - 1,
            });
        }
        if pos.y < self.height - 1 {
            neighbours.push(Position {
                x: pos.x,
                y: pos.y + 1,
            });
        }

        if pos.x < self.width - 1 {
            neighbours.push(Position {
                x: pos.x + 1,
                y: pos.y,
            });
        }
        if pos.x > 0 {
            neighbours.push(Position {
                x: pos.x - 1,
                y: pos.y,
            });
        }
        neighbours
    }

    // nicked from https://www.technical-recipes.com/2011/a-recursive-algorithm-to-find-all-paths-between-two-given-nodes/
    fn get_paths(&mut self, visited: &mut VecDeque<Position>, end: u8) {
        // get last element
        let current = visited.back().expect("Empty");
        let val = self.map[self.coord(&current)];

        let neighbours = self.neighbours(&current);

        // check for paths
        for pos in &neighbours {
            if visited.contains(pos) {
                continue;
            }

            let neighbour_val = self.map[self.coord(&pos)];
            if neighbour_val == val + 1 {
                if neighbour_val == end {
                    visited.push_back(*pos);
                    let len = visited.len();
                    let hops = len - 1;

                    visited.make_contiguous();
                    let path = visited.as_slices().0;
                    self.paths.push(path.to_vec());

                    visited.remove(hops);
                    break;
                }
            }
        }

        // recurse
        for pos in neighbours {
            let neighbour_val = self.map[self.coord(&pos)];
            if visited.contains(&pos) || neighbour_val == end {
                continue;
            }
            if neighbour_val == val + 1 {
                visited.push_back(pos);
                self.get_paths(visited, end);

                let len = visited.len();
                if len > 0 {
                    visited.remove(len - 1);
                }
            }
        }
    }

    fn find_all_paths(&mut self) {
        let mut visited = VecDeque::new();
        for start in self.trailheads.clone()[..].iter() {
            visited.push_back(*start);
            self.get_paths(&mut visited, 9);
            // reset for each starting point
            visited.truncate(0);
        }
    }
}

struct Svg<'a> {
    tile_size: usize,
    offset: usize,
    stroke_width: usize,
    colour: &'a str,
    width: usize,
    height: usize,
    end_radius: usize,
}

impl<'a> Svg<'a> {
    fn new(
        tile_size: usize,
        offset: usize,
        map_width: usize,
        map_height: usize,
        stroke_width: usize,
        colour: &'a str,
        end_radius: usize,
    ) -> Self {
        Svg {
            tile_size,
            offset,
            stroke_width,
            colour,
            width: (map_width - 1) * tile_size + (offset * 2),
            height: (map_height - 1) * tile_size + (offset * 2),
            end_radius,
        }
    }

    // TODO  use path
    fn line(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> String {
        format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"{}\" stroke-linecap=\"square\" />\n",
            x1,
            y1,
            x2,
            y2,
            self.colour,
            self.stroke_width
        )
    }
    fn start(&self, centre: Position) -> String {
        format!("<circle cx=\"{}\" cy=\"{}\" stroke-width=\"{}\" fill=\"transparent\" stroke=\"{}\" r=\"{}\" />", centre.x * self.tile_size + self.offset, centre.y * self.tile_size + self.offset, self.stroke_width, self.colour, self.end_radius)
    }

    fn end(&self, centre: Position) -> String {
        format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" stroke-width=\"{}\" fill=\"transparent\" stroke=\"{}\" />", (centre.x * self.tile_size + self.offset) - self.end_radius, (centre.y * self.tile_size + self.offset) - self.end_radius, self.end_radius * 2, self.end_radius * 2, self.stroke_width, self.colour)
    }

    fn to_pixel(&self, point: usize) -> usize {
        point * self.tile_size + self.offset
    }

    fn draw(&self, map: &Map) -> String {
        let mut output = format!(
            "<svg viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">",
            self.width, self.height
        );

        map.paths.iter().for_each(|trail| {
            let mut index = 0;
            trail.windows(2).for_each(|slice| {
                if index == 0 {
                    output += &self.start(slice[0]);
                }
                // max index = 9, -1 because window
                if index == 8 {
                    output += &self.end(slice[1]);
                }

                let mut x1 = self.to_pixel(slice[0].x);
                let mut y1 = self.to_pixel(slice[0].y);
                let mut x2 = self.to_pixel(slice[1].x);
                let mut y2 = self.to_pixel(slice[1].y);

                // horz moving right
                if slice[0].x < slice[1].x {
                    if index == 0 {
                        x1 += self.end_radius;
                    } else if index == 8 {
                        x2 -= self.end_radius;
                    }
                }

                // horz moving left
                if slice[0].x > slice[1].x {
                    if index == 0 {
                        x1 -= self.end_radius;
                    } else if index == 8 {
                        x2 += self.end_radius;
                    }
                }

                // vert moving down
                if slice[0].y < slice[1].y {
                    if index == 0 {
                        y1 += self.end_radius;
                    } else if index == 8 {
                        y2 -= self.end_radius;
                    }
                }
                // vert moving up
                if slice[0].y > slice[1].y {
                    if index == 0 {
                        y1 -= self.end_radius;
                    } else if index == 8 {
                        y2 += self.end_radius;
                    }
                }
                output += &self.line(x1, y1, x2, y2);
                index += 1;
            });
        });
        output += "</svg>";

        output
    }
}

#[derive(Debug)]
struct Leaf {
    depth: usize,
    min_size: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    children: Vec<Leaf>,
}

impl Leaf {
    fn new(x: usize, y: usize, width: usize, height: usize, min_size: usize, depth: usize) -> Self {
        Leaf {
            depth,
            x,
            y,
            width,
            height,
            min_size,
            children: vec![],
        }
    }

    fn split(&mut self, rng: &mut ChaCha8Rng) -> bool {
        // if over size and is big enough to split
        // if depth < 2 always split otherwise it'll be boring
        // otherwise 75% chance to split

        // usually split
        let should_split = if self.depth > 1 {
            rng.gen_bool(0.75)
        } else {
            true
        };
        if should_split && self.width / 2 > self.min_size && self.height / 2 > self.min_size {
            let horz_mid = self.width / 2;
            let vert_mid = self.height / 2;
            let north_west = Leaf::new(
                self.x,
                self.y,
                horz_mid,
                vert_mid,
                self.min_size,
                self.depth + 1,
            );
            let north_east = Leaf::new(
                self.x + horz_mid,
                self.y,
                horz_mid,
                vert_mid,
                self.min_size,
                self.depth + 1,
            );
            let south_east = Leaf::new(
                self.x + horz_mid,
                self.y + vert_mid,
                horz_mid,
                vert_mid,
                self.min_size,
                self.depth + 1,
            );
            let south_west = Leaf::new(
                self.x,
                self.y + vert_mid,
                horz_mid,
                vert_mid,
                self.min_size,
                self.depth + 1,
            );

            self.children.push(north_west);
            self.children.push(north_east);
            self.children.push(south_east);
            self.children.push(south_west);

            true
        } else {
            false
        }

        // if width > 25% height, split vert
        // if height > 25% width, split horz
        // // otherwise random

        // let mut split_horz = rng.gen_bool(0.5);
        // if self.width > self.height && (self.width as f32 / self.height as f32) >= 1.25 {
        //     split_horz = false;
        // } else if self.height > self.width && (self.height as f32 / self.width as f32) >= 1.25 {
        //     split_horz = true;
        // }

        // let max = match split_horz {
        //     true => self.height - self.min_size,
        //     false => self.width - self.min_size,
        // };

        // // small enough so stop
        // if max <= self.min_size {
        //     return false;
        // }
        // let split_pos = rng.gen_range(self.min_size..max);

        // if split_horz {
        //     let left = Leaf::new(
        //         self.x,
        //         self.y,
        //         self.width,
        //         split_pos,
        //         self.min_size,
        //         self.depth + 1,
        //     );
        //     let right = Leaf::new(
        //         self.x,
        //         self.y + split_pos,
        //         self.width,
        //         self.height - split_pos,
        //         self.min_size,
        //         self.depth + 1,
        //     );
        //     self.children.push(left);
        //     self.children.push(right);
        // } else {
        //     let left = Leaf::new(
        //         self.x,
        //         self.y,
        //         split_pos,
        //         self.height,
        //         self.min_size,
        //         self.depth + 1,
        //     );
        //     let right = Leaf::new(
        //         self.x + split_pos,
        //         self.y,
        //         self.width - split_pos,
        //         self.height,
        //         self.min_size,
        //         self.depth + 1,
        //     );
        //     self.children.push(left);
        //     self.children.push(right);
        // }

        // true
    }

    fn can_split(&self) -> bool {
        self.children.len() == 0
    }

    fn generate(&mut self, rng: &mut ChaCha8Rng) {
        if self.can_split() {
            if self.split(rng) {
                self.children
                    .iter_mut()
                    .for_each(|child| child.generate(rng));
            }
        }
    }

    fn add_start(&self, starting_points: &mut HashSet<Position>, rng: &mut ChaCha8Rng) {
        if self.children.len() > 0 {
            self.children.iter().for_each(|child| {
                child.add_start(starting_points, rng);
            });
        } else {
            let x = rng.gen_range(self.x..self.x + self.width);
            let y = rng.gen_range(self.y..self.y + self.height);
            starting_points.insert(Position { x, y });
        }
    }
}

// enum Direction {
//     North,
//     East,
//     South,
//     West,
// }

// impl Distribution<Direction> for Standard {
//     fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
//         match rng.gen_range(0..=3) {
//             0 => Direction::North,
//             1 => Direction::East,
//             2 => Direction::South,
//             _ => Direction::West,
//         }
//     }
// }

#[derive(Debug)]
struct Input {
    map: Vec<char>,
    width: usize,
    height: usize,
    // starting_points: HashSet<Position>,
}

impl Input {
    fn new(width: usize, height: usize) -> Self {
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

    fn neighbours(&self, pos: &Position) -> Vec<Position> {
        let mut neighbours = vec![];

        if pos.y > 0 {
            let neighbour = Position {
                x: pos.x,
                y: pos.y - 1,
            };
            if self.map[self.coord(&neighbour)] == '.' {
                neighbours.push(neighbour);
            }
        }
        if pos.y < self.height - 1 {
            let neighbour = Position {
                x: pos.x,
                y: pos.y + 1,
            };
            if self.map[self.coord(&neighbour)] == '.' {
                neighbours.push(neighbour);
            }
        }

        if pos.x < self.width - 1 {
            let neighbour = Position {
                x: pos.x + 1,
                y: pos.y,
            };
            if self.map[self.coord(&neighbour)] == '.' {
                neighbours.push(neighbour);
            }
        }
        if pos.x > 0 {
            let neighbour = Position {
                x: pos.x - 1,
                y: pos.y,
            };
            if self.map[self.coord(&neighbour)] == '.' {
                neighbours.push(neighbour);
            }
        }
        neighbours
    }

    fn pick_direction(
        &self,
        start: &Position,
        rng: &mut ChaCha8Rng,
        mut tries: u8,
    ) -> Option<Position> {
        // TODO just get neighbours instead
        let options = self.neighbours(start);
        if options.len() == 0 {
            return None;
        }

        // tries += 1;

        // if tries > 4 {
        //     return None;
        // }

        match options.choose(rng) {
            None => return None,
            Some(pos) => {
                let current_val = self.map[(start.y * self.width) + start.x];
                let coord = (pos.y * self.width) + pos.x;
                let val = self.map[coord];

                // if val != '.' {
                //     return self.pick_direction(&start, rng, tries);
                // }

                Some(*pos)
            }
        }

        // let direction: Direction = rng.gen();

        // let pos = match direction {
        //     Direction::North => {
        //         if start.x == 0 {
        //             return self.pick_direction(start, rng, tries);
        //         }
        //         Position {
        //             x: start.x - 1,
        //             y: start.y,
        //         }
        //     }
        //     Direction::East => {
        //         if start.y == self.width {
        //             return self.pick_direction(start, rng, tries);
        //         }
        //         Position {
        //             x: start.x,
        //             y: start.y + 1,
        //         }
        //     }
        //     Direction::South => {
        //         if start.x == self.height {
        //             return self.pick_direction(start, rng, tries);
        //         }
        //         Position {
        //             x: start.x + 1,
        //             y: start.y,
        //         }
        //     }
        //     Direction::West => {
        //         if start.y == 0 {
        //             return self.pick_direction(start, rng, tries);
        //         }
        //         Position {
        //             x: start.x,
        //             y: start.y - 1,
        //         }
        //     }
        // };
    }

    fn random_walk(&mut self, starting_points: &HashSet<Position>, rng: &mut ChaCha8Rng) {
        // each start point
        // do a random walk to 9
        // if already set and not next num
        // pick another direction
        starting_points.iter().for_each(|start| {
            // let mut current = Position { x: 2, y: 4 };
            let mut current = *start;
            let current_coord = self.coord(&current);
            self.map[current_coord] = '0';

            for i in 0..9 {
                let pos = self.pick_direction(&current, rng, 0);
                dbg!(&pos);

                match pos {
                    None => return,
                    Some(pos) => {
                        let coord = (pos.y * self.width) + pos.x;
                        dbg!(i);
                        current = pos;

                        let val = self.map[coord];
                        if val == '.' {
                            dbg!(val);

                            // update map - Rust unhappy with multiple ref to self
                            let mut map = std::mem::take(&mut self.map);
                            map[coord] = char::from_digit(i as u32 + 1, 10)
                                .expect("Failed to convert to char");
                            self.map = map;
                        } else if val.to_digit(10).expect("Failed to parse current value")
                            != i as u32 + 1
                        {
                            dbg!("not adding");
                            return;
                        }
                    }
                }
            }
        })
    }

    fn fill(&mut self, rng: &mut ChaCha8Rng) {
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

fn main() -> Result<(), io::Error> {
    let seed = 14;
    let mut rng = ChaCha8Rng::seed_from_u64(seed);

    let width = 16;
    let height = 16;
    let mut root = Leaf::new(0, 0, width, height, 2, 0);
    root.generate(&mut rng);

    // println!("{:#?}", root);

    let mut input = Input::new(width, height);

    // let mut starting_points: Vec<Position> = vec![];
    let mut starting_points = HashSet::new();
    root.add_start(&mut starting_points, &mut rng);

    input.random_walk(&starting_points, &mut rng);
    input.fill(&mut rng);

    println!("{}", input);
    println!("{:?}", &starting_points);
    // for node in root
    // if leaf
    // pick a point in leaf
    // add to starting points

    // let args: Vec<String> = args().collect();
    //
    // let filename = &args[1];
    // let input = read_to_string(filename)?;
    // let mut map = Map::parse(&input);
    let mut map = Map::parse(&format!("{}", input));

    map.find_all_paths();

    // TODO gen input
    // quadtree
    // min size for 0-9 is 4x3
    // 6x6
    // 1-3 starts per node
    // TODO svg struct display

    let svg = Svg::new(64, 32, map.width, map.height, 2, "black", 10);
    let output = svg.draw(&map);

    write("./test.svg", output)?;

    Ok(())
}
