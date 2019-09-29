use num_traits::{Num, NumOps, One, Zero};
use std::ops::Rem;

pub trait Gcd: Clone + PartialEq + Zero + Rem<Output = Self> {
    fn gcd(self, other: Self) -> Self {
        let mut a = self.clone();
        let mut b = other.clone();
        while b != Self::zero() {
            let old_b = b.clone();
            b = a % b;
            a = old_b;
        }

        a
    }
}
impl<T: Clone + PartialEq + Zero + Rem<Output = Self>> Gcd for T {}

pub trait RationalItem: Gcd + NumOps + Zero + One {}
impl<T: Gcd + NumOps + Zero + One> RationalItem for T {}

pub trait Eval<T> {
    fn eval(self) -> T;
}
