use geo_types::Coordinate;
use num_traits::Float;
use robust::{Coord, orient2d};

#[inline]
pub fn coordinate_to_robust<F>(p : Coordinate<F>) -> Coord
where
    F: Float,
{
    Coord{x: p.x.to_f64().unwrap(), y: p.y.to_f64().unwrap()}
}

#[inline]
pub fn signed_area<F>(p0: Coordinate<F>, p1: Coordinate<F>, p2: Coordinate<F>) -> F
where
    F: Float,
{
    let res = orient2d(
        coordinate_to_robust(p0),
        coordinate_to_robust(p1),
        coordinate_to_robust(p2),
    );
    if res > 0f64 {
        F::one()
    } else if res < 0f64 {
        -F::one()
    } else {
        F::zero()
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
