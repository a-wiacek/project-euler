use crate::utils::input::get_input;
use itertools::Itertools;

pub fn euler059() -> String {
    let encrypted_ascii = get_input(59)
        .split(',')
        .map(|c| c.parse::<u8>().unwrap())
        .collect_vec();
    let word = "Euler".as_bytes();
    let the_key = (0..3)
        .map(|_| 'a' as u8..='z' as u8)
        .multi_cartesian_product()
        .find(|key| {
            encrypted_ascii
                .iter()
                .zip(key.iter().cycle())
                .map(|(a, &b)| a ^ b)
                .collect_vec()
                .windows(5)
                .any(|x| x == word)
        })
        .unwrap();
    encrypted_ascii
        .iter()
        .zip(the_key.iter().cycle())
        .map(|(a, &b)| (a ^ b) as usize)
        .sum::<usize>()
        .to_string()
}
