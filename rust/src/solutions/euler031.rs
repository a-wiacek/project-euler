pub fn euler031() -> String {
    let mut arr = vec![vec![0; 8]; 201];
    let coins = [1, 2, 5, 10, 20, 50, 100, 200];
    for i in 0..=200 {
        arr[i][0] = 1;
    }
    for j in 1..coins.len() {
        for i in 0..coins[j] {
            arr[i][j] = arr[i][j - 1];
        }
        for i in coins[j]..=200 {
            arr[i][j] = arr[i - coins[j]][j] + arr[i][j - 1];
        }
    }
    arr[200][7].to_string()
}
