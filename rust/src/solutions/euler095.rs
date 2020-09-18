use crate::utils::numeric::divisors_with_sieve::divisors;

pub fn euler095() -> String {
    let sieve = primal::Sieve::new(1000000);
    let mut sum_of_divisors = vec![None];
    for n in 1..=1000000 {
        sum_of_divisors.push(
            Some(divisors(&sieve, n).into_iter().sum::<usize>() - n).filter(|&s| s <= 1000000),
        );
    }
    // sum_of_divisors[i] contains sum of divisors of i
    // (value will be changed to None after first use)
    let mut max_chain_len = 0;
    let mut max_chain_min_elem = 0;
    for i in 2usize..=1000000 {
        let mut visited = vec![i];
        let mut last = i;
        loop {
            if let Some(next) = sum_of_divisors[last] {
                sum_of_divisors[last] = None;
                if let Some(prev) = visited.iter().position(|e| *e == next) {
                    let chain_len = visited.len() - prev;
                    let min_chain_elem = *visited[prev..].iter().min().unwrap();
                    if chain_len > max_chain_len {
                        max_chain_len = chain_len;
                        max_chain_min_elem = min_chain_elem;
                        break;
                    }
                } else {
                    visited.push(next);
                    last = next;
                }
            } else {
                break;
            }
        }
    }
    max_chain_min_elem.to_string()
}
