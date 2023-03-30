use rand::distributions::{Distribution, Uniform};
use rand::prelude::ThreadRng;

const ABS_NOISE: f64 = 0.001;
const RELATIVE_NOISE: f64 = 0.01;

pub struct StochasticProcess {
    pub rel_dist: Uniform<f64>,
    pub abs_dist: Uniform<f64>,
    pub rng: ThreadRng,
}

pub struct Sample {
    pub rel: f64,
    pub abs: f64,
}

impl StochasticProcess {
    pub(crate) fn default() -> Self {
        StochasticProcess {
            rel_dist: Uniform::new(-RELATIVE_NOISE, RELATIVE_NOISE),
            abs_dist: Uniform::new(-ABS_NOISE, ABS_NOISE),
            rng: rand::thread_rng(),
        }
    }

    pub fn sample(&mut self) -> Sample {
        Sample {
            rel: self.rel_dist.sample(&mut self.rng),
            abs: self.abs_dist.sample(&mut self.rng),
        }
    }
}
