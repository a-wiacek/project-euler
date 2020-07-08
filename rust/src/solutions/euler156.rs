use crate::utils::numeric::digits::Digits;
use num::Integer;
use std::cmp::{max, Ordering};
use std::ops::Add;

fn f(n: usize, d: usize) -> usize {
    let (m, g) = n.div_mod_floor(&10);
    let e = (g >= d) as usize;
    if m == 0 {
        e
    } else {
        10 * f(m, d) + m + e - m.digits_count()[d] * (9 - g)
    }
}

fn iter(d: usize) -> usize {
    let mut n = 100_000_000_000;
    let mut acc = 0;
    loop {
        if n < 10 {
            return acc;
        } else {
            let fnd = f(n, d);
            match n.cmp(&fnd) {
                Ordering::Less => n -= max(1, (fnd - n) / n.digits().len()),
                Ordering::Equal => {
                    acc += n;
                    n -= 1;
                }
                Ordering::Greater => n = fnd,
            }
        }
    }
}

pub fn euler156() -> String {
    (1..10).map(iter).sum::<usize>().add(1).to_string()
}
