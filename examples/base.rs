use rational::Rational;

fn main() {
    let r1 = Rational::new(4, 8);
    let r2 = Rational::new(3, 5);

    println!("{} + {} = {}", r1, r2, r1 + r2);
}
