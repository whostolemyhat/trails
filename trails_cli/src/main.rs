use clap::{Parser, Subcommand};
use rand::rngs::SmallRng;
use rand_seeder::Seeder;
use std::{
    fs::{read_to_string, write},
    io,
};

use trails::{input::Input, map::Map, quadtree::Leaf, svg::Svg};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create new trail image
    Generate(GenerateArgs),
    /// Read input map from file
    FromFile(FromFileArgs),
}

#[derive(Parser)]
struct GenerateArgs {
    #[arg(short, long)]
    seed: String,
    #[arg(short, long, default_value_t = 3)]
    min_leaf_size: usize,
    #[arg(short, long, default_value_t = 45)]
    canvas_size: usize,
    input: Option<String>,
    #[arg(short, long, default_value_t = 2)]
    density: u8,
}

#[derive(Parser)]
struct FromFileArgs {
    #[arg(short, long)]
    name: String,
}

fn main() -> Result<(), io::Error> {
    let args = Args::parse();

    match &args.command {
        Commands::Generate(args) => {
            // let mut rng: SmallRng = Seeder::from(&args.seed).into_rng();

            // let width = args.canvas_size;
            // let height = args.canvas_size;
            // let depth = 0;
            // let mut root = Leaf::new(0, 0, width, height, args.min_leaf_size, depth);
            // root.generate(&mut rng);

            // let mut input = Input::new(width, height);
            // let mut starting_points = vec![];
            // root.add_start(&mut starting_points, &mut rng, args.density);

            // input.add_trails(&starting_points, &mut rng);
            // input.fill(&mut rng);

            // println!("{}", input);
            // println!("{:?}", &starting_points.len());

            // let mut map = Map::parse(&format!("{}", input));
            // map.find_all_paths();

            // let svg = Svg::new(64, 32, map.width, map.height, 2, "black", 10);
            // let output = svg.draw(&map);
            let output = trails::create(
                &args.seed,
                args.canvas_size,
                args.min_leaf_size,
                args.density,
            );

            write("./test.svg", output)?;

            Ok(())
        }
        Commands::FromFile(args) => {
            let filename = &args.name;
            let input = read_to_string(filename)?;
            let mut map = Map::parse(&input);

            map.find_all_paths();

            let svg = Svg::new(64, 32, map.width, map.height, 2, "black", 10);
            let output = svg.draw(&map);

            write("./test.svg", output)?;
            Ok(())
        }
    }
}
