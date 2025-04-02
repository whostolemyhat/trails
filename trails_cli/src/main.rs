use clap::{Parser, Subcommand};
use std::{
    fs::{read_to_string, write},
    io,
};

use trails::{map::Map, svg::Svg};

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
            let output = trails::create(
                &args.seed,
                args.canvas_size,
                args.min_leaf_size,
                args.density,
            );

            write(format!("./trail-{}.svg", args.seed), output)?;

            Ok(())
        }
        Commands::FromFile(args) => {
            let filename = &args.name;
            let input = read_to_string(filename)?;
            let mut map = Map::parse(&input);

            map.find_all_paths();

            let svg = Svg::new(64, 32, map.width, map.height, 2, "black", 10);
            let output = svg.draw(&map);

            write("./trail.svg", output)?;
            Ok(())
        }
    }
}
