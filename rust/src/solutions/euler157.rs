use crate::utils::numeric::divisors::divisors;

fn solutions(n: u32) -> usize {
    let p10 = 10usize.pow(n);
    let mut sols = Vec::new();
    for x in 0..=n + n {
        for y in 0..=n + n {
            let d = 2usize.pow(x) * 5usize.pow(y);
            if d <= p10 {
                let pa = d + p10;
                let pb = p10 * p10 / d + p10;
                for p in divisors(num::integer::gcd(pa, pb)) {
                    sols.push((pa / p, pb / p, p, n))
                }
            }
        }
    }
    sols.sort();
    sols.dedup();
    sols.len()
}

pub fn euler157() -> String {
    (1..10).map(solutions).sum::<usize>().to_string()
}
