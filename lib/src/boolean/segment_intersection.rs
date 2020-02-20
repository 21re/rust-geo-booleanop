use super::helper::Float;
use geo_types::{Coordinate, Rect};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineIntersection<F>
where
    F: Float,
{
    None,
    Point(Coordinate<F>),
    Overlap(Coordinate<F>, Coordinate<F>),
}

#[inline]
fn get_intersection_bounding_box<F>(
    a1: Coordinate<F>,
    a2: Coordinate<F>,
    b1: Coordinate<F>,
    b2: Coordinate<F>,
) -> Option<Rect<F>>
where
    F: Float,
{
    let a_min_x = a1.x.min(a2.x);
    let a_max_x = a1.x.max(a2.x);
    let a_min_y = a1.y.min(a2.y);
    let a_max_y = a1.y.max(a2.y);
    let b_min_x = b1.x.min(b2.x);
    let b_max_x = b1.x.max(b2.x);
    let b_min_y = b1.y.min(b2.y);
    let b_max_y = b1.y.max(b2.y);
    let min_x = a_min_x.max(b_min_x);
    let max_x = a_max_x.min(b_max_x);
    let min_y = a_min_y.max(b_min_y);
    let max_y = a_max_y.min(b_max_y);
    Some(Rect{
        min: Coordinate{x: min_x, y: min_y},
        max: Coordinate{x: max_x, y: max_y},
    })
}

#[inline]
fn constrain_to_bounding_box<F>(p: Coordinate<F>, bb: Rect<F>) -> Coordinate<F>
where
    F: Float,
{
    Coordinate{
        x: if p.x < bb.min.x {
            bb.min.x
        } else if p.x > bb.max.x {
            bb.max.x
        } else {
            p.x
        },
        y: if p.y < bb.min.y {
            bb.min.y
        } else if p.y > bb.max.y {
            bb.max.y
        } else {
            p.y
        },
    }
}

pub fn intersection<F>(
    a1: Coordinate<F>,
    a2: Coordinate<F>,
    b1: Coordinate<F>,
    b2: Coordinate<F>,
) -> LineIntersection<F>
where
    F: Float,
{
    let bb = get_intersection_bounding_box(a1, a2, b1, b2);
    if let Some(bb) = bb {
        let inter = intersection_impl(a1, a2, b1, b2);
        match inter {
            LineIntersection::Point(p) => {
                LineIntersection::Point(constrain_to_bounding_box(p, bb))
            },
            _ => inter
        }
    } else {
        LineIntersection::None
    }
}

fn intersection_impl<F>(
    a1: Coordinate<F>,
    a2: Coordinate<F>,
    b1: Coordinate<F>,
    b2: Coordinate<F>,
) -> LineIntersection<F>
where
    F: Float,
{
    println!("{:?} {:?} {:?} {:?}", a1, a2, b1, b2);
    let va = Coordinate {
        x: a2.x - a1.x,
        y: a2.y - a1.y,
    };
    let vb = Coordinate {
        x: b2.x - b1.x,
        y: b2.y - b1.y,
    };
    let e = Coordinate {
        x: b1.x - a1.x,
        y: b1.y - a1.y,
    };
    let mut kross = cross_product(va, vb);
    let mut sqr_kross = kross * kross;
    let sqr_len_a = dot_product(va, va);

    if sqr_kross > F::zero() {
        let s = cross_product(e, vb) / kross;
        if s < F::zero() || s > F::one() {
            return LineIntersection::None;
        }
        let t = cross_product(e, va) / kross;
        if t < F::zero() || t > F::one() {
            return LineIntersection::None;
        }

        if s == F::zero() || s == F::one() {
            println!("s = {:?} => {:?}", s, mid_point(a1, s, va));
            return LineIntersection::Point(mid_point(a1, s, va));
        }
        if t == F::zero() || t == F::one() {
            println!("t = {:?} => {:?}", s, mid_point(a1, s, va));
            return LineIntersection::Point(mid_point(b1, t, vb));
        }

        println!("s = {:?} => {:?}", s, mid_point(a1, s, va));
        return LineIntersection::Point(mid_point(a1, s, va));
    }

    kross = cross_product(e, va);
    sqr_kross = kross * kross;

    if sqr_kross > F::zero() {
        return LineIntersection::None;
    }

    let sa = dot_product(va, e) / sqr_len_a;
    let sb = sa + dot_product(va, vb) / sqr_len_a;
    let smin = sa.min(sb);
    let smax = sa.max(sb);

    if smin <= F::one() && smax >= F::zero() {
        if smin == F::one() {
            return LineIntersection::Point(mid_point(a1, smin, va));
        }
        if smax == F::zero() {
            return LineIntersection::Point(mid_point(a1, smax, va));
        }

        return LineIntersection::Overlap(
            mid_point(a1, smin.max(F::zero()), va),
            mid_point(a1, smax.min(F::one()), va),
        );
    }

    LineIntersection::None
}

fn mid_point<F>(p: Coordinate<F>, s: F, d: Coordinate<F>) -> Coordinate<F>
where
    F: Float,
{
    Coordinate {
        x: p.x + s * d.x,
        y: p.y + s * d.y,
    }
}

