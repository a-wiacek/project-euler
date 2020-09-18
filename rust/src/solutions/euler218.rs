use itertools::Itertools;

fn f(x: i32, y: i32) -> i32 {
    (x * x - y * y - 2 * x * y) * (x * x - y * y + 2 * x * y) * (x - y) * (x + y) * 2 * x * y
}

pub fn euler218() -> String {
    (0..2)
        .map(|_| 0..7)
        .multi_cartesian_product()
        .map(|v| f(v[0], v[1]))
        .filter(|&x| x % 7 != 0)
        .count()
        .to_string()
}
