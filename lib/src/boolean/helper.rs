use float_next_after::NextAfter as NextAfterFloat;
use geo_types::{Coord, CoordNum};
use num_traits::Float as NumTraitsFloat;
use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::{f32, f64};

pub trait Float: NumTraitsFloat + Debug + Display + NextAfter + Into<f64> {}

impl<T: NumTraitsFloat + Debug + Display + NextAfter + Into<f64>> Float for T {}

pub trait NextAfter: NumTraitsFloat {
    fn nextafter(self, up: bool) -> Self;
}

impl NextAfter for f64 {
    fn nextafter(self, up: bool) -> Self {
        if up {
            self.next_after(std::f64::INFINITY)
        } else {
            self.next_after(std::f64::NEG_INFINITY)
        }
    }
}

impl NextAfter for f32 {
    fn nextafter(self, up: bool) -> Self {
        if up {
            self.next_after(std::f32::INFINITY)
        } else {
            self.next_after(std::f32::NEG_INFINITY)
        }
    }
}

#[inline]
pub fn less_if(condition: bool) -> Ordering {
    if condition {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

#[inline]
pub fn less_if_inversed(condition: bool) -> Ordering {
    if condition {
        Ordering::Greater
    } else {
        Ordering::Less
    }
}

/// A bounded 2D quadrilateral whose area is defined by minimum and maximum `Coords`.
///
/// A simple implementation copied from geo_types 0.4.0, because this version is a better
/// fit for the needs of this crate than the newer ones.
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct BoundingBox<T>
where
    T: CoordNum,
{
    pub min: Coord<T>,
    pub max: Coord<T>,
}

impl<T: CoordNum> BoundingBox<T> {
    pub fn width(self) -> T {
        self.max.x - self.min.x
    }

    pub fn height(self) -> T {
        self.max.y - self.min.y
    }
}

#[cfg(test)]
pub mod test {
    use super::Float;
    use float_next_after::NextAfter as NextAfterFloat;
    use geo_types::Coord;

    pub fn xy<X: Into<f64>, Y: Into<f64>>(x: X, y: Y) -> Coord<f64> {
        Coord {
            x: x.into(),
            y: y.into(),
        }
    }

    #[test]
    fn test_float_type_trait() {
        fn dummy<T>(x: T) -> T
        where
            T: Float,
        {
            x.nextafter(true)
        }

        assert_eq!(dummy(0_f64), 0_f64.next_after(std::f64::INFINITY));
        assert_eq!(dummy(0_f32), 0_f32.next_after(std::f32::INFINITY));
    }
}
