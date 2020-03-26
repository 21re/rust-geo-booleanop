use num_traits::Float as NumTraitsFloat;
use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::{f32, f64};

fn nextafter(x: f64, y: f64) -> f64 {
    if y == x {
        return y;
    }         

    if x >= f64::INFINITY {
        return f64::INFINITY;
    }
        

    if x <= f64::NEG_INFINITY {
        return f64::NEG_INFINITY;
    }


    if -1.0 <= x && x <= 1.0 {
        dbg!("Hit the min/max if");
        if y > x {
            return x + f64::EPSILON;
        }    
        else {
            return x - f64::EPSILON;
        }
    }
    
    
    dbg!("Destructuring the float");
    let (m, e, s) = integer_decode(x);        
    if y > x {
        let adj_m = m + 1;
        return (s as f64) * (adj_m as f64) * (2f64.powf(e as f64)) as f64
    }
    else {
        let adj_m = m - 1;
        return (s as f64) * (adj_m as f64) * (2f64.powf(e as f64)) as f64
    }
}

fn integer_decode(val: f64) -> (u64, i16, i8) {
    let bits = val.to_bits();
    let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
    let mut exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
    let mantissa = if exponent == 0 {
        (bits & 0xfffffffffffff) << 1
    } else {
        (bits & 0xfffffffffffff) | 0x10000000000000
    };
    exponent -= 1023 + 52;
    (mantissa, exponent, sign)
}

fn nextafterf(x: f32, y: f32) -> f32 {
    if y == x {
        return y;
    }         

    if x >= f32::INFINITY {
        return f32::INFINITY;
    }
        

    if x <= f32::NEG_INFINITY {
        return f32::NEG_INFINITY;
    }


    if -1.0 <= x && x <= 1.0 {
        dbg!("Hit the min/max if");
        if y > x {
            return x + f32::EPSILON;
        }    
        else {
            return x - f32::EPSILON;
        }
    }
    
    
    dbg!("Destructuring the float");
    let (m, e, s) = integer_decodef(x);        
    if y > x {
        let adj_m = m + 1;
        return (s as f32) * (adj_m as f32) * (2f32.powf(e as f32)) as f32
    }
    else {
        let adj_m = m - 1;
        return (s as f32) * (adj_m as f32) * (2f32.powf(e as f32)) as f32
    }
}

fn integer_decodef(val: f32) -> (u32, i16, i8) {
    let bits = val.to_bits();
    let sign: i8 = if bits >> 31 == 0 { 1 } else { -1 };
    let mut exponent: i16 = ((bits >> 23) & 0xff) as i16;
    let mantissa =
        if exponent == 0 { (bits & 0x7fffff) << 1 } else { (bits & 0x7fffff) | 0x800000 };
    exponent -= 127 + 23;
    (mantissa, exponent, sign)
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
            nextafterf(self, std::f32::INFINITY)
        } else {
            nextafterf(self, std::f32::NEG_INFINITY)
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
    use super::{nextafter, nextafterf, Float};
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
        assert_eq!(dummy(0_f32), unsafe { nextafterf(0_f32, std::f32::INFINITY) });
    }
}
