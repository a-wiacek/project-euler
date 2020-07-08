use crate::utils::numeric::binsearch::binsearch;
use crate::utils::numeric::digits::Digits;
use num::{cast::ToPrimitive, integer::Roots, pow::Pow, BigUint};

pub fn euler080() -> String {
    let v = &BigUint::from(10u32).pow(101u32);
    (2u32..100)
        .filter_map(|n| {
            if n.sqrt().pow(2) == n {
                None
            } else {
                let _n = &BigUint::from(n);
                binsearch(v, &(v * _n), |x: BigUint| &x * &x <= _n * v * v)
                    .digits()
                    .iter()
                    .take(100)
                    .sum::<BigUint>()
                    .to_usize()
            }
        })
        .sum::<usize>()
        .to_string()
}
