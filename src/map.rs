use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, PartialEq)]
pub struct Map {
    pub map: Vec<u8>,
    pub width: usize,
    pub height: usize,
    pub trailheads: Vec<Position>,
    pub paths: Vec<Vec<Position>>,
}
impl Map {
    pub fn parse(input: &str) -> Self {
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
        let val = self.map[self.coord(current)];

        let neighbours = self.neighbours(current);

        // check for paths
        for pos in &neighbours {
            if visited.contains(pos) {
                continue;
            }

            let neighbour_val = self.map[self.coord(pos)];
            if neighbour_val == val + 1 && neighbour_val == end {
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

    pub fn find_all_paths(&mut self) {
        let mut visited = VecDeque::new();
        for start in self.trailheads.clone()[..].iter() {
            visited.push_back(*start);
            self.get_paths(&mut visited, 9);
            // reset for each starting point
            visited.truncate(0);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::map::{Map, Position};
    #[test]
    fn it_should_parse_text_input() {
        let input = "0123
          1234
          8765
          9876";
        let map = Map::parse(&input);

        assert_eq!(
            map,
            Map {
                width: 4,
                height: 4,
                paths: vec![],
                trailheads: vec![Position { x: 0, y: 0 }],
                map: vec![0, 1, 2, 3, 1, 2, 3, 4, 8, 7, 6, 5, 9, 8, 7, 6],
            }
        );
    }

    #[test]
    fn it_should_find_trailheads() {
        let input = "0123
          1204
          8760
          9076";
        let map = Map::parse(&input);

        assert_eq!(
            map,
            Map {
                width: 4,
                height: 4,
                paths: vec![],
                trailheads: vec![
                    Position { x: 0, y: 0 },
                    Position { x: 2, y: 1 },
                    Position { x: 3, y: 2 },
                    Position { x: 1, y: 3 }
                ],
                map: vec![0, 1, 2, 3, 1, 2, 0, 4, 8, 7, 6, 0, 9, 0, 7, 6],
            }
        );

        let input = "1123
          1214
          8761
          9176";
        let map = Map::parse(&input);

        assert_eq!(
            map,
            Map {
                width: 4,
                height: 4,
                paths: vec![],
                trailheads: vec![],
                map: vec![1, 1, 2, 3, 1, 2, 1, 4, 8, 7, 6, 1, 9, 1, 7, 6],
            }
        );
    }

    #[test]
    fn it_should_map_position_to_arr_index() {
        let input = "0123
          1234
          8765
          9876";
        let map = Map::parse(&input);
        assert_eq!(map.coord(&Position { x: 0, y: 0 }), 0);
        assert_eq!(map.coord(&Position { x: 0, y: 1 }), 4);
        assert_eq!(map.coord(&Position { x: 3, y: 3 }), 15);
        assert_eq!(map.coord(&Position { x: 1, y: 2 }), 9);
    }

    #[test]
    fn it_should_find_cardinal_neighbours() {
        let input = "0123
          1234
          8765
          9876";
        let map = Map::parse(&input);

        assert_eq!(
            map.neighbours(&Position { x: 1, y: 1 }),
            vec![
                Position { x: 1, y: 0 },
                Position { x: 1, y: 2 },
                Position { x: 2, y: 1 },
                Position { x: 0, y: 1 }
            ]
        );

        assert_eq!(
            map.neighbours(&Position { x: 4, y: 4 }),
            vec![Position { x: 4, y: 3 }, Position { x: 3, y: 4 }]
        );
        assert_eq!(
            map.neighbours(&Position { x: 2, y: 0 }),
            vec![
                Position { x: 2, y: 1 },
                Position { x: 3, y: 0 },
                Position { x: 1, y: 0 }
            ]
        );
    }

    #[test]
    fn it_should_find_all_paths() {
        let input = "890
          781
          874
          965
          456
          320
          013
          104";
        let mut map = Map::parse(&input);
        map.find_all_paths();
        let expected: Vec<Vec<Position>> = vec![
            vec![
                Position { x: 0, y: 6 },
                Position { x: 1, y: 6 },
                Position { x: 1, y: 5 },
                Position { x: 0, y: 5 },
                Position { x: 0, y: 4 },
                Position { x: 1, y: 4 },
                Position { x: 1, y: 3 },
                Position { x: 1, y: 2 },
                Position { x: 1, y: 1 },
                Position { x: 1, y: 0 },
            ],
            vec![
                Position { x: 0, y: 6 },
                Position { x: 1, y: 6 },
                Position { x: 1, y: 5 },
                Position { x: 0, y: 5 },
                Position { x: 0, y: 4 },
                Position { x: 1, y: 4 },
                Position { x: 1, y: 3 },
                Position { x: 1, y: 2 },
                Position { x: 0, y: 2 },
                Position { x: 0, y: 3 },
            ],
            vec![
                Position { x: 1, y: 7 },
                Position { x: 1, y: 6 },
                Position { x: 1, y: 5 },
                Position { x: 0, y: 5 },
                Position { x: 0, y: 4 },
                Position { x: 1, y: 4 },
                Position { x: 1, y: 3 },
                Position { x: 1, y: 2 },
                Position { x: 1, y: 1 },
                Position { x: 1, y: 0 },
            ],
            vec![
                Position { x: 1, y: 7 },
                Position { x: 1, y: 6 },
                Position { x: 1, y: 5 },
                Position { x: 0, y: 5 },
                Position { x: 0, y: 4 },
                Position { x: 1, y: 4 },
                Position { x: 1, y: 3 },
                Position { x: 1, y: 2 },
                Position { x: 0, y: 2 },
                Position { x: 0, y: 3 },
            ],
        ];
        assert_eq!(map.paths, expected);
    }
}
