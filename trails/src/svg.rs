use std::fmt::Display;

use crate::map::{Map, Position};

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum LineCommand {
    Horizontal,
    Vertical,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct SvgCommand {
    command: LineCommand,
    distance: i16,
}

impl Display for SvgCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.command {
            LineCommand::Horizontal => {
                write!(f, "h{}", self.distance).expect("Failed to write command");
            }
            LineCommand::Vertical => {
                write!(f, "v{}", self.distance).expect("Failed to write command");
            }
        }
        Ok(())
    }
}

#[derive(PartialEq, Debug)]
pub struct Svg<'a> {
    pub tile_size: usize,
    pub offset: usize,
    pub stroke_width: usize,
    pub colour: &'a str,
    pub width: usize,
    pub height: usize,
    pub end_radius: usize,
}

impl<'a> Svg<'a> {
    pub fn new(
        tile_size: usize,
        offset: usize,
        map_width: usize,
        map_height: usize,
        stroke_width: usize,
        colour: &'a str,
        end_radius: usize,
    ) -> Self {
        Svg {
            tile_size,
            offset,
            stroke_width,
            colour,
            width: (map_width - 1) * tile_size + (offset * 2),
            height: (map_height - 1) * tile_size + (offset * 2),
            end_radius,
        }
    }

    fn draw_path(&self, path: &str) -> String {
        format!(
            "<path d=\"{path}\" fill=\"none\" stroke=\"{}\" stroke-width=\"{}\" stroke-linecap=\"square\" />",
            self.colour, self.stroke_width
        )
    }

    fn get_path(&self, points: &Vec<Position>) -> Vec<SvgCommand> {
        let commands = points
            .windows(2)
            .map(|slice| self.make_command(slice[0], slice[1]))
            .collect();
        commands
    }

    fn make_command(&self, first: Position, second: Position) -> SvgCommand {
        let is_horz = first.x != second.x;
        if is_horz {
            return SvgCommand {
                command: LineCommand::Horizontal,
                distance: self.to_pixel(second.x) as i16 - self.to_pixel(first.x) as i16,
            };
        } else {
            return SvgCommand {
                command: LineCommand::Vertical,
                distance: self.to_pixel(second.y) as i16 - self.to_pixel(first.y) as i16,
            };
        }
    }

    /// Combine consecutive duplicate commands
    /// eg v64v64h16v32 -> v128h16v32
    fn merge_commands(&self, path: Vec<SvgCommand>) -> Vec<SvgCommand> {
        let mut output: Vec<SvgCommand> = vec![];

        for (i, item) in path.iter().enumerate() {
            if i == 0 {
                output.push(item.clone());
            } else if item.command == output[output.len() - 1].command {
                let stored_index = output.len() - 1;
                output[stored_index].distance += item.distance;
            } else {
                output.push(item.clone());
            }
        }

        output
    }

    fn start(&self, centre: Position) -> String {
        format!(
            "<circle cx=\"{}\" cy=\"{}\" stroke-width=\"{}\" fill=\"transparent\" stroke=\"{}\" r=\"{}\" />",
            centre.x * self.tile_size + self.offset,
            centre.y * self.tile_size + self.offset,
            self.stroke_width,
            self.colour,
            self.end_radius
        )
    }

    fn end(&self, centre: Position) -> String {
        format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" stroke-width=\"{}\" fill=\"transparent\" stroke=\"{}\" />",
            (centre.x * self.tile_size + self.offset) - self.end_radius,
            (centre.y * self.tile_size + self.offset) - self.end_radius,
            self.end_radius * 2,
            self.end_radius * 2,
            self.stroke_width,
            self.colour
        )
    }

    fn to_pixel(&self, point: usize) -> usize {
        point * self.tile_size + self.offset
    }

    fn get_direction(&self, first: Position, second: Position) -> Direction {
        if first.x == second.x {
            // if x's are equal, moving vert
            if first.y < second.y {
                Direction::South
            } else {
                Direction::North
            }
        } else if first.x < second.x {
            Direction::West
        } else {
            Direction::East
        }
    }

    pub fn draw(&self, map: &Map) -> String {
        let mut output = format!(
            "<svg viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">",
            self.width, self.height
        );

        map.paths.iter().for_each(|trail| {
            let start_dir = self.get_direction(trail[0], trail[1]);
            let end_dir = self.get_direction(trail[trail.len() - 2], trail[trail.len() - 1]);

            let mut path_cmds = self.get_path(&trail);

            // adjust end of trail for rect
            // -2: len - 1 for last item, and there should be one fewer edges than nodes
            let last_index = path_cmds.len() - 1;
            match end_dir {
                Direction::North => path_cmds[last_index].distance += self.end_radius as i16,
                Direction::South => path_cmds[last_index].distance -= self.end_radius as i16,
                Direction::East => path_cmds[last_index].distance += self.end_radius as i16,
                Direction::West => path_cmds[last_index].distance -= self.end_radius as i16,
            };

            let mut start_x = self.to_pixel(trail[0].x);
            let mut start_y = self.to_pixel(trail[0].y);

            match start_dir {
                Direction::North => {
                    start_y -= self.end_radius;
                    path_cmds[0].distance += self.end_radius as i16;
                }
                Direction::South => {
                    start_y += self.end_radius;
                    path_cmds[0].distance -= self.end_radius as i16;
                }
                Direction::East => {
                    start_x -= self.end_radius;
                    path_cmds[0].distance += self.end_radius as i16;
                }
                Direction::West => {
                    start_x += self.end_radius;
                    path_cmds[0].distance -= self.end_radius as i16;
                }
            };

            // add start circle
            output += &self.start(Position {
                x: trail[0].x,
                y: trail[0].y,
            });

            // add end rect
            output += &self.end(trail[trail.len() - 1]);

            // add start move
            let mut merged = format!("M{},{}", start_x, start_y);

            // squash cmds
            for cmd in self.merge_commands(path_cmds) {
                merged += &format!("{}", cmd);
            }
            output += &format!("{}", self.draw_path(&merged));
        });
        output += "</svg>";

        output
    }
}

