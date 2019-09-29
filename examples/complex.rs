use num_complex::Complex;
use rational::{Eval, Rational};

fn main() {
    let r1 = Rational::new(Complex::new(1, 2), Complex::new(5, 4));
    let r2 = Rational::new(Complex::new(1, 0), Complex::i());
    let complex_r = Complex::new(Rational::new(1, 2), Rational::from(2));
    let res = r1 + r2;
    let res_c: Complex<Rational<i32>> = res.into();
    let c_eval: Complex<f64> = complex_r.eval();

    println!("{} + {} = {} (reduced {})", r1, r2, res, res_c);

    println!("{} -> {}", complex_r, c_eval);
}
