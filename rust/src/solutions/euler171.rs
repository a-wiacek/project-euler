use cached::proc_macro::cached;
use num::integer::Roots;

// Input: How long number has to be,
//        what sum of squares of digits must it have,
//        can we have leading zero?
// Output: How many numbers are there and what is their sum?
#[cached]
fn count_nums(len: u32, sum_of_squares: u128, leading_zero_allowed: bool) -> (usize, u128) {
    if len == 1 {
        let sqrt = sum_of_squares.sqrt();
        let is_proper_square = sum_of_squares < 100 && sqrt.pow(2) == sum_of_squares;
        if is_proper_square && (leading_zero_allowed || sum_of_squares > 0) {
            (1, sqrt)
        } else {
            (0, 0)
        }
    } else {
        (if leading_zero_allowed { 0 } else { 1 }..10)
            .filter_map(|d| {
                if sum_of_squares < d * d {
                    None
                } else {
                    let (how_many, total_sum) = count_nums(len - 1, sum_of_squares - d * d, true);
                    Some((
                        how_many,
                        total_sum + d * how_many as u128 * 10u128.pow(len - 1),
                    ))
                }
            })
            .fold((0, 0), |(a, b), (c, d)| (a + c, b + d))
    }
}

pub fn euler171() -> String {
    (0..)
        .map(|x| x * x)
        .take_while(|&x| x <= 81 * 20)
        .map(|x| count_nums(20, x, true).1)
        .sum::<u128>()
        .rem_euclid(1_000_000_000)
        .to_string()
}
