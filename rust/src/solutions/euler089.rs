use crate::utils::input::get_input;

pub fn minimum_form(n: usize) -> String {
    match n {
        _ if n >= 1000 => String::from("M") + &minimum_form(n - 1000),
        _ if n >= 900 => String::from("CM") + &minimum_form(n - 900),
        _ if n >= 500 => String::from("D") + &minimum_form(n - 500),
        _ if n >= 400 => String::from("CD") + &minimum_form(n - 400),
        _ if n >= 100 => String::from("C") + &minimum_form(n - 100),
        _ if n >= 90 => String::from("XC") + &minimum_form(n - 90),
        _ if n >= 50 => String::from("L") + &minimum_form(n - 50),
        _ if n >= 40 => String::from("XL") + &minimum_form(n - 40),
        _ if n >= 10 => String::from("X") + &minimum_form(n - 10),
        _ => String::from(["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"][n]),
    }
}

fn read_roman(s: &[char]) -> usize {
    match s {
        ['M', ..] => 1000 + read_roman(&s[1..]),
        ['C', 'M', ..] => 900 + read_roman(&s[2..]),
        ['D', ..] => 500 + read_roman(&s[1..]),
        ['C', 'D', ..] => 400 + read_roman(&s[2..]),
        ['C', ..] => 100 + read_roman(&s[1..]),
        ['X', 'C', ..] => 90 + read_roman(&s[2..]),
        ['L', ..] => 50 + read_roman(&s[1..]),
        ['X', 'L', ..] => 40 + read_roman(&s[2..]),
        ['X', ..] => 10 + read_roman(&s[1..]),
        ['I', 'X', ..] => 9 + read_roman(&s[2..]),
        ['V', ..] => 5 + read_roman(&s[1..]),
        ['I', 'V', ..] => 4 + read_roman(&s[2..]),
        ['I', ..] => 1 + read_roman(&s[1..]),
        [] => 0,
        _ => panic!("Invalid roman number"),
    }
}

pub fn euler089() -> String {
    get_input(89)
        .lines()
        .map(|line| {
            line.len() - minimum_form(read_roman(&line.chars().collect::<Vec<char>>()[..])).len()
        })
        .sum::<usize>()
        .to_string()
}
