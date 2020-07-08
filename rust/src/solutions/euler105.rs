use crate::solutions::euler103::test_set;
use crate::utils::input::get_input;

pub fn euler105() -> String {
    get_input(105)
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .flat_map(|vec| {
            if test_set(&vec) {
                Some(vec.into_iter().sum::<u32>())
            } else {
                None
            }
        })
        .sum::<u32>()
        .to_string()
}
