// http://oeis.org/A215721
// Found using simulation
// The most important line is:
// For n > 14, a(n) = a(n - 5) + 34

fn f(n: usize) -> usize {
    let init_seq = vec![1usize, 5, 9, 15, 21, 25, 29, 35];
    let period = vec![39usize, 43, 55, 59, 63];
    let and_then = (0..)
        .step_by(34)
        .flat_map(|s| period.iter().map(move |&t| s + t));
    let total_seq = init_seq.into_iter().chain(and_then);
    n - total_seq.take_while(|&x| x <= n).count()
}

pub fn euler306() -> String {
    f(1_000_000).to_string()
}
