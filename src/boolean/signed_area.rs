use geo::Coordinate;
use num_traits::Float;

#[inline]
pub fn signed_area<F>(p0: Coordinate<F>, p1: Coordinate<F>, p2: Coordinate<F>) -> F
where
    F: Float,
{
    (p0.x - p2.x) * (p1.y - p2.y) - (p1.x - p2.x) * (p0.y - p2.y)
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
