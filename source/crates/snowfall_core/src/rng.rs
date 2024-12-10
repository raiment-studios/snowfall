use rand::{
    distributions::{
        uniform::{SampleRange, SampleUniform},
        Standard,
    },
    prelude::*,
};
use rand_chacha::ChaCha8Rng;

pub struct RNG {
    seed: u64,
    rng: ChaCha8Rng,
}

impl RNG {
    pub fn generate_seed() -> u64 {
        rand::random()
    }

    pub fn new_from_time() -> Self {
        let seed = rand::random();
        Self::new(seed)
    }
    pub fn new(seed: u64) -> Self {
        let rng = ChaCha8Rng::seed_from_u64(seed);
        Self { seed, rng }
    }

    pub fn fork(&mut self) -> Self {
        Self::new(self.gen())
    }

    pub fn gen<T>(&mut self) -> T
    where
        Standard: Distribution<T>,
    {
        self.rng.gen()
    }

    pub fn bool(&mut self) -> bool {
        self.gen()
    }

    pub fn range<T, R>(&mut self, range: R) -> T
    where
        T: SampleUniform,
        R: SampleRange<T>,
    {
        self.rng.gen_range(range)
    }

    pub fn select<'a, T>(&mut self, v: &'a Vec<T>) -> &'a T {
        let index: usize = self.range(0..v.len());
        &v[index]
    }

    pub fn select_weighted<'a, T>(&mut self, v: &'a Vec<(u32, T)>) -> &'a T {
        let total_weight: u32 = v.iter().map(|(w, _)| *w).sum();
        let mut index = self.range(0..total_weight);
        for (weight, item) in v {
            if index < *weight {
                return item;
            }
            index -= weight;
        }
        panic!("select_weighted: unreachable");
    }
}
