use model::{options::Options, cave::Cave};

use crate::random::Random;

pub mod model;
pub mod random;

pub fn generate(options: &Options) -> Cave {
    let mut cave = Cave::new(options.width, options.height);
    let mut random = Random::new();

    for i in 0..cave.width * cave.height {
        cave.data[i] = random.next() % 2 == 0;
    }

    cave
}
