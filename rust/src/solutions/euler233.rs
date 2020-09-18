use std::iter::successors;

pub fn euler233() -> String {
    let bound = 10usize.pow(11);
    let qmin = bound / (5 * 5 * 5 * 13 * 13 * 17);
    let (primes_4_1, primes_4_23) = primal::Sieve::new(bound / 216)
        .primes_from(0)
        .partition::<Vec<usize>, _>(|&p| p % 4 == 1);
    let q_product = {
        let mut vec =
            primes_4_23
                .into_iter()
                .take_while(|&p| p < qmin)
                .fold(vec![1usize], |mut vec, p| {
                    let mut new = Vec::new();
                    for &prod in &vec {
                        let mul = |b| {
                            let q = p * b;
                            if q <= qmin {
                                Some(q)
                            } else {
                                None
                            }
                        };
                        for k in successors(mul(prod), |&prod| mul(prod)) {
                            new.push(k);
                        }
                    }
                    vec.append(&mut new);
                    vec
                });
        vec.sort();
        vec
    };
    let mut ans = 0;

    // Case 1
    for &p1 in primes_4_1.iter().take_while(|p| p.pow(7) <= bound) {
        let p1_7 = p1.pow(7);
        for &p2 in primes_4_1.iter().take_while(|p| p1_7 * p.pow(3) <= bound) {
            if p1 != p2 {
                let p2_3 = p2.pow(3);
                let p = p1_7 * p2_3;
                for &q in q_product.iter().take_while(|&&qs| qs * p <= bound) {
                    ans += p * q;
                }
            }
        }
    }

    // Case 2
    for &p1 in primes_4_1.iter().take_while(|p| p.pow(10) <= bound) {
        let p1_10 = p1.pow(10);
        for &p2 in primes_4_1.iter().take_while(|p| p1_10 * p.pow(2) <= bound) {
            if p1 != p2 {
                let p2_2 = p2.pow(2);
                let p = p1_10 * p2_2;
                for &q in q_product.iter().take_while(|&&qs| qs * p <= bound) {
                    ans += p * q;
                }
            }
        }
    }

    // Case 3
    for &p1 in primes_4_1.iter().take_while(|p| p.pow(3) <= bound) {
        let p1_3 = p1.pow(3);
        for &p2 in primes_4_1.iter().take_while(|p| p1_3 * p.pow(2) <= bound) {
            if p1 != p2 {
                let p2_2 = p2.pow(2);
                let zu: Vec<_> = primes_4_1
                    .iter()
                    .take_while(|&&p| p1_3 * p2_2 * p <= bound)
                    .collect();
                println!("{} {} {}", p1, p2, zu.len());
                for &p3 in zu {
                    if p1 != p3 && p2 != p3 {
                        let p = p1_3 * p2_2 * p3;
                        for &q in q_product.iter().take_while(|&&qs| qs * p <= bound) {
                            ans += p * q;
                        }
                    }
                }
            }
        }
    }

    ans.to_string()
}
