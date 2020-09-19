use crate::utils::numeric::linear_recurrence_relation::matrix_pow_mod;
use nalgebra::*;
use num::CheckedMul;

// Unlike in linear_recurrence_relation.rs, here we have
// two recursive sequences, so we can't use LRR.

pub fn euler377() -> String {
    let initial_values = DVector::<i64>::from_vec(vec![
        261994131, 23817625, 2165227, 196833, 17891, 1625, 147, 13, 1, // initial f values
        256, 128, 64, 32, 16, 8, 4, 2, 1, // initial g values
    ]);
    let recurrence_matrix = DMatrix::<i64>::from_fn(18, 18, |row, col| {
        if row == 0 && col <= 8 {
            10
        } else if row == 0 {
            col as i64 - 8
        } else if row == 9 && col >= 9 {
            1
        } else if row != 9 && row.checked_sub(col) == Some(1) {
            1
        } else {
            0
        }
    });
    let p = 1_000_000_000;
    let f = |n: usize| {
        if n <= 9 {
            initial_values[9 - n]
        } else {
            (&matrix_pow_mod(recurrence_matrix.clone(), n - 9, p) * &initial_values)[0] % p
        }
    };
    std::iter::successors(Some(13), |i| i.checked_mul(&13))
        .take(17)
        .map(f)
        .sum::<i64>()
        .rem_euclid(p)
        .to_string()
}
