use cached::proc_macro::cached;

#[cached]
fn collatz_length(n: i64) -> i64 {
    match n {
        1 => 0,
        n if n & 1 == 1 => 1 + collatz_length(3 * n + 1),
        _ => 1 + collatz_length(n >> 1),
    }
}

pub fn euler014() -> String {
    (1..1000000)
        .max_by_key(|&n| collatz_length(n))
        .unwrap()
        .to_string()
}
