pub fn euler181() -> String {
    let b_max = 60;
    let w_max = 40;
    let mut arr = vec![vec![0u64; w_max + 1]; b_max + 1];
    arr[0][0] = 1;
    for black in 0..=b_max {
        for white in 0..=w_max {
            if black + white > 0 {
                for i in black..=b_max {
                    for j in white..=w_max {
                        arr[i][j] += arr[i - black][j - white];
                    }
                }
            }
        }
    }
    arr[b_max][w_max].to_string()
}
