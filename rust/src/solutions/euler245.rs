use crate::utils::numeric::divisors_with_sieve::divisors;
use num::{integer::Roots, Integer};
use primal::Sieve;

struct Query {
    n: usize,
    phi: usize,
    this_prime: usize,
    next_prime: usize,
}

fn queries<'a>(
    bound: usize,
    primes_count: u32,
    n: usize,
    phi: usize,
    primes: &'a [usize],
) -> Box<dyn Iterator<Item = Query> + 'a> {
    if primes_count == 0 {
        Box::new(std::iter::once(Query {
            n,
            phi,
            this_prime: primes[0],
            next_prime: primes[1],
        }))
    } else {
        Box::new(
            (1usize..)
                .take_while(move |&i| n * primes[i].pow(primes_count + 1) <= bound)
                .flat_map(move |i| {
                    let p = primes[i];
                    queries(bound, primes_count - 1, n * p, phi * (p - 1), &primes[i..])
                }),
        )
    }
}

pub fn euler245() -> String {
    let bound: usize = 200_000_000_000;
    let bound_23 = bound.cbrt().pow(2);
    let sieve = Sieve::new(bound_23);
    let primes = sieve
        .primes_from(0)
        .take_while(|&p| p < bound_23)
        .collect::<Vec<usize>>();
    let mut ans = 0;

    // Semiprimes
    for k in (2..bound.sqrt()).step_by(2) {
        for d in divisors(&sieve, k * k - k + 1) {
            let p1 = d + k;
            let p2 = k + (k * k - k + 1) / d;
            if p1 > p2 && p1 * p2 <= bound && sieve.is_prime(p1) && sieve.is_prime(p2) {
                ans += p1 * p2;
            }
        }
    }

    // Standard
    for primes_count in 2..5 {
        for Query {
            n,
            phi,
            this_prime,
            next_prime,
        } in queries(bound, primes_count, 1, 1, &primes)
        {
            if n * next_prime <= bound {
                let k_max = (n - 1) / (n - phi);
                let k_min = {
                    let mut k_min =
                        (n * next_prime - 1) / (n * next_prime - phi * (next_prime - 1));
                    k_min += k_min % 2;
                    k_min
                };
                for k in (k_min..=k_max).step_by(2) {
                    let (p, r) = (1 + k * phi).div_rem(&(n - k * (n - phi)));
                    if r == 0 && n * p <= bound && p > this_prime && sieve.is_prime(p) {
                        ans += n * p;
                    }
                }
            }
        }
    }

    ans.to_string()
}
