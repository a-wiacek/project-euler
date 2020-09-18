use crate::utils::numeric::factorization::Factorization;
use num::integer::Roots;
use primal::Sieve;
use std::iter::successors;

fn totient_of_square_is_a_cube(sieve: &Sieve, factorization: Factorization<usize>) -> bool {
    factorization
        .clone()
        .factors()
        .into_iter()
        .map(|(p, times)| Factorization::new_prime(p, times - 1) * Factorization::new(p - 1, sieve))
        .fold(factorization, |a, b| a * b)
        .factors()
        .into_iter()
        .all(|(_, times)| times % 3 == 0)
}

fn less_than(
    base: Factorization<usize>,
    bound: usize,
    primes: &[usize],
) -> Vec<Factorization<usize>> {
    let mut ans = vec![base];
    let mut final_ans = vec![];
    for &p in primes {
        let mut new_ans = vec![];
        let p_fact = Factorization::new_prime(p, 1);
        for fact in ans {
            let facts = successors(Some(fact), |fact| Some(fact.clone() * p_fact.clone()))
                .take_while(|fact| fact.value() < bound)
                .collect::<Vec<_>>();
            let (last_fact, rem_facts) = facts.split_last().unwrap();
            final_ans.push(last_fact.clone());
            new_ans.extend_from_slice(rem_facts);
        }
        ans = new_ans;
    }
    ans.extend_from_slice(&final_ans[..]);
    ans
}

fn compute_with_prime(sieve: &Sieve, bound: usize, p: usize, ps: &[usize]) -> usize {
    successors(
        Some(Factorization::new_unsafe(vec![(p, 2)], Some(p * p))),
        |fact| {
            let (p, count) = fact.clone().factors()[0];
            Some(Factorization::new_unsafe(
                vec![(p, count + 3)],
                fact.value_safe()
                    .and_then(|value| value.checked_mul(p * p * p)),
            ))
        },
    )
    .take_while(|fact| {
        fact.value_safe()
            .map(|value| value < bound)
            .unwrap_or(false)
    })
    .flat_map(|fact| less_than(fact, bound, ps))
    .map(|fact| {
        let value = fact.value();
        if totient_of_square_is_a_cube(sieve, fact) {
            value
        } else {
            0
        }
    })
    .sum()
}

fn compute(bound: usize) -> usize {
    let sieve = Sieve::new(bound.sqrt() + 1);
    let primes = sieve.primes_from(0).collect::<Vec<_>>();
    (0..primes.len())
        .map(|i| compute_with_prime(&sieve, bound, primes[i], &primes[..i]))
        .sum()
}

pub fn euler342() -> String {
    compute(10_000_000_000).to_string()
}
