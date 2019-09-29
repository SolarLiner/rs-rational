use num_traits::{Float, Num, One, Zero};
use std::cmp::Ordering;
use std::fmt::{self, Display};
use std::ops::{Add, Div, Mul, Rem, Sub};

#[cfg(feature = "complex")]
mod complex;
mod traits;
pub use traits::{Eval, Gcd, RationalItem};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Rational<T: RationalItem> {
    num: T,
    den: T,
}

impl<T: RationalItem> Rational<T> {
    pub fn new(a: T, b: T) -> Self {
        let (num, den) = Self::simplify(a, b);

        Self { num, den }
    }

    pub fn inverse(self) -> Self {
        Self {
            num: self.den,
            den: self.num,
        }
    }

    fn simplify(a: T, b: T) -> (T, T) {
        if b.is_zero() {
            (T::zero(), T::zero())
        } else if a.is_zero() {
            (T::zero(), T::one())
        } else {
            let gcd = a.clone().gcd(b.clone());
            (a / gcd.clone(), b / gcd)
        }
    }
}

impl<T: RationalItem + Copy + PartialOrd> PartialOrd for Rational<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.num == T::zero() {
            None
        } else {
            (self.num * other.den).partial_cmp(&(other.num * self.den))
        }
    }
}

impl<T: RationalItem> From<(T, T)> for Rational<T> {
    fn from(x: (T, T)) -> Self {
        Self::new(x.0, x.1)
    }
}

impl<T: RationalItem> From<T> for Rational<T> {
    fn from(num: T) -> Self {
        Self { num, den: T::one() }
    }
}

impl<T: RationalItem + Display> Display for Rational<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

impl<T: RationalItem + Into<f32>> Eval<f32> for Rational<T> {
    fn eval(self) -> f32 {
        self.num.into() / self.den.into()
    }
}
impl<T: RationalItem + Into<f64>> Eval<f64> for Rational<T> {
    fn eval(self) -> f64 {
        self.num.into() / self.den.into()
    }
}

impl<T: RationalItem + Copy> Add for Rational<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        let a = self.num * other.den;
        let b = self.den * other.num;

        Self::new(a + b, self.den * other.den)
    }
}

impl<T: RationalItem + Copy> Sub for Rational<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        let a = self.num * other.den;
        let b = self.den * other.num;

        Self::new(a - b, self.den * other.den)
    }
}

impl<T: RationalItem + Copy> Mul for Rational<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self::new(self.num * other.num, self.den * other.den)
    }
}

impl<T: RationalItem + Copy> Div for Rational<T> {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        self * other.inverse()
    }
}

impl<T: RationalItem + Copy> Rem for Rational<T> {
    type Output = Self;
    fn rem(self, other: Self) -> Self::Output {
        Self::new((self.num % other.num) * self.den, other.den * self.den)
    }
}

impl<T: RationalItem + Copy> Zero for Rational<T> {
    fn zero() -> Self {
        Self {
            num: T::zero(),
            den: T::one(),
        }
    }
    fn is_zero(&self) -> bool {
        self.num.is_zero() && !self.den.is_zero()
    }
}

impl<T: RationalItem + Copy> One for Rational<T> {
    fn one() -> Self {
        Self {
            num: T::one(),
            den: T::one(),
        }
    }
    fn is_one(&self) -> bool {
        self.den.is_one() && self.num.is_one()
    }
}

impl<T: RationalItem + Num + Copy> Num for Rational<T> {
    type FromStrRadixErr = <T as Num>::FromStrRadixErr;
    fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        let v = s
            .splitn(2, |c| c == '/' || c == ':')
            .map(|el| T::from_str_radix(el, radix))
            .collect::<Result<Vec<T>, _>>()?;
        Ok(Self::new(v[0], v[1]))
    }
}

#[cfg(test)]
mod tests {
    use super::{Eval, Rational};
    use num_traits::Num;

    #[test]
    fn new() {
        assert_eq!(Rational::new(1, 2), Rational::new(4, 8));
        assert_eq!(Rational::new(0, 1), Rational::new(0, 4));
        assert_eq!(Rational::new(0, 0), Rational::new(3, 0));
    }

    #[test]
    fn eval() {
        assert_eq!(0.5, Rational::new(1, 2).eval())
    }

    #[test]
    fn add() {
        assert_eq!(
            Rational::new(5, 4),
            Rational::new(1, 2) + Rational::new(3, 4)
        );
    }

    #[test]
    fn sub() {
        assert_eq!(
            Rational::new(1, 5),
            Rational::new(3, 5) - Rational::new(2, 5)
        );
        assert_eq!(
            Rational::new(0, 0),
            Rational::new(1, 0) - Rational::new(3, 2)
        );
    }

    #[test]
    fn mul() {
        assert_eq!(
            Rational::new(3, 8),
            Rational::new(1, 2) * Rational::new(3, 4)
        );
    }

    #[test]
    fn div() {
        assert_eq!(
            Rational::new(1, 1),
            Rational::new(1, 2) / Rational::new(1, 2)
        );
        assert_eq!(
            Rational::new(0, 0),
            Rational::new(0, 1) / Rational::new(1, 0)
        );
    }

    #[test]
    fn from_str() {
        assert_eq!(
            Rational::new(2, 3),
            Rational::from_str_radix("2/3", 10).unwrap()
        );
        assert_eq!(
            Rational::new(0x10, 0x12),
            Rational::from_str_radix("10/12", 16).unwrap()
        );
    }
}
