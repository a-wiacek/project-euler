use crate::utils::numeric::digits::{undigits, Digits};
use permutohedron::LexicalPermutation;
use primal::Sieve;

pub fn euler041() -> String {
    let mut vec: Vec<usize> = 7654321.digits();
    let sieve = Sieve::new(7654321);
    loop {
        let n = undigits(&vec);
        if sieve.is_prime(n) {
            break n.to_string();
        }
        vec.prev_permutation();
    }
}
