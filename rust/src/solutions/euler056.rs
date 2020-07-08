use crate::utils::numeric::digits::Digits;
use num::bigint::BigUint;
use num::cast::ToPrimitive;
use num::pow::Pow;

pub fn euler056() -> String {
    let mut ans = 0;
    for a in 1u32..100 {
        for b in 1usize..100 {
            let d: u32 = BigUint::from(a)
                .pow(b)
                .digits()
                .into_iter()
                .map(|x| x.to_u32().unwrap())
                .sum();
            ans = std::cmp::max(ans, d);
        }
    }
    ans.to_string()
}
