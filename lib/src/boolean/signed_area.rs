use super::helper::Float;
use geo_types::Coord;
use robust::{orient2d, Coord as RobustCoord};

#[inline]
pub fn coord_to_robust<F>(p: Coord<F>) -> RobustCoord<F>
where
    F: Float,
{
    RobustCoord { x: p.x, y: p.y }
}

#[inline]
pub fn signed_area<F>(p0: Coord<F>, p1: Coord<F>, p2: Coord<F>) -> f64
where
    F: Float,
{
    orient2d(coord_to_robust(p0), coord_to_robust(p1), coord_to_robust(p2))
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
