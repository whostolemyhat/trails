#[cfg(test)]
mod test {
    use crate::{Map, Position};

    // #[test]
    // fn it_should_count_trails() {
    //     let input = "0123
    //       1234
    //       8765
    //       9876";
    //     let map = Map::parse(&input);

    //     assert_eq!(1, map.find_trail());
    // }

    // #[test]
    // fn it_should_score_trails() {
    //     let input = "9990999
    //                   9991999
    //                   9992999
    //                   6543456
    //                   7111117
    //                   8111118
    //                   9111119";
    //     let map = Map::parse(&input);
    //     assert_eq!(2, map.find_trail());

    //     let input = "1066966
    //                   2666866
    //                   3111711
    //                   4567654
    //                   1118663
    //                   1119662
    //                   1116601";
    //     let map = Map::parse(&input);
    //     assert_eq!(3, map.find_trail());
    // }

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

    mod part_two {
        use crate::Map;

        #[test]
        fn it_should_rate_trails() {
            let input = "9999909
                         9943219
                         9959929
                         9965439
                         9979949
                         1187651
                         1191111";
            let map = Map::parse(input);
            // assert_eq!(3, map.rate_trails());

            let input = "89010123
                        78121874
                        87430965
                        96549874
                        45678903
                        32019012
                        01329801
                        10456732";
            let map = Map::parse(input);
            assert_eq!(81, map.rate_trails());

            let input = "012345
                         123456
                         234567
                         345678
                         416789
                         567891";
            let map = Map::parse(input);
            assert_eq!(227, map.rate_trails());
        }
    }
}
