use crate::utils::numeric::fibonacci::Fibonacci;

pub fn euler002() -> String {
    Fibonacci::new(1, 1)
        .take_while(|&n| n < 4000000)
        .filter(num::Integer::is_even)
        .sum::<i32>()
        .to_string()
}
