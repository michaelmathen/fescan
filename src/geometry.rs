use super::sample;
use std::f64;
extern crate random_choice;
use self::random_choice::random_choice;

fn det2(a1: f64, a2: f64, b1: f64, b2: f64) -> f64 {
	a1 * b2 - a2 * b1
}

fn approx_zero(a: f64) -> bool {
	a.abs() <= f64::EPSILON
}

fn approx_gte(a: f64, b: f64) -> bool {
	approx_zero(a - b) || b <= a
}

fn approx_lte(a: f64, b: f64) -> bool {
	approx_zero(a - b) || a <= b
}

fn approx_gt(a: f64, b: f64) -> bool {
	!approx_zero(a - b) && b < a
}

fn approx_lt(a: f64, b: f64) -> bool {
	!approx_zero(a - b) && a < b
}

#[derive(Debug, Copy, Clone)]
pub struct HLine {
	a: f64,
	b: f64,
	c: f64,
}

#[derive(Debug, Copy, Clone)]
pub struct HSegment {
	line: HLine,
	l_pt: HLine,
	r_pt: HLine,
}

#[derive(Debug)]
pub struct Polygon { boundary: Vec<HSegment>,
			crossing: Vec<HSegment>,
			weights: Vec<f64>,
			points: Vec<HLine>,
}

pub fn get_empty_polygon(crossing: Vec<HSegment>,
	 					weights: Vec<f64>,
						points: Vec<HLine>) -> Polygon {
	Polygon {
		boundary: vec![ get_segment(
						&get_line(0.0, 0.0, -1.0),
						&get_line(0.0, 0.0, 0.0),
						&get_line(0.0, 0.0, 0.0))
					, get_segment(
						&get_line(0.0, 0.0, 1.0),
						&get_line(0.0, 0.0, 0.0),
						&get_line(0.0, 0.0, 0.0))],
		crossing: crossing,
        points: points,
        weights: weights
	}
}
pub trait LineTraits {
	fn get_a(&self) -> f64;
	fn get_b(&self) -> f64;
	fn get_c(&self) -> f64;

	fn intersection(&self, other: &LineTraits) -> HLine {
		HLine {a: det2(self.get_b(), self.get_c(),
					other.get_b(), other.get_c()),
			  b: det2(self.get_a(), self.get_c(),
			  		other.get_a(), other.get_c()),
			  c: det2(self.get_a(), self.get_b(),
			  		other.get_a(), other.get_b())}
	}

	fn evaluate(&self, dual: &LineTraits) -> f64 {
		self.get_a() * dual.get_a() + self.get_b() * dual.get_b() + self.get_c() * dual.get_c()
	}

	fn parallel(&self, other: &LineTraits) -> bool {
		approx_zero(det2(self.get_a(), self.get_b(), other.get_a(), other.get_b()))
	}

	fn above_dual_closed(&self, dual: &LineTraits) -> bool{
		approx_lte(0.0, self.evaluate(dual))
	}

	fn below_dual_closed(&self, dual: &LineTraits) -> bool {
		approx_lte(self.evaluate(dual), 0.0)
	}
}

pub fn get_line(a1: f64, b1: f64, c1: f64) -> HLine {
	HLine {a: a1, b: b1, c: c1}
}

pub fn get_segment(a1: &LineTraits, le: &LineTraits, re: &LineTraits) -> HSegment {
	HSegment { line: get_line(a1.get_a(), a1.get_b(), a1.get_c()),
		l_pt: get_line(le.get_a(), le.get_b(), le.get_c()),
		r_pt: get_line(re.get_a(), re.get_b(), re.get_c()),
	}
}

pub fn left_inf(line: &LineTraits) -> HLine {
	get_line(-line.get_b(), -line.get_a(), 0.0)
}

pub fn right_inf(line: &LineTraits) -> HLine {
	get_line(line.get_b(), line.get_a(), 0.0)
}

impl LineTraits for HLine {
	fn get_a(&self) -> f64 { self.a }
	fn get_b(&self) -> f64 { self.b }
	fn get_c(&self) -> f64 { self.c }
}

impl LineTraits for HSegment {
	fn get_a(&self) -> f64 { self.line.a }
	fn get_b(&self) -> f64 { self.line.b }
	fn get_c(&self) -> f64 { self.line.c }
}

impl HSegment {

	fn get_left(&self) -> &HLine {
		&self.l_pt
	}

	fn get_right(&self) -> &HLine {
		&self.r_pt
	}

	fn above_line_closed(&self, line: &LineTraits) -> bool {
		if self.parallel(line) {
			approx_lt(self.get_c(), line.get_c())
		} else {
			line.below_dual_closed(self.get_left()) && line.below_dual_closed(self.get_right())
		}
	}

	fn below_line_closed(&self, line: &LineTraits) -> bool {
		if self.parallel(line) {
			approx_lt(line.get_c(), self.get_c())
		} else {
			line.above_dual_closed(self.get_left()) && line.above_dual_closed(self.get_right())
		}
	}

