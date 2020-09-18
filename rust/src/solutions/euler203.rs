pub fn euler203() -> String {
    let mut vec = vec![1];
    let sieve = primal::Sieve::new(50);
    for x in 0..=50 {
        for y in 1..x {
            let b = num::integer::binomial(x, y);
            if sieve.factor(b).unwrap().into_iter().all(|x| x.1 == 1) {
                vec.push(b);
            }
        }
    }
    vec.sort();
    vec.dedup();
    vec.into_iter().sum::<usize>().to_string()
}
