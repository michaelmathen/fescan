
use geometry;
use geometry::LineTraits;
use std::boxed;
use std::collections::BinaryHeap;
use std::cmp::Ordering;



pub enum PolyNode {
    SegNode { segment : geometry::HSegment,
            up: Box<PolyNode>,
            down: Box<PolyNode>,
        },
    Pnode { poly: geometry::Polygon }
}

fn new_tree(lines: Vec<geometry::HLine>,
            weights: Vec<f64>,
            points: Vec<geometry::HLine>) -> PolyNode {
    let mut segments: Vec<geometry::HSegment> = Vec::new();
    for line in lines {
        segments.push(geometry::get_segment(&line,
                    &geometry::left_inf(&line),
                    &geometry::right_inf(&line)));
    }
    PolyNode::Pnode {
        poly: geometry::get_empty_polygon(segments, weights, points)
    }
}

impl PolyNode {

    pub fn cutting_b() {

    }
}
