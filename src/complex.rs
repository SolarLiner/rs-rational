use crate::traits::{Eval, RationalItem};
use crate::Rational;
use num_complex::Complex;
use num_traits::Num;

impl<T: RationalItem + Into<f32>> Eval<Complex<f32>> for Complex<Rational<T>> {
    fn eval(self) -> Complex<f32> {
        Complex::new(self.re.eval(), self.im.eval())
    }
}
impl<T: RationalItem + Into<f64>> Eval<Complex<f64>> for Complex<Rational<T>> {
    fn eval(self) -> Complex<f64> {
        Complex::new(self.re.eval(), self.im.eval())
    }
}
impl<T: RationalItem + Num + Copy> Into<Complex<Rational<T>>> for Rational<Complex<T>> {
    fn into(self) -> Complex<Rational<T>> {
        let den = self.den.re * self.den.re + self.den.im * self.den.im;
        let re = Rational::new(self.num.re * self.den.re, den);
        let im = Rational::new(self.num.im * self.den.im, den);

        Complex::new(re, im)
    }
}

#[cfg(test)]
mod tests {
    use crate::Rational;
    use crate::traits::{Eval};
    use num_complex::Complex;

    #[test]
    fn complex() {
        let r1 = Rational::new(Complex::new(1, 2), Complex::new(3, -4));
        let rf: Complex<_> = r1.into();
        println!("{}", r1);
        assert_eq!(Complex::new(Rational::new(-1, 5), Rational::new(2, 5)), rf);
    }
}