#[inline]
fn cross_product<F>(a: Coordinate<F>, b: Coordinate<F>) -> F
where
    F: Float,
{
    a.x * b.y - a.y * b.x
}

#[inline]
fn dot_product<F>(a: Coordinate<F>, b: Coordinate<F>) -> F
where
    F: Float,
{
    a.x * b.x + a.y * b.y
}

#[cfg(test)]
mod test {
    use super::super::helper::test::xy;
    use super::*;

    #[test]
    fn test_get_intersection_bounding_box() {
        assert_eq!(
            get_intersection_bounding_box(xy(0, 0), xy(2, 2), xy(1, 1), xy(3, 3)),
            Some(Rect{min: xy(1, 1), max: xy(2, 2)}),
        );
        assert_eq!(
            get_intersection_bounding_box(xy(-1, 0), xy(1, 0), xy(0, -1), xy(0, 1)),
            Some(Rect{min: xy(0, 0), max: xy(0, 0)}),
        );
        assert_eq!(
            get_intersection_bounding_box(xy(0, 0), xy(1, 1), xy(2, 0), xy(3, 1)),
            None,
        );
        assert_eq!(
            get_intersection_bounding_box(xy(3, 0), xy(2, 1), xy(1, 0), xy(0, 1)),
            None,
        );
        assert_eq!(
            get_intersection_bounding_box(xy(0, 0), xy(1, 1), xy(0, 2), xy(1, 3)),
            None,
        );
        assert_eq!(
            get_intersection_bounding_box(xy(0, 3), xy(1, 2), xy(0, 1), xy(1, 0)),
            None,
        );
    }

    #[test]
    fn test_intersection() {
        assert_eq!(
            intersection(xy(0, 0), xy(1, 1), xy(1, 0), xy(2, 2)),
            LineIntersection::None
        );
        assert_eq!(
            intersection(xy(0, 0), xy(1, 1), xy(1, 0), xy(10, 2)),
            LineIntersection::None
        );
        assert_eq!(
            intersection(xy(2, 2), xy(3, 3), xy(0, 6), xy(2, 4)),
            LineIntersection::None
        );

        assert_eq!(
            intersection(xy(0, 0), xy(1, 1), xy(1, 0), xy(0, 1)),
            LineIntersection::Point(xy(0.5, 0.5))
        );

        assert_eq!(
            intersection(xy(0, 0), xy(1, 1), xy(0, 1), xy(0, 0)),
            LineIntersection::Point(xy(0, 0))
        );
        assert_eq!(
            intersection(xy(0, 0), xy(1, 1), xy(0, 1), xy(1, 1)),
            LineIntersection::Point(xy(1, 1))
        );

        assert_eq!(
            intersection(xy(0, 0), xy(1, 1), xy(0.5, 0.5), xy(1, 0)),
            LineIntersection::Point(xy(0.5, 0.5))
        );

        assert_eq!(
            intersection(xy(0, 0), xy(10, 10), xy(1, 1), xy(5, 5)),
            LineIntersection::Overlap(xy(1, 1), xy(5, 5))
        );
        assert_eq!(
            intersection(xy(1, 1), xy(10, 10), xy(1, 1), xy(5, 5)),
            LineIntersection::Overlap(xy(1, 1), xy(5, 5))
        );
        assert_eq!(
            intersection(xy(3, 3), xy(10, 10), xy(0, 0), xy(5, 5)),
            LineIntersection::Overlap(xy(3, 3), xy(5, 5))
        );
        assert_eq!(
            intersection(xy(0, 0), xy(1, 1), xy(0, 0), xy(1, 1)),
            LineIntersection::Overlap(xy(0, 0), xy(1, 1))
        );
        assert_eq!(
            intersection(xy(1, 1), xy(0, 0), xy(0, 0), xy(1, 1)),
            LineIntersection::Overlap(xy(1, 1), xy(0, 0))
        );

        assert_eq!(
            intersection(xy(0, 0), xy(1, 1), xy(1, 1), xy(2, 2)),
            LineIntersection::Point(xy(1, 1))
        );
        assert_eq!(
            intersection(xy(1, 1), xy(0, 0), xy(1, 1), xy(2, 2)),
            LineIntersection::Point(xy(1, 1))
        );
        assert_eq!(
            intersection(xy(0, 0), xy(1, 1), xy(2, 2), xy(4, 4)),
            LineIntersection::None
        );
        assert_eq!(
            intersection(xy(0, 0), xy(1, 1), xy(0, -1), xy(1, 0)),
            LineIntersection::None
        );
        assert_eq!(
            intersection(xy(1, 1), xy(0, 0), xy(0, -1), xy(1, 0)),
            LineIntersection::None
        );
        assert_eq!(
            intersection(xy(0, -1), xy(1, 0), xy(0, 0), xy(1, 1)),
            LineIntersection::None
        );

        assert_eq!(
            intersection(xy(0, 0.5), xy(1, 1.5), xy(0, 1), xy(1, 0)),
            LineIntersection::Point(xy(0.25, 0.75))
        );
    }
}
