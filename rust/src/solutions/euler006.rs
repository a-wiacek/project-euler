fn sum_of_squares(n: i64) -> i64 {
    n * (n + 1) * (2 * n + 1) / 6
}

fn square_of_sum(n: i64) -> i64 {
    (n * (n + 1) / 2).pow(2)
}

pub fn euler006() -> String {
    (square_of_sum(100) - sum_of_squares(100)).to_string()
}
