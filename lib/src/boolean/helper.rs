use num_traits::Float as NumTraitsFloat;
use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::{f32, f64};

fn nextafter<T>(x: T, y: T) -> T 
where 
    T: NumTraitsFloat
{
    if y == x {
        return y;
    }         

    if x >= T::infinity() {
        return T::infinity();
    }
        
    if x <= T::neg_infinity() {
        return T::neg_infinity();
    }

    if T::from(-1.0).unwrap() <= x && x <= T::from(1.0).unwrap() {
        if y > x {
            return x + T::epsilon();
        }    
        else {
            return x - T::epsilon();
        }
    }
    
    let (m, e, s) = x.integer_decode();        
    if y > x {
        let adj_m = m + 1;
        return T::from(s).unwrap() * T::from(adj_m).unwrap() * T::from(2f64).unwrap().powf(T::from(e).unwrap())
    }
    else {
        let adj_m = m - 1;
        return T::from(s).unwrap() * T::from(adj_m).unwrap() * T::from(2f64).unwrap().powf(T::from(e).unwrap())
    }
}

trait IntegerDecode {
    fn integer_decode(&self) -> (u64, i16, i8);
}

impl IntegerDecode for f64 {
    // See https://github.com/rust-lang/rust/blob/master/src/libcore/num/dec2flt/rawfp.rs
    fn integer_decode(&self) -> (u64, i16, i8) {
        let bits = self.to_bits();
        let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
        let mut exponent: i16 = ((bits >> (f64::MANTISSA_DIGITS - 1)) & 0x7ff) as i16;
        let mantissa = if exponent == 0 {
            (bits & 0xfffffffffffff) << 1
        } else {
            (bits & 0xfffffffffffff) | 0x10000000000000
        };
        exponent -= 1023 + (f64::MANTISSA_DIGITS as i16 - 1);
        (mantissa, exponent, sign)
    }
}

impl IntegerDecode for f32 {
    // See https://github.com/rust-lang/rust/blob/master/src/libcore/num/dec2flt/rawfp.rs
    fn integer_decode(&self) -> (u64, i16, i8) {
        let bits = self.to_bits();
        let sign: i8 = if bits >> 31 == 0 { 1 } else { -1 };
        let mut exponent: i16 = ((bits >> (f32::MANTISSA_DIGITS - 1)) & 0xff) as i16;
        let mantissa =
            if exponent == 0 { (bits & 0x7fffff) << 1 } else { (bits & 0x7fffff) | 0x800000 };
        exponent -= 127 + (f32::MANTISSA_DIGITS as i16 - 1);
        (mantissa as u64, exponent, sign)
    }
}

pub trait Float: NumTraitsFloat + Debug + Display + NextAfter + Into<f64> {}

impl<T: NumTraitsFloat + Debug + Display + NextAfter + Into<f64>> Float for T {}

pub trait NextAfter: NumTraitsFloat {
    fn nextafter(self, up: bool) -> Self;
}

impl NextAfter for f64 {
    fn nextafter(self, up: bool) -> Self {
        if up {
            nextafter(self, std::f64::INFINITY)
        } else {
            nextafter(self, std::f64::NEG_INFINITY)
        }
    }
}

impl NextAfter for f32 {
    fn nextafter(self, up: bool) -> Self {
        if up {
            nextafter(self, std::f32::INFINITY)
        } else {
            nextafter(self, std::f32::NEG_INFINITY)
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

        assert_eq!(dummy(0_f64), nextafter(0_f64, std::f64::INFINITY));
        assert_eq!(dummy(0_f32), nextafter(0_f32, std::f32::INFINITY));
    }
}
