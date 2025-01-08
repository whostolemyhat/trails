#[cfg(test)]
mod test {
    use crate::{Map, Position};

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
