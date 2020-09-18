use itertools::Itertools;

fn is_ok(phi: u128, d: u128) -> bool {
    phi * 94744 < (d - 1) * 15499
}

pub fn euler243() -> String {
    let sieve = primal::Sieve::new(1000000);
    let max_p = sieve
        .primes_from(3)
        .scan((2u128, 1u128), |(product, phi), p| {
            let ans = (*product, *phi, p);
            *product *= p as u128;
            *phi *= p as u128 - 1;
            Some(ans)
        })
        .find_map(|(d, phi, p)| if is_ok(phi, d) { Some(p) } else { None })
        .unwrap();
    let phi_f = |k: u128| {
        sieve
            .primes_from(0)
            .take_while(|&p| p < max_p)
            .map(|p| p as u128)
            .filter(|&p| k % p == 0)
            .fold(k, |n, p| n / p * (p - 1))
    };
    sieve
        .primes_from(0)
        .take_while(|&p| p < max_p)
        .map(|p| p as u128)
        .map(|p| vec![1, p, p * p, p * p * p])
        .multi_cartesian_product()
        .map(|vec| vec.into_iter().product::<u128>())
        .filter(|&d| is_ok(phi_f(d), d))
        .min()
        .unwrap()
        .to_string()
}
