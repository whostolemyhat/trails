use crate::map::{Map, Position};

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

    // TODO  use path
    fn line(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> String {
        format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"{}\" stroke-linecap=\"square\" />\n",
            x1,
            y1,
            x2,
            y2,
            self.colour,
            self.stroke_width
        )
    }
    fn start(&self, centre: Position) -> String {
        format!("<circle cx=\"{}\" cy=\"{}\" stroke-width=\"{}\" fill=\"transparent\" stroke=\"{}\" r=\"{}\" />", centre.x * self.tile_size + self.offset, centre.y * self.tile_size + self.offset, self.stroke_width, self.colour, self.end_radius)
    }

    fn end(&self, centre: Position) -> String {
        format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" stroke-width=\"{}\" fill=\"transparent\" stroke=\"{}\" />", (centre.x * self.tile_size + self.offset) - self.end_radius, (centre.y * self.tile_size + self.offset) - self.end_radius, self.end_radius * 2, self.end_radius * 2, self.stroke_width, self.colour)
    }

    fn to_pixel(&self, point: usize) -> usize {
        point * self.tile_size + self.offset
    }

    pub fn draw(&self, map: &Map) -> String {
        let mut output = format!(
            "<svg viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">",
            self.width, self.height
        );

        map.paths.iter().for_each(|trail| {
            let mut index = 0;
            trail.windows(2).for_each(|slice| {
                if index == 0 {
                    output += &self.start(slice[0]);
                }
                // max index = 9, -1 because window
                if index == 8 {
                    output += &self.end(slice[1]);
                }

                let mut x1 = self.to_pixel(slice[0].x);
                let mut y1 = self.to_pixel(slice[0].y);
                let mut x2 = self.to_pixel(slice[1].x);
                let mut y2 = self.to_pixel(slice[1].y);

                // horz moving right
                if slice[0].x < slice[1].x {
                    if index == 0 {
                        x1 += self.end_radius;
                    } else if index == 8 {
                        x2 -= self.end_radius;
                    }
                }

                // horz moving left
                if slice[0].x > slice[1].x {
                    if index == 0 {
                        x1 -= self.end_radius;
                    } else if index == 8 {
                        x2 += self.end_radius;
                    }
                }

                // vert moving down
                if slice[0].y < slice[1].y {
                    if index == 0 {
                        y1 += self.end_radius;
                    } else if index == 8 {
                        y2 -= self.end_radius;
                    }
                }
                // vert moving up
                if slice[0].y > slice[1].y {
                    if index == 0 {
                        y1 -= self.end_radius;
                    } else if index == 8 {
                        y2 += self.end_radius;
                    }
                }
                output += &self.line(x1, y1, x2, y2);
                index += 1;
            });
        });
        output += "</svg>";

        output
    }
}

#[cfg(test)]
mod test {
    use crate::{map::Map, svg::Svg};

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

        assert_eq!(output, "<svg viewBox=\"0 0 640 640\" xmlns=\"http://www.w3.org/2000/svg\"><circle cx=\"160\" cy=\"288\" stroke-width=\"2\" fill=\"transparent\" stroke=\"black\" r=\"10\" /><line x1=\"160\" y1=\"298\" x2=\"160\" y2=\"352\" stroke=\"black\" stroke-width=\"2\" stroke-linecap=\"square\" />\n<line x1=\"160\" y1=\"352\" x2=\"160\" y2=\"416\" stroke=\"black\" stroke-width=\"2\" stroke-linecap=\"square\" />\n<line x1=\"160\" y1=\"416\" x2=\"160\" y2=\"480\" stroke=\"black\" stroke-width=\"2\" stroke-linecap=\"square\" />\n<line x1=\"160\" y1=\"480\" x2=\"96\" y2=\"480\" stroke=\"black\" stroke-width=\"2\" stroke-linecap=\"square\" />\n<line x1=\"96\" y1=\"480\" x2=\"96\" y2=\"544\" stroke=\"black\" stroke-width=\"2\" stroke-linecap=\"square\" />\n<line x1=\"96\" y1=\"544\" x2=\"32\" y2=\"544\" stroke=\"black\" stroke-width=\"2\" stroke-linecap=\"square\" />\n<line x1=\"32\" y1=\"544\" x2=\"32\" y2=\"480\" stroke=\"black\" stroke-width=\"2\" stroke-linecap=\"square\" />\n<line x1=\"32\" y1=\"480\" x2=\"32\" y2=\"416\" stroke=\"black\" stroke-width=\"2\" stroke-linecap=\"square\" />\n<rect x=\"22\" y=\"342\" width=\"20\" height=\"20\" stroke-width=\"2\" fill=\"transparent\" stroke=\"black\" /><line x1=\"32\" y1=\"416\" x2=\"32\" y2=\"362\" stroke=\"black\" stroke-width=\"2\" stroke-linecap=\"square\" />\n<circle cx=\"352\" cy=\"352\" stroke-width=\"2\" fill=\"transparent\" stroke=\"black\" r=\"10\" /><line x1=\"352\" y1=\"342\" x2=\"352\" y2=\"288\" stroke=\"black\" stroke-width=\"2\" stroke-linecap=\"square\" />\n<line x1=\"352\" y1=\"288\" x2=\"352\" y2=\"224\" stroke=\"black\" stroke-width=\"2\" stroke-linecap=\"square\" />\n<line x1=\"352\" y1=\"224\" x2=\"416\" y2=\"224\" stroke=\"black\" stroke-width=\"2\" stroke-linecap=\"square\" />\n<line x1=\"416\" y1=\"224\" x2=\"480\" y2=\"224\" stroke=\"black\" stroke-width=\"2\" stroke-linecap=\"square\" />\n<line x1=\"480\" y1=\"224\" x2=\"544\" y2=\"224\" stroke=\"black\" stroke-width=\"2\" stroke-linecap=\"square\" />\n<line x1=\"544\" y1=\"224\" x2=\"544\" y2=\"288\" stroke=\"black\" stroke-width=\"2\" stroke-linecap=\"square\" />\n<line x1=\"544\" y1=\"288\" x2=\"544\" y2=\"352\" stroke=\"black\" stroke-width=\"2\" stroke-linecap=\"square\" />\n<line x1=\"544\" y1=\"352\" x2=\"544\" y2=\"416\" stroke=\"black\" stroke-width=\"2\" stroke-linecap=\"square\" />\n<rect x=\"598\" y=\"406\" width=\"20\" height=\"20\" stroke-width=\"2\" fill=\"transparent\" stroke=\"black\" /><line x1=\"544\" y1=\"416\" x2=\"598\" y2=\"416\" stroke=\"black\" stroke-width=\"2\" stroke-linecap=\"square\" />\n</svg>");
    }
}
