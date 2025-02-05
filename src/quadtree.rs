use std::collections::HashSet;

use rand::Rng;
use rand_chacha::ChaCha8Rng;

use crate::Position;

#[derive(Debug)]
pub struct Leaf {
    pub depth: usize,
    pub min_size: usize,
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub children: Vec<Leaf>,
}

impl Leaf {
    pub fn new(
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        min_size: usize,
        depth: usize,
    ) -> Self {
        Leaf {
            depth,
            x,
            y,
            width,
            height,
            min_size,
            children: vec![],
        }
    }

    fn split(&mut self, rng: &mut ChaCha8Rng) -> bool {
        // if over size and is big enough to split
        // if depth < 2 always split otherwise it'll be boring
        // otherwise 75% chance to split

        // usually split
        // TODO base depth on size
        let should_split = if self.depth > 2 {
            rng.gen_bool(0.85)
        } else {
            true
        };
        if should_split && self.width / 2 >= self.min_size && self.height / 2 >= self.min_size {
            let horz_mid = self.width / 2;
            let vert_mid = self.height / 2;
            let north_west = Leaf::new(
                self.x,
                self.y,
                horz_mid,
                vert_mid,
                self.min_size,
                self.depth + 1,
            );
            let north_east = Leaf::new(
                self.x + horz_mid,
                self.y,
                horz_mid,
                vert_mid,
                self.min_size,
                self.depth + 1,
            );
            let south_east = Leaf::new(
                self.x + horz_mid,
                self.y + vert_mid,
                horz_mid,
                vert_mid,
                self.min_size,
                self.depth + 1,
            );
            let south_west = Leaf::new(
                self.x,
                self.y + vert_mid,
                horz_mid,
                vert_mid,
                self.min_size,
                self.depth + 1,
            );

            self.children.push(north_west);
            self.children.push(north_east);
            self.children.push(south_east);
            self.children.push(south_west);

            true
        } else {
            false
        }
    }

    fn can_split(&self) -> bool {
        self.children.len() == 0
    }

    pub fn generate(&mut self, rng: &mut ChaCha8Rng) {
        if self.can_split() {
            if self.split(rng) {
                self.children
                    .iter_mut()
                    .for_each(|child| child.generate(rng));
            }
        }
    }

    pub fn add_start(&self, starting_points: &mut HashSet<Position>, rng: &mut ChaCha8Rng) {
        if self.children.len() > 0 {
            self.children.iter().for_each(|child| {
                child.add_start(starting_points, rng);
            });
        } else {
            let x = rng.gen_range(self.x..self.x + self.width);
            let y = rng.gen_range(self.y..self.y + self.height);
            starting_points.insert(Position { x, y });

            // let x = rng.gen_range(self.x..self.x + self.width);
            // let y = rng.gen_range(self.y..self.y + self.height);
            // starting_points.insert(Position { x, y });
        }
    }

    pub fn draw(&self) -> String {
        let tile_size = 64;
        let mut output = format!(
            "<svg viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">",
            self.width * tile_size,
            self.height * tile_size
        );

        output += &self.draw_leaves(tile_size);

        output += "</svg>";

        output
    }

    fn draw_leaves(&self, tile_size: usize) -> String {
        self.children.iter().map(|leaf| if leaf.can_split() {
          format!("
          <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" stroke-width=\"2\" fill=\"transparent\" stroke=\"black\" />",
          (leaf.x * tile_size), (leaf.y * tile_size), leaf.width * tile_size, leaf.height * tile_size)
        } else {
          leaf.draw_leaves(tile_size)
        }).collect()
    }
}
