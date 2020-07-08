use num::Integer;
use std::cmp::{max, min};

fn m(n: i64) -> i64 {
    n.mod_floor(&1000000) - 500000
}

fn max_subsequence_sum(line: &Vec<i64>) -> i64 {
    let mut l = line.clone();
    for i in 1..l.len() {
        if l[i - 1] > 0 {
            l[i] += l[i - 1];
        }
    }
    l.into_iter().max().unwrap()
}

fn horizontal_max(table: &Vec<Vec<i64>>) -> i64 {
    table.iter().map(max_subsequence_sum).max().unwrap()
}

fn vertical_max(table: &Vec<Vec<i64>>) -> i64 {
    (0..2000)
        .map(|col| max_subsequence_sum(&(0..2000).map(|row| table[row][col]).collect()))
        .max()
        .unwrap()
}

fn diagonal_max(table: &Vec<Vec<i64>>) -> i64 {
    let ld_ru = (0..=3998) // left down -> right up
        .map(|diag| {
            let mut diag_row: Vec<i64> = Vec::new();
            for i in max(1999, diag) - 1999..=min(diag, 1999) {
                diag_row.push(table[diag - i][i]);
            }
            max_subsequence_sum(&diag_row)
        })
        .max()
        .unwrap();
    let lu_rd = (0..=3998) // left up -> right down
        .map(|diag| {
            let mut diag_row: Vec<i64> = Vec::new();
            for i in max(1999, diag) - 1999..=min(diag, 1999) {
                diag_row.push(table[1999 + i - diag][i])
            }
            max_subsequence_sum(&diag_row)
        })
        .max()
        .unwrap();
    max(ld_ru, lu_rd)
}

pub fn euler149() -> String {
    let mut table_flat: Vec<i64> = Vec::new();
    for k in 1..56 {
        table_flat.push(m(100003 - 200003 * k + 300007 * k * k * k));
    }
    for k in 56..=4000000 {
        table_flat.push(m(table_flat[k - 25] + table_flat[k - 56]));
    }
    let table: Vec<Vec<i64>> = table_flat.chunks(2000).map(|x| x.to_vec()).collect();
    [horizontal_max, vertical_max, diagonal_max]
        .iter()
        .map(|f| f(&table))
        .max()
        .unwrap()
        .to_string()
}
