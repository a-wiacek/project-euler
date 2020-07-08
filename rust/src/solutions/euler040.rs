pub fn euler040() -> String {
    let digits = (1..200000)
        .map(|n| n.to_string())
        .collect::<String>()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();
    vec![0, 9, 99, 999, 9999, 99999, 999999]
        .iter()
        .map(|&n| digits[n])
        .product::<usize>()
        .to_string()
}
