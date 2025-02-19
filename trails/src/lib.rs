use input::Input;
use map::Map;
use quadtree::Leaf;
use rand::rngs::SmallRng;
use rand_seeder::Seeder;
use svg::Svg;

pub mod input;
pub mod map;
pub mod quadtree;
pub mod svg;

pub fn create(seed: &String, canvas_size: usize, min_leaf_size: usize, density: u8) -> String {
    let mut rng: SmallRng = Seeder::from(&seed).into_rng();

    let width = canvas_size;
    let height = canvas_size;
    let depth = 0;
    let mut root = Leaf::new(0, 0, width, height, min_leaf_size, depth);
    root.generate(&mut rng);

    let mut input = Input::new(width, height);
    let mut starting_points = vec![];
    root.add_start(&mut starting_points, &mut rng, density);

    input.add_trails(&starting_points, &mut rng);
    input.fill(&mut rng);

    let mut map = Map::parse(&format!("{}", input));
    map.find_all_paths();

    let svg = Svg::new(64, 32, map.width, map.height, 2, "black", 10);
    svg.draw(&map)
}
