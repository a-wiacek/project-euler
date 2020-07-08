use std::ops::Add;

pub fn euler136() -> String {
    let bound = 50_000_000;
    primal::Sieve::new(bound)
        .primes_from(3)
        .take_while(|p| p < &bound)
        .map(|p| {
            (if p % 4 == 3 { 0 } else { 1 }..3)
                .take_while(|&x| p * 4usize.pow(x) <= bound)
                .count()
        })
        .sum::<usize>()
        .add(2)
        .to_string()
}
