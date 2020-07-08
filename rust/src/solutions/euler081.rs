use crate::utils::input::get_input;

pub fn parse_matrix(s: &str) -> Vec<Vec<usize>> {
    s.lines()
        .map(|line| {
            line.split(',')
                .map(|c| c.parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}

pub fn euler081() -> String {
    let mut arr = parse_matrix(&get_input(81));
    let l = arr.len();
    for i in 1..l {
        arr[i][0] += arr[i - 1][0];
        arr[0][i] += arr[0][i - 1];
    }
    for i in 1..l {
        for j in 1..l {
            arr[i][j] += std::cmp::min(arr[i - 1][j], arr[i][j - 1]);
        }
    }
    arr[l - 1][l - 1].to_string()
}
