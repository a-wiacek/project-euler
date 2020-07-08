// See problem 123 for explanation.

fn remainder(a: i32, n: i32) -> i32 {
    if n % 2 == 0 {
        2
    } else {
        (2 * a * n) % (a * a)
    }
}

fn max_remainder(n: i32) -> i32 {
    (0..=n + n).fold(0, |s, x| std::cmp::max(s, remainder(n, x)))
}

pub fn euler120() -> String {
    (3..=1000).map(max_remainder).sum::<i32>().to_string()
}
