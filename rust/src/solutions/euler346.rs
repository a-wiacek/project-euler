use std::iter::successors;

pub fn euler346() -> String {
    let bound = 1_000_000_000_000;
    let f = |a: Option<u64>| a.filter(|a| a < &bound);
    let mut ans = (2u64..=1_000_000)
        .flat_map(|n| {
            successors(f(Some(n * n + n + 1)), move |a| {
                f(a.checked_mul(n).map(|a| a + 1))
            })
        })
        .collect::<Vec<_>>();
    ans.sort();
    ans.dedup();
    (1 + ans.into_iter().sum::<u64>()).to_string()
}
