use primal::Sieve;

fn possible(n: usize, sieve: &Sieve) -> bool {
    (1..)
        // .map_while(|i| n.checked_sub(2 * i * i))
        .map(|i| n.checked_sub(2 * i * i))
        .take_while(|i| i.is_some())
        .flatten()
        .any(|i| sieve.is_prime(i))
}

pub fn euler046() -> String {
    let sieve = Sieve::new(7000);
    (3..)
        .step_by(2)
        .filter(|&n| !sieve.is_prime(n))
        .find(|&n| !possible(n, &sieve))
        .unwrap()
        .to_string()
}
