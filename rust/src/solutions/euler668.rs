use primal::Sieve;
use std::iter::successors;

fn count_products(sieve: &Sieve, max_prime: usize, max_product: usize) -> usize {
    let mut prods = vec![1];
    let mut ans = 1;
    for p in sieve.primes_from(0).take_while(|p| p <= &max_prime) {
        prods = prods.into_iter().flat_map(|prod| {
            successors(Some(prod), |&prod| Some(prod * p))
                .take_while(|&prod| prod * p <= max_product)
        }).collect();
        ans += prods.len();
    }
    ans
}

fn case_2_count(sieve: &Sieve, bound: usize) -> usize {
    count_products(sieve, bound, bound * bound)
}

fn case_2_non_smooth(sieve: &Sieve, bound: usize) -> usize {
    sieve
        .primes_from(0)
        .take_while(|p| p <= &bound)
        .map(|p| count_products(sieve, p, p))
        .sum()
}

pub fn euler668() -> String {
    let bound = 100_000;
    let sieve = Sieve::new(bound);
    (case_2_count(&sieve, bound) - case_2_non_smooth(&sieve, bound)).to_string()
}
