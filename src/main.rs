use std::{
    collections::{HashSet, VecDeque},
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

    // fn find_trail(&self) -> Vec<Vec<Position>> {
    //     let mut total = 0;
    //     let mut trails = vec![];

    //     for pos in self.trailheads[..].iter() {
    //         let mut visited = vec![];
    //         let mut count = 0;
    //         trails.push(self.search(&mut visited, pos, &mut count));
    //         total += count;
    //     }

    //     trails
    // }

    // how many '9's can be reached? (dfs)
    // fn search(
    //     &self,
    //     visited: &mut Vec<Position>,
    //     current: &Position,
    //     count: &mut usize,
    // ) -> Vec<Position> {
    //     let neighbours = self.neighbours(&current);
    //     let val = self.map[self.coord(current)];
    //     if self.map[self.coord(current)] == 9 {
    //         *count += 1;
    //     }

    //     visited.push(*current);

    //     for pos in neighbours {
    //         let neighbour_val = self.map[self.coord(&pos)];
    //         if neighbour_val == val + 1 && !visited.contains(&pos) {
    //             self.search(visited, &pos, count);
    //         }
    //     }

    //     visited.to_vec()
    // }

    fn rate_trails(&self) -> Vec<Vec<Position>> {
        let mut trails = vec![];

        for pos in self.trailheads[..].iter() {
            let mut another = Vec::new();
            let mut current_trail = Vec::new();
            current_trail.push(*pos);
            let mut visited = Vec::new();
            visited.push(*pos);

            let mut queue = VecDeque::new();
            queue.push_back(*pos);

            self.find_distinct(&mut visited, &mut queue, &mut another, &mut current_trail);
            // make all trails start from trailhead
            let expected_steps = 10;
            let mapped: Vec<Vec<Position>> = another
                .iter()
                .map(|t| {
                    // first one should always have correct no. steps
                    // if t.len() < expected_steps {
                    //     let beginning = &another[0][..(another[0].len() - t.len())];
                    //     let mut full_trail = beginning.to_vec();
                    //     full_trail.extend(t);
                    //     full_trail
                    // } else {
                    t.to_vec()
                    // }
                })
                .collect();

            trails.extend(mapped);
        }

        trails
    }

    // find distinct paths (bfs)
    fn find_distinct(
        &self,
        visited: &mut Vec<Position>,
        queue: &mut VecDeque<Position>,
        trails: &mut Vec<Vec<Position>>,
        current_trail: &mut Vec<Position>,
    ) {
        if queue.len() == 0 {
            dbg!("dead end", &current_trail);
            current_trail.truncate(0);
            return;
        }

        let current = queue.pop_front().expect("Failed to get current pos");
        let neighbours = self.neighbours(&current);
        let val = self.map[self.coord(&current)];

        // tODO truncate if don't get to 9
        if val == 9 {
            // push to trails
            trails.push(current_trail.to_vec());
        }

        for pos in neighbours {
            let neighbour_val = self.map[self.coord(&pos)];
            if neighbour_val == val + 1 {
                dbg!(current, val, pos, neighbour_val);
                // already full
                if current_trail.len() == 10 {
                    dbg!("current trail full", &current_trail);
                    // find last joining point
                    current_trail.truncate((val + 1).into());
                }

                queue.push_back(pos);
                visited.push(pos);
                current_trail.push(pos);
                self.find_distinct(visited, queue, trails, current_trail);
            }
        }
    }
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = args().collect();

    let filename = &args[1];
    let input = read_to_string(filename)?;
    let map = Map::parse(&input);

    println!("Rating: {:?}", map.rate_trails());
    // println!("Rating: {:?}", map.find_trail());
    let tile_size = 64;
    let offset = 16;
    let mut output =
        String::from("<svg viewBox=\"0 0 480 480\" xmlns=\"http://www.w3.org/2000/svg\">");
    map.rate_trails().iter().for_each(|trail| {
        trail.windows(2).for_each(|slice| {
            // map.rate_trails()[63].windows(2).for_each(|slice| {
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

    // println!("{}", output);
    write("./test.svg", output).expect("Couldn't write file");

    // dbg!(&map.rate_trails()[63]);

    Ok(())
}
