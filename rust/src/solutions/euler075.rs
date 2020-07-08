use std::collections::HashMap;

pub fn euler075() -> String {
    let mut vec: HashMap<usize, usize> = HashMap::new();
    for m in 2usize..=1000 {
        for n in 1..m {
            if (m + n) % 2 == 1 && num::integer::gcd(m, n) == 1 {
                let perimeter = 2 * m * (m + n);
                for k in 1..=1500000 / perimeter {
                    let count = vec.entry(perimeter * k).or_insert(0);
                    *count += 1;
                }
            }
        }
    }
    vec.into_iter().filter(|(_, v)| *v == 1).count().to_string()
}
