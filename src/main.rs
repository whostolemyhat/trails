use std::{
    collections::HashSet,
    env::args,
    fs::{read_to_string, write},
    io,
};

use input::Input;
use map::Map;
use quadtree::Leaf;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use svg::Svg;

mod input;
mod map;
mod quadtree;
mod svg;
mod test;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
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

fn main() -> Result<(), io::Error> {
    let seed = 1414212;
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let num = rng.gen_range(0..100);
    println!("{:?}", num);

    let width = 8;
    let height = 8;
    let mut root = Leaf::new(0, 0, width, height, 3, 0);
    root.generate(&mut rng);

    let mut input = Input::new(width, height);

    // let mut starting_points: Vec<Position> = vec![];
    let mut starting_points = HashSet::new();
    root.add_start(&mut starting_points, &mut rng);

    // dbg!(&root);
    let debug_quad = root.draw();
    write("./debug_quad.svg", debug_quad)?;

    input.add_trails(&starting_points, &mut rng);
    // input.random_walk(&starting_points, &mut rng);
    input.fill(&mut rng);

    println!("{}", input);
    println!("{:?}", &starting_points.len());
    // for node in root
    // if leaf
    // pick a point in leaf
    // add to starting points

    // let args: Vec<String> = args().collect();
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
