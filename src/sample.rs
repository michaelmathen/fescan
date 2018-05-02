//extern crate rand;

//use self::rand::Rng;
use ::rand;
use ::rand::distributions::{IndependentSample, Range};


pub fn choose(a: usize, b: usize) -> usize {
    let range = Range::new(a, b);
    let mut rng = rand::thread_rng();
    range.ind_sample(&mut rng)
}
