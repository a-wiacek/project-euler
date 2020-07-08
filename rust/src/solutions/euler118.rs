use itertools::Itertools;
use primal::is_prime;

// Invariant: digits.len() = commas.len() + 1
fn split(digits: &Vec<u64>, commas: &Vec<bool>) -> Vec<u64> {
    let mut ans = Vec::new();
    let mut acc = digits[0];
    for i in 1..digits.len() {
        if commas[i - 1] {
            acc = 10 * acc + digits[i];
        } else {
            ans.push(acc);
            acc = digits[i];
        }
    }
    ans.push(acc);
    ans
}

pub fn euler118() -> String {
    let mut vec = Vec::new();
    for digits in (1..10).permutations(9) {
        for commas in (0..8).map(|_| vec![true, false]).multi_cartesian_product() {
            let mut numbers = split(&digits, &commas);
            numbers.sort();
            if numbers.iter().all(|&p| is_prime(p)) {
                vec.push(numbers);
            }
        }
    }
    vec.sort();
    vec.dedup();
    vec.len().to_string()
}
