use rand::{rngs::SmallRng, Rng};

use crate::map::Position;

#[derive(Debug, PartialEq)]
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

    fn split(&mut self, rng: &mut SmallRng) -> bool {
        // if over size and is big enough to split
        // if depth < 2 always split otherwise it'll be boring
        // otherwise 75% chance to split

        // usually split
        // TODO base depth on size
        let should_split = if self.depth > 2 {
            rng.random_bool(0.85)
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
        self.children.is_empty()
    }

    pub fn generate(&mut self, rng: &mut SmallRng) {
        if self.can_split() && self.split(rng) {
            self.children
                .iter_mut()
                .for_each(|child| child.generate(rng));
        }
    }

    pub fn add_start(&self, starting_points: &mut Vec<Position>, rng: &mut SmallRng, density: u8) {
        if !self.children.is_empty() {
            self.children.iter().for_each(|child| {
                child.add_start(starting_points, rng, density);
            });
        } else {
            for _ in 0..density {
                let x = rng.random_range(self.x..self.x + self.width);
                let y = rng.random_range(self.y..self.y + self.height);
                starting_points.push(Position { x, y });

                // let x = rng.random_range(self.x..self.x + self.width);
                // let y = rng.random_range(self.y..self.y + self.height);
                // starting_points.push(Position { x, y });
            }
        }
    }
}

#[cfg(test)]
mod test {
    use rand::{rngs::SmallRng, SeedableRng};

    use crate::{map::Position, quadtree::Leaf};

    #[test]
    fn it_should_create_new_tree() {
        let root = Leaf::new(0, 0, 6, 6, 3, 0);
        assert_eq!(
            root,
            Leaf {
                depth: 0,
                min_size: 3,
                x: 0,
                y: 0,
                width: 6,
                height: 6,
                children: vec![]
            }
        );
    }

