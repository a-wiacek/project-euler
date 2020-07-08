fn a(k: i64) -> i64 {
    if num::integer::gcd(k, 10) == 1 {
        fn a_g(k: i64, acc: i64, n: i64) -> i64 {
            if acc % k == 0 {
                n
            } else {
                a_g(k, (10 * acc + 1) % k, n + 1)
            }
        }
        a_g(k, 1, 1)
    } else {
        -1
    }
}

pub fn euler129() -> String {
    // A(n) < n
    (1000001..).find(|&n| a(n) > 1000000).unwrap().to_string()
}
