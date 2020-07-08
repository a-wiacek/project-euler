pub fn euler179() -> String {
    let bound = 10_000_000;
    let mut number_of_divisors = vec![1; 10_000_000 - 2];
    for p in primal::Sieve::new(bound).primes_from(0) {
        let iter = |state: &mut usize, e| {
            *state *= p;
            if *state >= bound {
                None
            } else {
                Some((*state, e))
            }
        };
        for (pe, e) in (2..).scan(1, iter) {
            for i in (pe..)
                .step_by(pe)
                .take_while(|&x| x < bound)
                .filter(|&x| (x / pe % p) != 0)
            {
                number_of_divisors[i - 2] *= e;
            }
        }
    }
    number_of_divisors
        .windows(2)
        .filter(|window| window[0] == window[1])
        .count()
        .to_string()
}
