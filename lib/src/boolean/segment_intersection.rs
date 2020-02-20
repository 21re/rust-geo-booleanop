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
    let (a_start_x, a_end_x) = if a1.x < a2.x {
        (a1.x, a2.x)
    } else {
        (a2.x, a1.x)
    };
    let (a_start_y, a_end_y) = if a1.y < a2.y {
        (a1.y, a2.y)
    } else {
        (a2.y, a1.y)
    };
    let (b_start_x, b_end_x) = if b1.x < b2.x {
        (b1.x, b2.x)
    } else {
        (b2.x, b1.x)
    };
    let (b_start_y, b_end_y) = if b1.y < b2.y {
        (b1.y, b2.y)
    } else {
        (b2.y, b1.y)
    };
    let interval_start_x = a_start_x.max(b_start_x);
    let interval_start_y = a_start_y.max(b_start_y);
    let interval_end_x = a_end_x.min(b_end_x);
    let interval_end_y = a_end_y.min(b_end_y);
    if interval_start_x <= interval_end_x && interval_start_y <= interval_end_y {
        Some(Rect{
            min: Coordinate{x: interval_start_x, y: interval_start_y},
            max: Coordinate{x: interval_end_x, y: interval_end_y},
        })
    } else {
        None
    }
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
    //println!("{:?} {:?} {:?} {:?}", a1, a2, b1, b2);
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
            //println!("s = {:?} => {:?}", s, mid_point(a1, s, va));
            return LineIntersection::Point(mid_point(a1, s, va));
        }
        if t == F::zero() || t == F::one() {
            //println!("t = {:?} => {:?}", s, mid_point(a1, s, va));
            return LineIntersection::Point(mid_point(b1, t, vb));
        }

        //println!("s = {:?} => {:?}", s, mid_point(a1, s, va));
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
    fn test_constrain_to_bounding_box() {
        assert_eq!(
            constrain_to_bounding_box(xy(100, 0), Rect{min: xy(-1, -1), max: xy(1, 1)}),
            xy(1, 0),
        );
        assert_eq!(
            constrain_to_bounding_box(xy(-100, 0), Rect{min: xy(-1, -1), max: xy(1, 1)}),
            xy(-1, 0),
        );
        assert_eq!(
            constrain_to_bounding_box(xy(0, 100), Rect{min: xy(-1, -1), max: xy(1, 1)}),
            xy(0, 1),
        );
        assert_eq!(
            constrain_to_bounding_box(xy(0, -100), Rect{min: xy(-1, -1), max: xy(1, 1)}),
            xy(0, -1),
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
