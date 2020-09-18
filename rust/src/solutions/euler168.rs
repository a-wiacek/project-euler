use crate::utils::numeric::digits::Digits;
use num::{pow::Pow, BigInt, Integer};

pub fn euler168() -> String {
    let mut nums = Vec::<BigInt>::new();
    let _0 = BigInt::from(0);
    let _1 = BigInt::from(1);
    let _10 = BigInt::from(10);
    for bi32 in 0..10 {
        let b = BigInt::from(bi32);
        for k in 2u32..=100 {
            for l in 1..=10 {
                let anum = &b * &(&_10.pow(k - 1) - &BigInt::from(l));
                let adem = BigInt::from(10 * l - 1);
                if &anum % &adem == _0 {
                    let n = &(&(&_10 * &anum) / &adem) + &b;
                    if n.digits().len() == k as usize {
                        nums.push(n);
                    }
                }
            }
        }
    }
    nums.sort();
    nums.dedup();
    nums.into_iter()
        .sum::<BigInt>()
        .mod_floor(&BigInt::from(100000))
        .to_string()
}
