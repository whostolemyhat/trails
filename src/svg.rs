use crate::{map::Map, Position};

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
