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

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = args().collect();

    let filename = &args[1];
    let input = read_to_string(filename)?;
    let mut map = Map::parse(&input);

    map.find_all_paths();

    // TODO start and end
    // TODO width of stroke
    // TODO size of output
    // TODO gen input

    let tile_size = 64;
    let offset = 16;
    let mut output =
        String::from("<svg viewBox=\"0 0 2880 2880\" xmlns=\"http://www.w3.org/2000/svg\">");

    map.paths.iter().for_each(|trail| {
        trail.windows(2).for_each(|slice| {
            output += &*format!(
                "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"black\" />\n",
                slice[0].x * tile_size + offset,
                slice[0].y * tile_size + offset,
                slice[1].x * tile_size + offset,
                slice[1].y * tile_size + offset
            );
        });
    });
    output += "</svg>";

    write("./test.svg", output).expect("Couldn't write file");

    Ok(())
}
