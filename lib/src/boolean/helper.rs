use float_extras::f64::nextafter;
use num_traits::Float as NumTraitsFloat;
use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::{f32, f64};

// TODO: reimpliment the f32 version of nextafter(f) if possible

// pub trait Float: NumTraitsFloat + Debug + Display + NextAfter + Into<f64> {}

// impl<T: NumTraitsFloat + Debug + Display + NextAfter + Into<f64>> Float for T {}

// pub trait NextAfter: NumTraitsFloat {
//     fn nextafter(self, up: bool) -> Self;
// }

// /// nextafter returns the next representable floating-point value following x in the
// /// direction of y. If y is less than x, these functions will return the largest representable
// /// number less than x.
// pub fn nextafterf(x: f32, y: f32) -> f32 {
//     unsafe { cmath::nextafter(x, y) }
// };

// impl NextAfter for f64 {
//     fn nextafter(self, up: bool) -> Self {
//         if up {
//             unsafe { nextafter(self, std::f64::INFINITY) }
//         } else {
//             unsafe { nextafter(self, std::f64::NEG_INFINITY) }
//         }
//     }
// }

// impl NextAfter for f32 {
//     fn nextafter(self, up: bool) -> Self {
//         if up {
//             unsafe { nextafterf(self, std::f32::INFINITY) }
//         } else {
//             unsafe { nextafterf(self, std::f32::NEG_INFINITY) }
//         }
//     }
// }

pub trait Float: NumTraitsFloat + Debug + Display + NextAfter + Into<f64> {}

impl<T: NumTraitsFloat + Debug + Display + NextAfter + Into<f64>> Float for T {}

pub trait NextAfter: NumTraitsFloat {
    fn nextafter(self, up: bool) -> Self;
    fn nextafter_steps(self, steps: i32) -> Self;

    fn ulp(self) -> Self {
        if self > Self::zero() {
            self.nextafter(true) - self
        } else {
            self.nextafter(false) - self
        }
    }
}

impl NextAfter for f64 {
    fn nextafter(self, up: bool) -> Self {
        if up {
            nextafter(self, std::f64::INFINITY)
        } else {
            nextafter(self, std::f64::NEG_INFINITY)
        }
    }

    fn nextafter_steps(self, steps: i32) -> Self {
        let mut x = self;
        for _ in 0..steps.abs() {
            x = x.nextafter(steps > 0);
        }
        x
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

#[cfg(test)]
pub mod test {
    use super::{nextafter, Float};
    use geo_types::Coordinate;

    pub fn xy<X: Into<f64>, Y: Into<f64>>(x: X, y: Y) -> Coordinate<f64> {
        Coordinate {
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

        assert_eq!(dummy(0_f64), unsafe { nextafter(0_f64, std::f64::INFINITY) });
        //assert_eq!(dummy(0_f32), unsafe { nextafterf(0_f32, std::f32::INFINITY) });
    }
}
