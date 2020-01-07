use crate::utils::input::get_input;

fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|w| w.parse().unwrap())
        .collect()
}

fn horizontal_max(array: &Vec<Vec<i32>>) -> i32 {
    array
        .iter()
        .map(|row| {
            row.windows(4)
                .map(|w| w.into_iter().fold(1, |a, b| a * b))
                .fold(0, std::cmp::max)
        })
        .fold(0, std::cmp::max)
}

fn vertical_max(array: &Vec<Vec<i32>>) -> i32 {
    let mut max = 0;
    for row in 0..array.len() {
        for column in 0..(array.len() - 4) {
            let p = array[row][column]
                * array[row][column + 1]
                * array[row][column + 2]
                * array[row][column + 3];
            max = if p > max { p } else { max }
        }
    }
    max
}

fn diagonal_max(array: &Vec<Vec<i32>>) -> i32 {
    let mut max = 0;
    for row in 0..(array.len() - 4) {
        for column in 0..(array.len() - 4) {
            let p = array[row][column]
                * array[row + 1][column + 1]
                * array[row + 2][column + 2]
                * array[row + 3][column + 3];
            max = if p > max { p } else { max }
        }
    }
    for row in 3..array.len() {
        for column in 0..(array.len() - 4) {
            let p = array[row][column]
                * array[row - 1][column + 1]
                * array[row - 2][column + 2]
                * array[row - 3][column + 3];
            max = if p > max { p } else { max }
        }
    }
    max
}

pub fn euler011() -> String {
    let array: Vec<Vec<i32>> = get_input(11).lines().map(parse_line).collect();
    [horizontal_max, vertical_max, diagonal_max]
        .iter()
        .map(|f| f(&array))
        .fold(0, std::cmp::max)
        .to_string()
}
