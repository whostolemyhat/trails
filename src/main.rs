use std::{
    collections::VecDeque,
    env::args,
    fs::{read_to_string, write},
    io,
};

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
}

impl<'a> Svg<'a> {
    fn new(
        tile_size: usize,
        offset: usize,
        map_width: usize,
        map_height: usize,
        stroke_width: usize,
        colour: &'a str,
    ) -> Self {
        Svg {
            tile_size,
            offset,
            stroke_width,
            colour,
            width: (map_width - 1) * tile_size + (offset * 2),
            height: (map_height - 1) * tile_size + (offset * 2),
        }
    }

    // TODO  use path
    fn line(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> String {
        format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"{}\" />\n",
            x1 * self.tile_size + self.offset,
            y1 * self.tile_size + self.offset,
            x2 * self.tile_size + self.offset,
            y2 * self.tile_size + self.offset,
            self.colour,
            self.stroke_width
        )
    }
    fn start(&self, centre: Position) -> String {
        let radius = 10;
        format!("<circle cx=\"{}\" cy=\"{}\" stroke-width=\"{}\" fill=\"transparent\" stroke=\"{}\" r=\"{}\" />", centre.x * self.tile_size + self.offset, centre.y * self.tile_size + self.offset, self.stroke_width, self.colour, radius)
    }

    fn end(&self, centre: Position) -> String {
        let size = 20;
        format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" stroke-width=\"{}\" fill=\"transparent\" stroke=\"{}\" />", (centre.x * self.tile_size + self.offset) - (size / 2), (centre.y * self.tile_size + self.offset) - (size / 2), size, size, self.stroke_width, self.colour)
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
                output += &self.line(slice[0].x, slice[0].y, slice[1].x, slice[1].y);
                index += 1;
            });
        });
        output += "</svg>";

        output
    }
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = args().collect();

    let filename = &args[1];
    let input = read_to_string(filename)?;
    let mut map = Map::parse(&input);

    map.find_all_paths();

    // TODO gen input
    // TODO svg struct display

    let svg = Svg::new(64, 32, map.width, map.height, 2, "black");
    let output = svg.draw(&map);

    write("./test.svg", output)?;

    Ok(())
}
