// from https://github.com/ucarion/line_intersection/blob/8bd54337fc5732db185d8f4583513c5cbc678be1/src/lib.rs

use geo::{Line, Point};
use num_traits::Float;

/// An interval (continuous subset) of a line.
///
/// `interval_of_intersection` represents what subset of a line this `LineInterval` represents. If
/// `interval_of_intersection` is `[-Infinity, Infinity]`, then it's a line going through
/// `line.start` and `line.end`; if it's `[0, Infinity]` it's a ray, starting at `line.start`
/// extending infinitely in the direction of `line.end` and beyond; if it's `[0, 1]`, it's a line
/// segment from `line.from` to `line.end`.
///
/// It should always be the case that `interval_of_intersection.0 < interval_of_intersection.1`,
/// unless you want a degenerate line that cannot be intersected.
#[derive(Debug, PartialEq)]
pub struct LineInterval {
    pub line: Line<f64>,
    pub interval_of_intersection: (f64, f64),
}

/// The relationship between two line segments.
#[derive(Debug, PartialEq)]
pub enum LineRelation {
    /// The line intervals are not parallel (or anti-parallel), and "meet" each other at exactly
    /// one point.
    DivergentIntersecting(Point<f64>),
    /// The line intervals are not parallel (or anti-parallel), and do not intersect; they "miss"
    /// each other.
    DivergentDisjoint,
    /// The line intervals lie on the same line. They may or may not overlap, and this intersection
    /// is possibly infinite.
    Collinear,
    /// The line intervals are parallel or anti-parallel.
    Parallel,
}

impl LineRelation {
    pub fn unique_intersection(self) -> Option<Point<f64>> {
        match self {
            LineRelation::DivergentIntersecting(p) => {
                Some(p)
            }
            _ => None,
        }
    }
}

impl LineInterval {
    pub fn line_segment(line: Line<f64>) -> LineInterval {
        LineInterval {
            line: line,
            interval_of_intersection: (0.0, 1.0),
        }
    }

    pub fn ray(line: Line<f64>) -> LineInterval {
        LineInterval {
            line: line,
            interval_of_intersection: (
                0.0,
                f64::infinity(),
            ),
        }
    }

    pub fn line(line: Line<f64>) -> LineInterval {
        LineInterval {
            line: line,
            interval_of_intersection: (
                f64::neg_infinity(),
                f64::infinity(),
            ),
        }
    }

    /// Get the relationship between this line segment and another.
    pub fn relate(
        &self,
        other: &LineInterval,
    ) -> LineRelation {
        // see https://stackoverflow.com/a/565282
        let p = self.line.start_point();
        let q = other.line.start_point();
        let r = self.line.end_point() - p;
        let s = other.line.end_point() - q;

        let r_cross_s = Self::cross(&r, &s);
        let q_minus_p = q - p;
        let q_minus_p_cross_r = Self::cross(&q_minus_p, &r);

        // are the lines are parallel?
        if r_cross_s == 0.0 {
            // are the lines collinear?
            if q_minus_p_cross_r == 0.0 {
                // the lines are collinear
                LineRelation::Collinear
            } else {
                // the lines are parallel but not collinear
                LineRelation::Parallel
            }
        } else {
            // the lines are not parallel
            let t = Self::cross(
                &q_minus_p,
                &Self::div(&s, r_cross_s),
            );
            let u = Self::cross(
                &q_minus_p,
                &Self::div(&r, r_cross_s),
            );

            // are the intersection coordinates both in range?
            let t_in_range =
                self.interval_of_intersection.0 <= t
                    && t <= self.interval_of_intersection.1;
            let u_in_range = other
                .interval_of_intersection
                .0
                <= u
                && u <= other.interval_of_intersection.1;

            if t_in_range && u_in_range {
                // there is an intersection
                LineRelation::DivergentIntersecting(
                    Self::t_coord_to_point(&p, &r, t),
                )
            } else {
                // there is no intersection
                LineRelation::DivergentDisjoint
            }
        }
    }

    fn cross(a: &Point<f64>, b: &Point<f64>) -> f64 {
        a.x() * b.y() - a.y() * b.x()
    }

    fn div(a: &Point<f64>, b: f64) -> Point<f64> {
        (a.x() / b, a.y() / b).into()
    }

    fn t_coord_to_point(
        p: &Point<f64>,
        r: &Point<f64>,
        t: f64,
    ) -> Point<f64> {
        (p.x() + t * r.x(), p.y() + t * r.y()).into()
    }
}
