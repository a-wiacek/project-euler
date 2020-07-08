pub fn euler028() -> String {
    let mut sum = 1;
    let mut step = 2;
    let mut curr = 1;
    for _ in 0..500 {
        for _ in 0..4 {
            curr += step;
            sum += curr;
        }
        step += 2;
    }
    sum.to_string()
}
