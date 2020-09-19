use crate::utils::number_theory::totient::totient_sum;
use num::BigInt;

fn f(n: usize) -> BigInt {
    std::iter::successors(Some(n / 2), |&n| Some(n / 2).filter(|&n| n > 1))
        .map(|x| totient_sum::<BigInt>(x) - 1)
        .sum()
}

pub fn euler643() -> String {
    (f(10usize.pow(11)) % BigInt::from(1_000_000_007)).to_string()
}
