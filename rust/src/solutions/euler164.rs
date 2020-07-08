pub fn euler164() -> String {
    // [length left - 2][first digit][second digit]
    let mut arr = vec![vec![vec![0u64; 10]; 10]; 19];
    for i in 0..10 {
        for j in 0..10 - i {
            arr[0][i][j] = 1;
        }
    }
    for s in 1..19 {
        for i in 0..10 {
            for j in 0..10 - i {
                let plus = arr[s - 1][i][j];
                for d in 0..10 - i - j {
                    arr[s][d][i] += plus;
                }
            }
        }
    }
    let mut ans = 0;
    for i in 1..10 {
        for j in 0..10 {
            ans += arr[18][i][j];
        }
    }
    ans.to_string()
}
