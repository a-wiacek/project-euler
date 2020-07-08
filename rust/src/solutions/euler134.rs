use crate::utils::number_theory::invert_mod::InvertMod;
use crate::utils::numeric::digits::Digits;
use num::{pow::Pow, BigInt, Integer};

pub fn euler134() -> String {
    let bound = 1000005;
    primal::Sieve::new(bound)
        .primes_from(5)
        .take_while(|&p| p < bound)
        .collect::<Vec<usize>>()
        .windows(2)
        .map(|ps| {
            let p1 = ps[0] as i64;
            let p2 = ps[1] as i64;
            let l = p1.digits().len() as u32;
            let k = (BigInt::from(-p1) * BigInt::from(10.invert_mod(&p2).unwrap()).pow(l))
                .mod_floor(&BigInt::from(p2));
            BigInt::from(10).pow(l) * k + BigInt::from(p1)
        })
        .sum::<BigInt>()
        .to_string()
}
