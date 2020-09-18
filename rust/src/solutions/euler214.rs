use crate::utils::number_theory::totient::totient_array;

pub fn euler214() -> String {
    let bound = 40000000;
    let mut chain = Vec::<u32>::with_capacity(bound);
    chain.push(1);
    for tot in totient_array::<usize>(bound).into_iter().skip(1) {
        chain.push(1 + chain[tot - 1]);
    }
    primal::Sieve::new(bound)
        .primes_from(0)
        .take_while(|&p| p < bound)
        .filter(|&p| chain[p - 1] == 25)
        .sum::<usize>()
        .to_string()
}
