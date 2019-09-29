use nalgebra::{Matrix2, Vector2};
use num_complex::Complex;
use num_traits::{One, Zero};
use rational::{Eval, Rational};

fn main() {
	let mat: Matrix2<Complex<Rational<_>>> = Matrix2::new(Complex::one(), Complex::zero(), Complex::i(), Complex::new(Rational::new(1, 3), Rational::new(1,4)));
	let vec: Vector2<Complex<Rational<_>>> = Vector2::new(Complex::one(), Complex::i());

	println!("{} * {} = {}", mat, vec, mat * vec);
}
