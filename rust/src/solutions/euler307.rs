use num::{pow::Pow, BigInt};
use std::iter::once;

pub fn euler307() -> String {
    let _1 = || BigInt::from(1);
    // binomial(1_000_000, k): choose chips with 2 defects
    let it1 = once(_1()).chain((1i64..=10000).scan(_1(), |prev, n| {
        let a: BigInt = prev.clone() * (1_000_001 - n) / n;
        *prev = a.clone();
        Some(a)
    }));
    // binomial(1_000_000 - k, 20000 - 2 * k): choose chips with one defect
    let it2 = once(_1())
        .chain(
            (1i64..=10000)
                .scan(_1(), |prev, n| {
                    let t = 1_000_000 - 10_000 + n;
                    let a: BigInt = prev.clone() * t * (t + 1 - 2 * n) / ((2 * n - 1) * 2 * n);
                    *prev = a.clone();
                    Some(a)
                })
                .collect::<Vec<_>>(),
        )
        .rev();
    // binomial(20_000, 2 * k): choose defects going to 2-chips
    let it3 = once(_1())
        .chain(
            (1i64..=10000)
                .scan(_1(), |prev, n| {
                    let t = 20_002 - 2 * n;
                    let a: BigInt = prev.clone() * t * (t - 1) / (2 * n * (2 * n - 1));
                    *prev = a.clone();
                    Some(a)
                })
                .collect::<Vec<_>>(),
        )
        .rev();
    // (2 * k)! / 2^k: distribute those defects
    let it4 = once(_1()).chain((1i64..=10000).scan(_1(), |prev, n| {
        let a: BigInt = prev.clone() * (2 * n - 1) * n;
        *prev = a.clone();
        Some(a)
    }));
    // 20_000 - 2 * k)!: distribute remaining defects to 1-chips
    let it5 = once(_1())
        .chain(
            (1i64..=10000)
                .scan(_1(), |prev, n| {
                    let a: BigInt = prev.clone() * (2 * n - 1) * 2 * n;
                    *prev = a.clone();
                    Some(a)
                })
                .collect::<Vec<_>>(),
        )
        .rev();

    let s = it1
        .zip(it2)
        .map(|(a, b)| a * b)
        .zip(it3)
        .map(|(a, b)| a * b)
        .zip(it4)
        .map(|(a, b)| a * b)
        .zip(it5)
        .map(|(a, b)| a * b)
        .sum::<BigInt>();
    let total = BigInt::from(1_000_000).pow(20_000u32);
    format!("0.{}", &(total - s).to_string()[0..10]) // Nasty
}
