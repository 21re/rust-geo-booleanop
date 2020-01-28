use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use num_traits::Float as NumTraitsFloat;

pub trait Float: NumTraitsFloat + Debug + Display {}
impl<T: NumTraitsFloat + Debug + Display> Float for T {}

#[inline]
pub fn less_if(condition: bool) -> Ordering {
    if condition {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

#[cfg(test)]
pub mod test {
    use geo_types::Coordinate;

    pub fn xy<X: Into<f64>, Y: Into<f64>>(x: X, y: Y) -> Coordinate<f64> {
        Coordinate {
            x: x.into(),
            y: y.into(),
        }
    }
}
