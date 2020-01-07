pub fn euler003() -> String {
    let mut n: i64 = 600851475143;
    let mut p = 2;
    loop {
        while n % p == 0 {
            n /= p;
        }
        if n == 1 {
            return p.to_string();
        }
        p = p + 1;
    }
}