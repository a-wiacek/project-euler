use crate::utils::numeric::digits::Digits;
use num::{cast::ToPrimitive, BigUint};

fn is_pandigital(v: &[usize]) -> bool {
    let mut vv = Vec::from(v);
    vv.sort();
    vv.len() == 9 && vv.into_iter().zip(1..).all(|(x, y)| x == y)
}

fn exact_fib(n: usize) -> BigUint {
    match n {
        1 | 2 => BigUint::from(1u32),
        3 => BigUint::from(2u32),
        _ => {
            let fibh = &exact_fib(n / 2);
            let fibh1 = &exact_fib(1 + n / 2);
            if n % 2 == 0 {
                fibh * &(&(fibh1 + fibh1) - fibh)
            } else {
                &(fibh * fibh) + &(fibh1 * fibh1)
            }
        }
    }
}

const P: usize = 1000000000;

pub fn euler104() -> String {
    let mut prev1 = 1usize;
    let mut prev2 = 1usize;
    for i in 3.. {
        let curr = {
            let x = prev1 + prev2;
            if x >= P {
                x - P
            } else {
                x
            }
        };
        if is_pandigital(&curr.digits())
            && is_pandigital(
                &exact_fib(i)
                    .digits()
                    .into_iter()
                    .take(9)
                    .map(|d| d.to_usize().unwrap())
                    .collect::<Vec<usize>>()[..],
            )
        {
            return i.to_string();
        }
        prev2 = prev1;
        prev1 = curr;
    }
    unreachable!()
}
