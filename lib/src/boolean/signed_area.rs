use super::helper::Float;
use geo_types::Coordinate;
use robust::{orient2d, Coord};

#[inline]
pub fn coordinate_to_robust<F>(p: Coordinate<F>) -> Coord
where
    F: Float,
{
    Coord {
        x: p.x.into(),
        y: p.y.into(),
    }
}

#[inline]
pub fn signed_area<F>(p0: Coordinate<F>, p1: Coordinate<F>, p2: Coordinate<F>) -> f64
where
    F: Float,
{
    orient2d(
        coordinate_to_robust(p0),
        coordinate_to_robust(p1),
        coordinate_to_robust(p2),
    )
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
