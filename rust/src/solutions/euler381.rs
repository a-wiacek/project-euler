use crate::utils::number_theory::invert_mod::InvertMod;

fn s(p: i64) -> i64 {
    if let [a1, a2, a3, a4] = (1..=4)
        .map(|x| (p - x).invert_mod(&p).unwrap())
        .collect::<Vec<_>>()[..]
    {
        let b1 = a1;
        let b2 = b1 * a2 % p;
        let b3 = b2 * a3 % p;
        let b4 = b3 * a4 % p;
        (-1 - b1 - b2 - b3 - b4) % p + p
    } else {
        unreachable!()
    }
}

fn f(n: usize) -> i64 {
    primal::Sieve::new(n)
        .primes_from(5)
        .take_while(|&p| p < n)
        .map(|p| s(p as i64))
        .sum()
}

pub fn euler381() -> String {
    f(100_000_000).to_string()
}
