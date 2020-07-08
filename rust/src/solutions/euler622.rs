use crate::utils::numeric::divisors::divisors;

fn run_shuffle_eq(bound: i64, n: i64) -> bool {
    let mut left = bound - 1;
    let mut l = 2;
    loop {
        if left <= 0 {
            break l == 1;
        } else if l == 1 {
            break false;
        } else {
            left -= 1;
            l = (l + l) % n;
        }
    }
}

fn sum_s(n: u32) -> i64 {
    divisors(2i64.pow(n) - 1)
        .into_iter()
        .skip(1)
        .filter(|&l| run_shuffle_eq(n as i64, l))
        .map(|l| l + 1)
        .sum()
}

pub fn euler622() -> String {
    sum_s(60).to_string()
}
