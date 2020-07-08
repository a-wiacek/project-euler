pub fn euler174() -> String {
    let mut arr = vec![0; 1000000];
    for l in 2..2500001 {
        for t in (1..(l + 1) / 2).take_while(|&t| 4 * t * (l - t) < 1000000) {
            arr[4 * t * (l - t)] += 1;
        }
    }
    arr.into_iter()
        .filter(|&n| n >= 1 && n <= 10)
        .count()
        .to_string()
}
