use geo_types::Coordinate;
use num_traits::Float;
use robust::{Coord, orient2d};

#[inline]
pub fn signed_area<F>(p0: Coordinate<F>, p1: Coordinate<F>, p2: Coordinate<F>) -> F
where
    F: Float,
{
    let res = orient2d(
        Coord{x: p0.x.to_f64().unwrap(), y: p0.y.to_f64().unwrap()},
        Coord{x: p1.x.to_f64().unwrap(), y: p1.y.to_f64().unwrap()},
        Coord{x: p2.x.to_f64().unwrap(), y: p2.y.to_f64().unwrap()},
    );
    if res > 0. {
        F::from(1.).unwrap()
    } else if res < 0. {
        F::from(-1.).unwrap()
    } else {
        F::from(0.).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::super::helper::test::xy;
    use super::*;

    #[test]
    fn test_analytical_signed_area() {
        assert_eq!(signed_area(xy(0, 0), xy(0, 1), xy(1, 1)), -1.0);
        assert_eq!(signed_area(xy(0, 1), xy(0, 0), xy(1, 0)), 1.0);
        assert_eq!(signed_area(xy(0, 0), xy(1, 1), xy(2, 2)), 0.0);

        assert_eq!(signed_area(xy(-1, 0), xy(2, 3), xy(0, 1)), 0.0);
        assert_eq!(signed_area(xy(2, 3), xy(-1, 0), xy(0, 1)), 0.0);
    }
}
