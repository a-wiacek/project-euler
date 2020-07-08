use num::{rational::Ratio, One, Zero};
use std::ops::{Add, Mul};

type R = Ratio<i128>;

fn ratio(n: i128, d: i128) -> R {
    Ratio::new(n, d)
}

struct Poly(R, R); // (Poly a b)(x) == a x + b

impl<'a> Add<&'a R> for &'a Poly {
    type Output = Poly;
    fn add(self, rhs: &'a R) -> Self::Output {
        Poly(self.0.clone(), rhs + &self.1)
    }
}

impl<'a> Mul<&'a R> for &'a Poly {
    type Output = Poly;
    fn mul(self, rhs: &'a R) -> Self::Output {
        Poly(rhs * &self.0, rhs * &self.1)
    }
}

fn eval(poly: &Poly, at: &R) -> R {
    &poly.0 * at + &poly.1
}

fn decipher(chars: &[char]) -> Poly {
    if chars.is_empty() {
        Poly(R::one(), R::zero())
    } else {
        let d3 = decipher(&chars[1..]).mul(&ratio(3, 1));
        match chars[0] {
            'D' => d3,
            'U' => d3.add(&ratio(-2, 1)).mul(&ratio(1, 4)),
            _ => d3.add(&ratio(1, 1)).mul(&ratio(1, 2)),
        }
    }
}

pub fn euler277() -> String {
    let msg = "UDDDUdddDDUDDddDdDddDDUDDdUUDd";
    let the_poly = decipher(&msg.chars().collect::<Vec<char>>()[..]);
    let least_x = (1..)
        .map(|x| ratio(x, 1))
        .find(|x| eval(&the_poly, x).denom() == &1)
        .unwrap();
    let least_a = *eval(&the_poly, &least_x).numer();
    (least_a as usize..)
        .step_by(3usize.pow(msg.len() as u32))
        .find(|&n| n > 10usize.pow(15))
        .unwrap()
        .to_string()
}
