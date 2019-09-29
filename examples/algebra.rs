use nalgebra::{Vector3, Vector2, Matrix2};
use rational::Rational;

fn main() {
    let mat = Vector3::new(Rational::new(1, 2), Rational::new(2, 3), Rational::new(3, 4));
    let q = Vector3::new(Rational::new(3, 5), Rational::new(1, 2), Rational::from(0));

    println!("{} + {} = {}", mat, q, mat+q);

    let mat = Matrix2::new(Rational::from(0), Rational::from(1), Rational::new(1, 2), Rational::new(3, 4));
    let vec = Vector2::new(Rational::new(1, 3), Rational::new(1, 2));

    println!("{} * {} = {}", mat, vec, mat*vec);
}
