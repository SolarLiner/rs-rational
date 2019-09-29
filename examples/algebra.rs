use nalgebra::Vector3;
use rational::Rational;

fn main() {
    let mat = Vector3::new(Rational::new(1, 2), Rational::new(2, 3), Rational::new(3, 4));
    let q = Vector3::new(Rational::new(3, 5), Rational::new(1, 2), Rational::from(0));

    println!("{} + {} = {}", mat, q, mat+q);
}
