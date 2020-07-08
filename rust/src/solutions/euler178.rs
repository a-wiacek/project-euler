use cached::proc_macro::cached;

// Input: How long number has to be,
//        what sum of squares of digits must it have,
//        can we have leading zero?
// Output: How many numbers are there and what is their sum?
#[cached]
fn count_nums(lowest_digit: u8, highest_digit: u8, length_left: usize, current_digit: u8) -> u64 {
    if length_left == 0 {
        if lowest_digit == 0 && highest_digit == 9 {
            1
        } else {
            0
        }
    } else {
        let go_down = if let Some(new_digit) = current_digit.checked_sub(1) {
            count_nums(
                std::cmp::min(lowest_digit, new_digit),
                highest_digit,
                length_left - 1,
                new_digit,
            )
        } else {
            0
        };
        let go_up = {
            let new_digit = current_digit + 1;
            if new_digit < 10 {
                count_nums(
                    lowest_digit,
                    std::cmp::max(highest_digit, new_digit),
                    length_left - 1,
                    new_digit,
                )
            } else {
                0
            }
        };
        go_down + go_up
    }
}

pub fn euler178() -> String {
    (10..=40)
        .map(|len| (1..10).map(|d| count_nums(d, d, len - 1, d)).sum::<u64>())
        .sum::<u64>()
        .to_string()
}
