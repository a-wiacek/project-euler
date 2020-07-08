fn count_solutions(n: usize) -> usize {
    let mut ans = 0;
    for a in 1..n {
        for b in a + 1..n - a + 1 {
            let c = n - a - b;
            if a * a + b * b == c * c {
                ans += 1;
            }
        }
    }
    ans
}

pub fn euler039() -> String {
    (1..=1000)
        .max_by_key(|&n| count_solutions(n))
        .unwrap()
        .to_string()
}
