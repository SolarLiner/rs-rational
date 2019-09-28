use num_complex::Complex;
use rational::Rational;

fn main() {
    let r1 = Rational::new(Complex::new(1, 2), Complex::new(5, 4));
    let r2 = Rational::new(Complex::new(1, 0), Complex::i());

    println!("{} + {} = {}", r1, r2, r1 + r2);

    println!("{}", Complex::new(Rational::new(1, 2), Rational::from(2)));
}
