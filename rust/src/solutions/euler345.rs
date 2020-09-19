pub fn euler345() -> String {
    let input: Vec<Vec<u16>> = crate::utils::input::get_input(345)
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();
    let n = input.len();
    let mut f = vec![vec![0; 1 << n]; n + 1];
    for i in 0..n {
        for b in 0..1 << n {
            for j in 0..n {
                if ((b >> j) & 1) == 0 {
                    let k = f[i][b];
                    let at = &mut f[i + 1][b | (1 << j)];
                    *at = std::cmp::max(*at, k + input[i][j]);
                }
            }
        }
    }
    f[n][(1 << n) - 1].to_string()
}