#[cfg(test)]
mod test {
    use crate::{
        map::{Map, Position},
        svg::{LineCommand, Svg, SvgCommand},
    };

    #[test]
    fn it_should_create_struct() {
        assert_eq!(
            Svg::new(16, 10, 16, 16, 2, "black", 3),
            Svg {
                tile_size: 16,
                offset: 10,
                stroke_width: 2,
                colour: "black",
                width: 260,
                height: 260,
                end_radius: 3
            }
        )
    }

    #[test]
    fn it_should_draw_map() {
        let input = "0010195998
9523236072
8774548688
7465623458
8806717067
9517809170
8726583389
7433376641
6596526545
6010449363";
        let mut map = Map::parse(input);
        map.find_all_paths();

        let svg = Svg::new(64, 32, 10, 10, 2, "black", 10);
        let output = svg.draw(&map);

        assert_eq!(
            output,
            "<svg viewBox=\"0 0 640 640\" xmlns=\"http://www.w3.org/2000/svg\"><circle cx=\"160\" cy=\"288\" stroke-width=\"2\" fill=\"transparent\" stroke=\"black\" r=\"10\" /><rect x=\"22\" y=\"342\" width=\"20\" height=\"20\" stroke-width=\"2\" fill=\"transparent\" stroke=\"black\" /><path d=\"M160,298v182h-64v64h-64v-182\" fill=\"none\" stroke=\"black\" stroke-width=\"2\" stroke-linecap=\"square\" /><circle cx=\"352\" cy=\"352\" stroke-width=\"2\" fill=\"transparent\" stroke=\"black\" r=\"10\" /><rect x=\"598\" y=\"406\" width=\"20\" height=\"20\" stroke-width=\"2\" fill=\"transparent\" stroke=\"black\" /><path d=\"M352,342v-118h192v192h54\" fill=\"none\" stroke=\"black\" stroke-width=\"2\" stroke-linecap=\"square\" /></svg>"
        );
    }

    #[test]
    fn it_should_create_path_commands() {
        let trail = vec![
            Position { x: 4, y: 4 },
            Position { x: 4, y: 5 },
            Position { x: 5, y: 5 },
            Position { x: 6, y: 5 },
            Position { x: 7, y: 5 },
            Position { x: 8, y: 5 },
            Position { x: 8, y: 4 },
            Position { x: 8, y: 3 },
            Position { x: 7, y: 3 },
            Position { x: 6, y: 3 },
        ];

        let svg = Svg::new(64, 32, 10, 10, 2, "black", 10);
        let expected = vec![
            SvgCommand {
                command: LineCommand::Vertical,
                distance: 64,
            },
            SvgCommand {
                command: LineCommand::Horizontal,
                distance: 64,
            },
            SvgCommand {
                command: LineCommand::Horizontal,
                distance: 64,
            },
            SvgCommand {
                command: LineCommand::Horizontal,
                distance: 64,
            },
            SvgCommand {
                command: LineCommand::Horizontal,
                distance: 64,
            },
            SvgCommand {
                command: LineCommand::Vertical,
                distance: -64,
            },
            SvgCommand {
                command: LineCommand::Vertical,
                distance: -64,
            },
            SvgCommand {
                command: LineCommand::Horizontal,
                distance: -64,
            },
            SvgCommand {
                command: LineCommand::Horizontal,
                distance: -64,
            },
        ];
        assert_eq!(svg.get_path(&trail), expected);
    }

    #[test]
    fn it_merges_duplicate_commands() {
        let path = vec![
            SvgCommand {
                command: LineCommand::Horizontal,
                distance: 54,
            },
            SvgCommand {
                command: LineCommand::Vertical,
                distance: 64,
            },
            SvgCommand {
                command: LineCommand::Horizontal,
                distance: -64,
            },
            SvgCommand {
                command: LineCommand::Vertical,
                distance: 64,
            },
            SvgCommand {
                command: LineCommand::Vertical,
                distance: 64,
            },
            SvgCommand {
                command: LineCommand::Horizontal,
                distance: -64,
            },
            SvgCommand {
                command: LineCommand::Horizontal,
                distance: -64,
            },
            SvgCommand {
                command: LineCommand::Vertical,
                distance: 64,
            },
            SvgCommand {
                command: LineCommand::Vertical,
                distance: 54,
            },
        ];
        let expected = vec![
            SvgCommand {
                command: LineCommand::Horizontal,
                distance: 54,
            },
            SvgCommand {
                command: LineCommand::Vertical,
                distance: 64,
            },
            SvgCommand {
                command: LineCommand::Horizontal,
                distance: -64,
            },
            SvgCommand {
                command: LineCommand::Vertical,
                distance: 128,
            },
            SvgCommand {
                command: LineCommand::Horizontal,
                distance: -128,
            },
            SvgCommand {
                command: LineCommand::Vertical,
                distance: 118,
            },
        ];

        let svg = Svg::new(64, 32, 10, 10, 2, "black", 10);
        assert_eq!(svg.merge_commands(path), expected);
    }
}
