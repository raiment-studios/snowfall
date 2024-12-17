use noise::{NoiseFn, OpenSimplex};
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
    pub fn seed(&self) -> u64 {
        self.seed
    }
    pub fn generate_seed() -> u64 {
        rand::random()
    }

    pub fn seed8k(&mut self) -> u64 {
        self.range(1..8192)
    }

    pub fn new_random() -> Self {
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

    pub fn open_simplex(&mut self) -> NoiseGeneratorBuilder {
        let seed: u32 = self.gen();
        NoiseGeneratorBuilder::new(seed)
    }

    pub fn gen<T>(&mut self) -> T
    where
        Standard: Distribution<T>,
    {
        self.rng.gen()
    }

    pub fn sign(&mut self) -> i32 {
        if self.gen() {
            1
        } else {
            -1
        }
    }

    pub fn bool(&mut self) -> bool {
        self.gen()
    }

    pub fn radians(&mut self) -> f32 {
        self.range(0.0..std::f32::consts::PI * 2.0)
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
    pub fn select_n<'a, T>(&mut self, n: usize, v: &'a Vec<T>) -> Vec<&'a T>
    where
        T: Clone,
    {
        let mut n = n.min(v.len());
        let mut pool = Vec::with_capacity(v.len());
        for i in 0..v.len() {
            pool.push(i);
        }

        let mut chosen: Vec<usize> = Vec::with_capacity(n);
        let mut pool_len = pool.len();
        while n > 0 {
            let index: usize = self.range(0..pool.len());
            pool.swap(index, pool_len - 1);
            let j = pool.pop().unwrap();
            chosen.push(j);
            n -= 1;
            pool_len -= 1;
        }

        let mut r: Vec<&'a T> = Vec::with_capacity(chosen.len());
        for i in 0..chosen.len() {
            r.push(&v[chosen[i]]);
        }
        r
    }

    pub fn select_fn<T>(&mut self, v: Vec<T>) -> impl FnMut() -> T
    where
        T: Clone,
    {
        let mut rng = self.fork();
        move || {
            let index: usize = rng.range(0..v.len());
            v[index].clone()
        }
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

#[derive(Debug)]
pub struct NoiseGen {
    generator: OpenSimplex,
    scale: f32,
}

impl NoiseGen {
    pub fn gen_2d(&self, u: f32, v: f32) -> f32 {
        let u = u / self.scale;
        let v = v / self.scale;
        let n = self.generator.get([u as f64, v as f64]) / 2.0 + 0.5;
        n as f32
    }

    pub fn gen_3d(&self, u: f32, v: f32, w: f32) -> f32 {
        let u = u / self.scale;
        let v = v / self.scale;
        let w = w / self.scale;
        let n = self.generator.get([u as f64, v as f64, w as f64]) / 2.0 + 0.5;
        n as f32
    }
}

pub struct NoiseGeneratorBuilder {
    seed: u32,
    scale: f32,
}

impl NoiseGeneratorBuilder {
    fn new(seed: u32) -> Self {
        Self { seed, scale: 1.0 }
    }

    pub fn seed(mut self, seed: u32) -> Self {
        self.seed = seed;
        self
    }
    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }
    pub fn build(self) -> NoiseGen {
        NoiseGen {
            generator: OpenSimplex::new(self.seed),
            scale: self.scale,
        }
    }
}
