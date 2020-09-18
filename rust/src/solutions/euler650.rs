use crate::utils::number_theory::invert_mod::InvertMod;
use crate::utils::numeric::{factorization::Factorization, modpow::ModPow};
use num::pow::Pow;
use primal::Sieve;
use std::default::Default;

const P: usize = 1_000_000_007;

fn add(a: usize, b: usize) -> usize {
    let s = a + b;
    if s >= P {
        s - P
    } else {
        s
    }
}

fn mul(a: usize, b: usize) -> usize {
    (a * b) % P
}

fn inv(a: usize) -> usize {
    (a as i64).invert_mod(&(P as i64)).unwrap() as usize
}

fn f(q: usize, a: usize) -> usize {
    mul(q.modpow(&(a + 1), &P) - 1, inv(q - 1))
}

fn sum_of_divisors(fact: Factorization<usize>) -> usize {
    fact.factors()
        .into_iter()
        .fold(1, |a, (p, times)| mul(a, f(p, times)))
}

pub fn euler650() -> String {
    let bound = 20_000;
    let sieve = Sieve::new(bound);
    let number_factorization: Vec<Factorization<usize>> = (1..=bound + 1)
        .map(|n| Factorization::new(n, &sieve))
        .collect();
    let factorial_factorization = {
        let mut vec: Vec<Factorization<usize>> = vec![Default::default()];
        for i in 0..=bound {
            vec.push(&vec[i] * &number_factorization[i]);
        }
        vec
    };
    let g = |fact: &Factorization<usize>, i: usize| -> Factorization<usize> {
        (fact * &number_factorization.get(i - 1).unwrap().pow(i - 1))
            .try_div(&factorial_factorization[i - 1])
            .unwrap()
    };
    (1..=bound)
        .scan(Default::default(), |state: &mut Factorization<usize>, i| {
            *state = g(state, i);
            Some(state.clone())
        })
        .fold(0, |sum, fact| add(sum, sum_of_divisors(fact)))
        .to_string()
}