    #[test]
    fn it_should_make_children() {
        let mut root = Leaf::new(0, 0, 16, 16, 2, 0);
        // test seed
        let seed = 123;
        let mut rng = SmallRng::seed_from_u64(seed);
        root.generate(&mut rng);
        assert_eq!(
            root,
            Leaf {
                depth: 0,
                min_size: 2,
                x: 0,
                y: 0,
                width: 16,
                height: 16,
                children: vec![
                    Leaf {
                        depth: 1,
                        min_size: 2,
                        x: 0,
                        y: 0,
                        width: 8,
                        height: 8,
                        children: vec![
                            Leaf {
                                depth: 2,
                                min_size: 2,
                                x: 0,
                                y: 0,
                                width: 4,
                                height: 4,
                                children: vec![
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 0,
                                        y: 0,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 2,
                                        y: 0,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 2,
                                        y: 2,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 0,
                                        y: 2,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    }
                                ]
                            },
                            Leaf {
                                depth: 2,
                                min_size: 2,
                                x: 4,
                                y: 0,
                                width: 4,
                                height: 4,
                                children: vec![
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 4,
                                        y: 0,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 6,
                                        y: 0,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 6,
                                        y: 2,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 4,
                                        y: 2,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    }
                                ]
                            },
                            Leaf {
                                depth: 2,
                                min_size: 2,
                                x: 4,
                                y: 4,
                                width: 4,
                                height: 4,
                                children: vec![
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 4,
                                        y: 4,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 6,
                                        y: 4,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 6,
                                        y: 6,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 4,
                                        y: 6,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    }
                                ]
                            },
                            Leaf {
                                depth: 2,
                                min_size: 2,
                                x: 0,
                                y: 4,
                                width: 4,
                                height: 4,
                                children: vec![
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 0,
                                        y: 4,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 2,
                                        y: 4,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 2,
                                        y: 6,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 0,
                                        y: 6,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    }
                                ]
                            }
                        ]
                    },
                    Leaf {
                        depth: 1,
                        min_size: 2,
                        x: 8,
                        y: 0,
                        width: 8,
                        height: 8,
                        children: vec![
                            Leaf {
                                depth: 2,
                                min_size: 2,
                                x: 8,
                                y: 0,
                                width: 4,
                                height: 4,
                                children: vec![
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 8,
                                        y: 0,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 10,
                                        y: 0,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 10,
                                        y: 2,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 8,
                                        y: 2,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    }
                                ]
                            },
                            Leaf {
                                depth: 2,
                                min_size: 2,
                                x: 12,
                                y: 0,
                                width: 4,
                                height: 4,
                                children: vec![
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 12,
                                        y: 0,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 14,
                                        y: 0,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 14,
                                        y: 2,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 12,
                                        y: 2,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    }
                                ]
                            },
                            Leaf {
                                depth: 2,
                                min_size: 2,
                                x: 12,
                                y: 4,
                                width: 4,
                                height: 4,
                                children: vec![
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 12,
                                        y: 4,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 14,
                                        y: 4,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 14,
                                        y: 6,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 12,
                                        y: 6,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    }
                                ]
                            },
                            Leaf {
                                depth: 2,
                                min_size: 2,
                                x: 8,
                                y: 4,
                                width: 4,
                                height: 4,
                                children: vec![
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 8,
                                        y: 4,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 10,
                                        y: 4,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 10,
                                        y: 6,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 8,
                                        y: 6,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    }
                                ]
                            }
                        ]
                    },
                    Leaf {
                        depth: 1,
                        min_size: 2,
                        x: 8,
                        y: 8,
                        width: 8,
                        height: 8,
                        children: vec![
                            Leaf {
                                depth: 2,
                                min_size: 2,
                                x: 8,
                                y: 8,
                                width: 4,
                                height: 4,
                                children: vec![
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 8,
                                        y: 8,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 10,
                                        y: 8,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 10,
                                        y: 10,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 8,
                                        y: 10,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    }
                                ]
                            },
                            Leaf {
                                depth: 2,
                                min_size: 2,
                                x: 12,
                                y: 8,
                                width: 4,
                                height: 4,
                                children: vec![
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 12,
                                        y: 8,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 14,
                                        y: 8,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 14,
                                        y: 10,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 12,
                                        y: 10,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    }
                                ]
                            },
                            Leaf {
                                depth: 2,
                                min_size: 2,
                                x: 12,
                                y: 12,
                                width: 4,
                                height: 4,
                                children: vec![
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 12,
                                        y: 12,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 14,
                                        y: 12,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 14,
                                        y: 14,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 12,
                                        y: 14,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    }
                                ]
                            },
                            Leaf {
                                depth: 2,
                                min_size: 2,
                                x: 8,
                                y: 12,
                                width: 4,
                                height: 4,
                                children: vec![
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 8,
                                        y: 12,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 10,
                                        y: 12,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 10,
                                        y: 14,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 8,
                                        y: 14,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    }
                                ]
                            }
                        ]
                    },
                    Leaf {
                        depth: 1,
                        min_size: 2,
                        x: 0,
                        y: 8,
                        width: 8,
                        height: 8,
                        children: vec![
                            Leaf {
                                depth: 2,
                                min_size: 2,
                                x: 0,
                                y: 8,
                                width: 4,
                                height: 4,
                                children: vec![
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 0,
                                        y: 8,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 2,
                                        y: 8,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 2,
                                        y: 10,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 0,
                                        y: 10,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    }
                                ]
                            },
                            Leaf {
                                depth: 2,
                                min_size: 2,
                                x: 4,
                                y: 8,
                                width: 4,
                                height: 4,
                                children: vec![
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 4,
                                        y: 8,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 6,
                                        y: 8,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 6,
                                        y: 10,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 4,
                                        y: 10,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    }
                                ]
                            },
                            Leaf {
                                depth: 2,
                                min_size: 2,
                                x: 4,
                                y: 12,
                                width: 4,
                                height: 4,
                                children: vec![
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 4,
                                        y: 12,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 6,
                                        y: 12,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 6,
                                        y: 14,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 4,
                                        y: 14,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    }
                                ]
                            },
                            Leaf {
                                depth: 2,
                                min_size: 2,
                                x: 0,
                                y: 12,
                                width: 4,
                                height: 4,
                                children: vec![
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 0,
                                        y: 12,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 2,
                                        y: 12,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 2,
                                        y: 14,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    },
                                    Leaf {
                                        depth: 3,
                                        min_size: 2,
                                        x: 0,
                                        y: 14,
                                        width: 2,
                                        height: 2,
                                        children: vec![]
                                    }
                                ]
                            }
                        ]
                    }
                ]
            }
        );
    }

    #[test]
    fn it_should_add_start_nodes() {
        let root = Leaf {
            depth: 0,
            min_size: 3,
            x: 0,
            y: 0,
            width: 6,
            height: 6,
            children: vec![
                Leaf {
                    depth: 1,
                    min_size: 3,
                    x: 0,
                    y: 0,
                    width: 3,
                    height: 3,
                    children: vec![],
                },
                Leaf {
                    depth: 1,
                    min_size: 3,
                    x: 0,
                    y: 0,
                    width: 3,
                    height: 3,
                    children: vec![],
                },
            ],
        };

        let seed = 123;
        let mut rng = SmallRng::seed_from_u64(seed);
        let mut starting_points = Vec::new();
        root.add_start(&mut starting_points, &mut rng, 2);

        assert_eq!(
            starting_points,
            vec![
                Position { x: 1, y: 2 },
                Position { x: 1, y: 1 },
                Position { x: 0, y: 1 },
                Position { x: 2, y: 2 }
            ]
        );
    }
}
