
extern crate rand;

pub mod geometry;
pub mod sample;
pub mod polytree;

use geometry::LineTraits;


fn main() {
	let a = geometry::get_line(0.0, 1.0, 1.0);
	let b = geometry::get_line(1.0, 0.0, 1.0);
	println!("{:?}", a.intersection(&b));
}
