use crate::utils::input::get_input;
use crate::utils::numeric::digits::undigits;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

struct Substitution {
    substitution: HashMap<char, usize>,
}

impl Substitution {
    fn is_mapped_to_zero(&self, c: &char) -> bool {
        self.substitution.get(c).map_or(false, |&v| v == 0)
    }

    fn square_anagrams(&self, squares: &HashSet<usize>, s1: &String, s2: &String) -> Option<usize> {
        let n1 = undigits(
            &s1.chars()
                .map(|c| *self.substitution.get(&c).unwrap())
                .collect(),
        );
        let n2 = undigits(
            &s2.chars()
                .map(|c| *self.substitution.get(&c).unwrap())
                .collect(),
        );
        if squares.contains(&n1) && squares.contains(&n2) {
            Some(std::cmp::max(n1, n2))
        } else {
            None
        }
    }

    fn generate_substitutions(
        trailing_letters: &Vec<char>,
        all_letters: &Vec<char>,
    ) -> Vec<Substitution> {
        (0..10)
            .combinations(all_letters.len())
            .flat_map(|digits_subset| {
                let l = digits_subset.len();
                digits_subset.into_iter().permutations(l)
            })
            .map(|digits_order| Substitution {
                substitution: all_letters
                    .clone()
                    .into_iter()
                    .zip(digits_order.into_iter())
                    .collect(),
            })
            .filter(|substitution| {
                trailing_letters
                    .iter()
                    .all(|c| !substitution.is_mapped_to_zero(c))
            })
            .collect()
    }
}

fn find_square(squares: &HashSet<usize>, s1: &String, s2: &String) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }
    let mut sorted_s1: Vec<char> = s1.chars().collect();
    sorted_s1.sort();
    let mut sorted_s2: Vec<char> = s2.chars().collect();
    sorted_s2.sort();
    let mut all_letters = sorted_s2.clone();
    all_letters.dedup();
    if sorted_s1 == sorted_s2 && all_letters.len() < 10 {
        Substitution::generate_substitutions(
            &vec![s1.chars().next().unwrap(), s2.chars().next().unwrap()],
            &all_letters,
        )
        .into_iter()
        .flat_map(|substitution| substitution.square_anagrams(squares, s1, s2))
        .max()
    } else {
        None
    }
}

pub fn euler098() -> String {
    let squares: HashSet<usize> = (1..10000000).map(|n| n * n).collect();
    let words: Vec<String> = get_input(98)
        .split(',')
        .map(|l| l.trim_matches('"').to_string())
        .collect();
    (0..words.len())
        .flat_map(|i| (i + 1..words.len()).map(move |j| (i, j)))
        .flat_map(|(i, j)| find_square(&squares, &words[i], &words[j]))
        .max()
        .unwrap()
        .to_string()
}
