// args
// seed
// points per leaf
// leaf size
// canvas size
// generate or use an input file

use std::{fs::write, io};

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use trails::{input::Input, map::Map, quadtree::Leaf, svg::Svg};

fn main() -> Result<(), io::Error> {
    let seed = 1414212;
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let num = rng.gen_range(0..100);
    println!("{:?}", num);

    let width = 45;
    let height = 45;
    let mut root = Leaf::new(0, 0, width, height, 3, 0);
    root.generate(&mut rng);

    let mut input = Input::new(width, height);
    let mut starting_points = vec![];
    root.add_start(&mut starting_points, &mut rng);

    input.add_trails(&starting_points, &mut rng);
    input.fill(&mut rng);

    println!("{}", input);
    println!("{:?}", &starting_points.len());

    // let args: Vec<String> = args().collect();
    // let filename = &args[1];
    // let input = read_to_string(filename)?;
    // let mut map = Map::parse(&input);

    let mut map = Map::parse(&format!("{}", input));
    map.find_all_paths();

    let svg = Svg::new(64, 32, map.width, map.height, 2, "black", 10);
    let output = svg.draw(&map);

    write("./test.svg", output)?;

    Ok(())
}
