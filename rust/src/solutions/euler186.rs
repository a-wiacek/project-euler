use crate::utils::collections::find_union::FindUnion;

pub fn euler186() -> String {
    let people: usize = 1000000;
    let prime_minister = 524287;
    let mut table_flat: Vec<usize> = Vec::with_capacity(10000000);
    for k in 1..56 {
        table_flat.push((100003 + 300007 * k * k * k - 200003 * k) % people);
    }
    for k in 56..=10000000 {
        table_flat.push((table_flat[k - 25] + table_flat[k - 56]) % people);
    }
    let mut find_union = FindUnion::new(people);
    let mut failed_calls = 0;
    for i in 0.. {
        let caller = table_flat[2 * i];
        let called = table_flat[2 * i + 1];
        if caller != called {
            find_union.union(caller, called);
        } else {
            failed_calls += 1;
        }
        if find_union.size(prime_minister) >= people / 100 * 99 {
            return (i - failed_calls + 1).to_string();
        }
    }
    unreachable!()
}
