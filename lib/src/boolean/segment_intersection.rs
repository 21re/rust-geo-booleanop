use super::helper::Float;
use geo_types::Coordinate;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineIntersection<F>
where
    F: Float,
{
    None,
    Point(Coordinate<F>),
    Overlap(Coordinate<F>, Coordinate<F>),
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
    let inter = intersection_impl(a1, a2, b1, b2);
    match inter {
        LineIntersection::Point(mut p) => {
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
            if p.x < min_x {
                p.x = min_x;
            }
            if p.x < max_x {
                p.x = max_x;
            }
            if p.y < min_y {
                p.y = min_y;
            }
            if p.y < max_y {
                p.y = max_y;
            }
            LineIntersection::Point(p)
        },
        _ => inter
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
