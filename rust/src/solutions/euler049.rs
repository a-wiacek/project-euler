use crate::utils::numeric::digits::Digits;
use primal::Sieve;

pub fn euler049() -> String {
    let sieve = Sieve::new(10000);
    for a in 1000..10000 {
        if a == 1487 || !sieve.is_prime(a) {
            continue;
        }
        let da = a.digits_count();
        for r in 1..=(10000 - a) / 2 {
            let b = a + r;
            let c = b + r;
            if !sieve.is_prime(b) || !sieve.is_prime(c) {
                continue;
            }
            let db = b.digits_count();
            let dc = c.digits_count();
            if da == db && da == dc {
                return [a, b, c].iter().map(|&n| n.to_string()).collect();
            }
        }
    }
    unreachable!()
}