	fn crosses(&self, line: &LineTraits) -> bool {
		if self.parallel(line) {
			false
		} else {
			(line.above_dual_closed(self.get_left()) && line.below_dual_closed(self.get_right())) ||
				(line.below_dual_closed(self.get_left()) && line.above_dual_closed(self.get_right()))
		}
	}

	fn part_above_nparallel(&self, line: &LineTraits) -> bool {
		line.above_dual_closed(self.get_left()) || line.above_dual_closed(self.get_right())
	}

	fn part_below_nparallel(&self, line: &LineTraits) -> bool {
		line.below_dual_closed(self.get_left()) || line.below_dual_closed(self.get_right())
	}

	fn split(&self, line: &LineTraits) -> (HSegment, HSegment) {
		/*
		* This assumes that the line crosses the segment. The behavior is undefined
		* otherwise.
		*/
		let new_mid = line.intersection(self);
		let left_seg = get_segment(self, self.get_left(), &new_mid);
		let right_seg = get_segment(self, &new_mid, self.get_right());
		if left_seg.above_dual_closed(line) {
			(left_seg, right_seg)
		} else {
			(right_seg, left_seg)
		}
	}
}


fn divide_lines<T>(split_line: &LineTraits, objects: Vec<T>,
					f: fn(&T) -> HSegment,
					g: fn(HSegment, &T) -> T) -> (Vec<T>, Vec<T>) {
	/*
	* Divides the lines into two sets. One above and one below.
	* Can also be used to divide the lines corresponding weights.
	*/
	let mut lower_lines: Vec<T> = Vec::new();
	let mut upper_lines: Vec<T> = Vec::new();
	for obj in objects {
		let seg = f(&obj);
		if seg.parallel(split_line) {
			if approx_lt(seg.get_c(), split_line.get_c()) {
				upper_lines.push(obj);
			} else if approx_gt(seg.get_c(), split_line.get_c()) {
				lower_lines.push(obj);
			}
		} else {
			let u_p = seg.part_above_nparallel(split_line);
			let l_p = seg.part_below_nparallel(split_line);
			if u_p && l_p {
				let (u, l) = seg.split(split_line);
				upper_lines.push(g(u, &obj));
				lower_lines.push(g(l, &obj));
			} else if u_p {
				upper_lines.push(obj);
			} else {
				lower_lines.push(obj);
			}
		}
	}
	(upper_lines, lower_lines)
}


impl Polygon {

	pub fn split(self, split_line: HSegment) -> (Polygon, Polygon) {
		/*
		* Consumes the polygon and creates two new ones.
		*/
		let (upper, lower) = divide_lines(
			&split_line,
			self.crossing.into_iter().zip(self.weights).collect(),
			|&x| x.0,
			|x, &y| (x, y.1));

		let (mut upper_b, mut lower_b) = divide_lines(
			&split_line,
			self.boundary,
			|&x| x,
			|x, &_y| x);
		//let internal_seg = self.make_segment(split_line);
		upper_b.push(split_line);
		lower_b.push(split_line);

		let (l_pts, u_pts) : (Vec<HLine>, Vec<HLine>) = self.points
			.iter()
			.partition(|&x| split_line.below_dual_closed(x));
		(Polygon{ boundary: upper_b,
			crossing: upper.iter().map(|x| x.0).collect(),
			weights: upper.iter().map(|x| x.1).collect(),
			points: u_pts,
		}, Polygon{ boundary: lower_b,
			crossing: lower.iter().map(|x| x.0).collect(),
			weights: lower.iter().map(|x| x.1).collect(),
			points: l_pts,
		})
	}

	pub fn get_pts(&self) -> &Vec<HLine> {
		&self.points
	}

	pub fn get_lines(&self) -> &Vec<HSegment> {
		&self.crossing
	}

	pub fn get_good_split_l(&self) -> HSegment {
		random_choice().random_choice_f64(&self.crossing, &self.weights, 1)[0].clone()
	}

	pub fn get_boundary_pts(&self) -> Vec<HLine> {
		/*
		* This doesn't make sense unless we have at least one line.
		*/
		let mut it = self.boundary.iter().cycle();
		it.next();
		self.boundary.iter()
					.zip(it)
					.map(|x| x.0.intersection(x.1))
					.collect()
	}

	pub fn get_good_split_v(&self) -> HSegment {
		//let bd_pts = self.get_boundary_pts();
		let k = self.boundary.len();
		let first_ix = sample::choose(0, k - 1);
		let second_ix = ((k - 1) / 2 + 1) % k;
		let fp = self.boundary[first_ix].intersection(&self.boundary[(first_ix + 1) % k]);
		let sp = self.boundary[second_ix].intersection(&self.boundary[(second_ix + 1) % k]);
		get_segment(&fp.intersection(&sp), &fp, &sp)
	}

}
