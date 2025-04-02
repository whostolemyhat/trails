# trails

A Rust project which creates procedural art, inspired by Advent of Code. [Use the web app here](https://trails.jamesbaum.co.uk) or [read about it here](https://www.jamesbaum.co.uk/blether/procedural-art-with-rust).

![](./example.svg)

## Usage

Usage: trails_cli <COMMAND>

or from workspace root `cargo run -- generate -s ...`

```
Commands:
  generate   Create new trail image
  from-file  Read input map from file
  help       Print this message or the help of the given subcommand(s)
```

### Generate
Create new trail image

Usage: trails_cli generate [OPTIONS] --seed <SEED>

```
Options:
  -s, --seed <SEED>                    
  -m, --min-leaf-size <MIN_LEAF_SIZE>  [default: 3]
  -c, --canvas-size <CANVAS_SIZE>      [default: 45]
  -d, --density <DENSITY>              [default: 2]
  -h, --help                           Print help
```


### from-file

Read input map from file. Generate creates a dynamic input map, so this command just runs the drawing commands.

Usage: trails_cli from-file --name <NAME>

```
Options:
  -n, --name <NAME>  
  -h, --help         Print help
```