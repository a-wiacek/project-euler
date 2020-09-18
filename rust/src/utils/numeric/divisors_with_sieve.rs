use itertools::Itertools;
use primal::Sieve;

pub fn divisors(sieve: &Sieve, n: usize) -> Vec<usize> {
    if n == 1 {
        vec![1]
    } else if let Ok(factorization) = sieve.factor(n) {
        factorization
            .into_iter()
            .map(|(p, exp)| {
                let mut vec = Vec::with_capacity(exp + 1);
                vec.push(1usize);
                for i in 0..exp {
                    vec.push(vec[i] * p);
                }
                vec
            })
            .multi_cartesian_product()
            .map(|x| x.into_iter().product::<usize>())
            .collect()
    } else {
        panic!("Divisors function failed: could not factorize {}", n)
    }
}
