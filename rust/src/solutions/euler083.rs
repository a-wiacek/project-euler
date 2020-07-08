use crate::solutions::euler081::parse_matrix;
use crate::utils::input::get_input;
use itertools::Itertools;

pub fn euler083() -> String {
    let input = parse_matrix(&get_input(83));
    let l = input.len();
    let mut dist = vec![vec![1000000000usize; l]; l];
    let mut update = vec![vec![false; l]; l];
    dist[0][0] = input[0][0];
    update[0][0] = true;
    loop {
        match (0..l)
            .cartesian_product(0..l)
            .filter(|&(i, j)| update[i][j])
            .min_by_key(|&(i, j)| dist[i][j])
        {
            None => break dist[l - 1][l - 1].to_string(),
            Some((i, j)) => {
                update[i][j] = false;
                if j < l - 1 {
                    let right = dist[i][j] + input[i][j + 1];
                    if right < dist[i][j + 1] {
                        dist[i][j + 1] = right;
                        update[i][j + 1] = true;
                    }
                }
                if j > 0 {
                    let left = dist[i][j] + input[i][j - 1];
                    if left < dist[i][j - 1] {
                        dist[i][j - 1] = left;
                        update[i][j - 1] = true;
                    }
                }
                if i < l - 1 {
                    let down = dist[i][j] + input[i + 1][j];
                    if down < dist[i + 1][j] {
                        dist[i + 1][j] = down;
                        update[i + 1][j] = true;
                    }
                }
                if i > 0 {
                    let up = dist[i][j] + input[i - 1][j];
                    if up < dist[i - 1][j] {
                        dist[i - 1][j] = up;
                        update[i - 1][j] = true;
                    }
                }
            }
        }
    }
}
