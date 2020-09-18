pub fn euler201() -> String {
    let squares = (1..=100).map(|x| x * x).collect::<Vec<usize>>();
    let n = squares.len();
    let sum = squares.iter().sum();
    let mut arr = vec![vec![0u128; n + 1]; sum + 1];
    arr[0][0] = 1;
    for sq in squares {
        for x in (sq..=sum).rev() {
            for y in (1..=n).rev() {
                arr[x][y] += arr[x - sq][y - 1];
            }
        }
    }
    (0..=sum)
        .map(|i| if arr[i][50] == 1 { i } else { 0 })
        .sum::<usize>()
        .to_string()
}
