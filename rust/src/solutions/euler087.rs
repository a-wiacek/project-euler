use primal::Sieve;

pub fn euler087() -> String {
    let mut vec = Vec::new();
    let bound = 50000000;
    let sieve = Sieve::new(bound);
    for p4 in sieve
        .primes_from(0)
        .map(|p| p.pow(4))
        .take_while(|&p| p < bound)
    {
        for p3 in sieve
            .primes_from(0)
            .map(|p| p.pow(3))
            .take_while(|&p| p4 + p < bound)
        {
            for p2 in sieve
                .primes_from(0)
                .map(|p| p.pow(2))
                .take_while(|&p| p4 + p3 + p < bound)
            {
                vec.push(p4 + p3 + p2);
            }
        }
    }
    vec.sort();
    vec.dedup();
    vec.len().to_string()
}
