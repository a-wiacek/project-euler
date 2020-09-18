use crate::solutions::euler089::minimum_form;
use itertools::Itertools;

// Let X be a random variable which gives value of rolled number.
// We have EX = 0,14 * (EX + 1000) + 0,86 * E[X | M was not rolled first]
// -> EX = 140 / 0,86 + E[X | M was not rolled first]

struct Numeral {
    decimal: usize,
    roman: String,
}

fn all_numerals() -> Vec<Numeral> {
    (0..1000)
        .map(|n| Numeral {
            decimal: n,
            roman: minimum_form(n) + "#",
        })
        .collect()
}

struct Tree {
    value: f64,
    children: Vec<Tree>,
}

fn build_suffix_tree(numerals: Vec<Numeral>) -> Tree {
    let grouped_numerals = numerals
        .into_iter()
        .group_by(|num| num.roman.chars().next().unwrap());
    let mut iterator = grouped_numerals.into_iter().map(|group| group.1);
    // The first group contains only one element: "#"
    let value = iterator.next().unwrap().next().unwrap().decimal as f64;
    let children = iterator
        .map(|group| {
            build_suffix_tree(
                group
                    .map(|Numeral { decimal, roman }| {
                        let mut chars = roman.chars();
                        chars.next();
                        Numeral {
                            decimal,
                            roman: chars.as_str().to_owned(),
                        }
                    })
                    .collect(),
            )
        })
        .collect();
    Tree { value, children }
}

impl Tree {
    fn expected_value(self) -> f64 {
        let Tree { value, children } = self;
        let w = 7.0 * children.len() as f64;
        (value + 7.0 * children.into_iter().map(Tree::expected_value).sum::<f64>()) / (w + 1.0)
    }
}

pub fn euler610() -> String {
    let mut numerals = all_numerals();
    numerals.sort_by(|a, b| a.roman.cmp(&b.roman));
    let the_expected_value = build_suffix_tree(numerals).expected_value();
    format!("{:.8}", 140.0 / 0.86 + the_expected_value)
}
