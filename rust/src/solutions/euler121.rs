use cached::proc_macro::cached;

#[cached]
fn p(turn: u64, bound: u64, blue: u64) -> f64 {
    if blue == bound / 2 + 1 {
        1.0
    } else if turn == bound {
        0.0
    } else {
        let balls = turn as f64 + 2.0;
        (p(turn + 1, bound, blue + 1) + (balls - 1.0) * p(turn + 1, bound, blue)) / balls
    }
}

pub fn euler121() -> String {
    (1.0 / p(0, 15, 0)).floor().to_string()
}
