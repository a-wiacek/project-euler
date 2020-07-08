use crate::utils::input::get_input;

fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|w| w.parse().unwrap())
        .collect()
}

fn parse_input(num: usize) -> Vec<Vec<i32>> {
    get_input(num).lines().map(parse_line).collect()
}

fn find_optimal_path(mut triangle: Vec<Vec<i32>>) -> i32 {
    for i in (0..triangle.len() - 1).rev() {
        for j in 0..=i {
            triangle[i][j] += std::cmp::max(triangle[i + 1][j], triangle[i + 1][j + 1]);
        }
    }
    triangle[0][0]
}

pub fn solve_triangle(num: usize) -> String {
    find_optimal_path(parse_input(num)).to_string()
}

pub fn euler018() -> String {
    solve_triangle(18)
}
