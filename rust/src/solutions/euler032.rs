use crate::utils::numeric::digits::Digits;

pub fn euler032() -> String {
    let mut vec = Vec::new();
    for a in 1..100 {
        for b in a + 1..4000 {
            let digits_a = a.digits_count();
            let digits_b = b.digits_count();
            let ab = a * b;
            let digits_ab = ab.digits_count();
            if (0..10)
                .all(|d| digits_a[d] + digits_b[d] + digits_ab[d] == if d > 0 { 1 } else { 0 })
            {
                vec.push(ab);
            }
        }
    }
    vec.sort();
    vec.dedup();
    vec.into_iter().sum::<usize>().to_string()
}
