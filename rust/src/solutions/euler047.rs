use primal::Sieve;

pub fn euler047() -> String {
    let sieve = Sieve::new(700000);
    let counters: Vec<usize> = (2..700000)
        .map(|n| sieve.factor(n).unwrap().len())
        .collect();
    let mut i = 2;
    let mut s = 0;
    loop {
        if counters[i - 2] == 4 {
            s += 1;
        } else {
            s = 0;
        }
        if s == 4 {
            break (i - 3).to_string();
        }
        i += 1;
    }
}
