use num::BigInt;

pub fn euler132() -> String {
    let _1 = BigInt::from(1);
    let _10 = BigInt::from(10);
    let exp = BigInt::from(1_000_000_000);
    primal::Sieve::new(1_000_000)
        .primes_from(0)
        .filter(|&p| _1 == _10.modpow(&exp, &BigInt::from(9 * p)))
        .take(40)
        .sum::<usize>()
        .to_string()
}
