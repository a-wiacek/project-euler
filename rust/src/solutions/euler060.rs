use crate::utils::numeric::digits::{undigits, Digits};
use primal::Sieve;

fn concat_prime(sieve: &Sieve, p: usize, q: usize) -> bool {
    let dp: Vec<usize> = p.digits();
    let dq: Vec<usize> = q.digits();
    [[dp.clone(), dq.clone()], [dq, dp]]
        .iter()
        .all(|d| sieve.is_prime(undigits(d.concat().as_ref())))
}

pub fn euler060() -> String {
    let sieve = Sieve::new(100000000);
    let primes = sieve
        .primes_from(0)
        .take_while(|&p| p < 10000)
        .collect::<Vec<usize>>();
    for &p1 in &primes {
        for &p2 in &primes {
            if !concat_prime(&sieve, p1, p2) {
                continue;
            }
            for &p3 in &primes {
                if ![p1, p2].iter().all(|&p| concat_prime(&sieve, p, p3)) {
                    continue;
                }
                for &p4 in &primes {
                    if ![p1, p2, p3].iter().all(|&p| concat_prime(&sieve, p, p4)) {
                        continue;
                    }
                    for &p5 in &primes {
                        if [p1, p2, p3, p4]
                            .iter()
                            .all(|&p| concat_prime(&sieve, p, p5))
                        {
                            return (p1 + p2 + p3 + p4 + p5).to_string();
                        }
                    }
                }
            }
        }
    }
    unreachable!()
}
