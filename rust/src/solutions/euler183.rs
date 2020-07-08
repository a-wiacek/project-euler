// Maximum is achieved at k closest to n/e.

fn uberdiv(n: i64, k: i64) -> i64 {
    if n % k == 0 {
        uberdiv(n / k, k)
    } else {
        n
    }
}

fn smallest(n: i64) -> i64 {
    let k = ((n as f64) / std::f64::consts::E).round() as i64;
    if uberdiv(uberdiv(k / num::integer::gcd(n, k), 2), 5) == 1 {
        -n
    } else {
        n
    }
}

pub fn euler183() -> String {
    (5..=10000).map(smallest).sum::<i64>().to_string()
}
