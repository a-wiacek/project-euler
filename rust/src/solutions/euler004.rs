fn reverse_int(mut n: i32) -> i32 {
    let mut k: i32 = 0;
    while n > 0 {
        k = 10 * k + n % 10;
        n = n / 10;
    }
    k
}

fn is_palindrome(n: i32) -> bool {
    n == reverse_int(n)
}

pub fn euler004() -> String {
    let mut max = 0;
    for m in 100..1000 {
        for n in m..1000 {
            let p = m * n;
            if is_palindrome(p) {
                max = if p > max { p } else { max }
            }
        }
    }
    max.to_string()
}
