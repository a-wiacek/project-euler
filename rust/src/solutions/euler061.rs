use crate::utils::numeric::{digits::Digits, sequences::*};
use permutohedron::{control::Control, heap_recursive};

fn four_digits<T: Iterator<Item = usize>>(it: T) -> Vec<usize> {
    Box::new(it.skip_while(|&n| n < 1000).take_while(|&n| n < 10000)).collect()
}

fn glueable(a: usize, b: usize) -> bool {
    let da: Vec<usize> = a.digits();
    let db: Vec<usize> = b.digits();
    da[2] == db[0] && da[3] == db[1]
}

pub fn euler061() -> String {
    let polysts: Vec<Vec<usize>> = vec![
        four_digits(square::Square::new()),
        four_digits(pentagonal::Pentagonal::new()),
        four_digits(hexagonal::Hexagonal::new()),
        four_digits(heptagonal::Heptagonal::new()),
        four_digits(octogonal::Octogonal::new()),
    ];
    let ans = heap_recursive(&mut [0usize, 1, 2, 3, 4], |permutation| {
        for n1 in four_digits(triangular::Triangular::new()) {
            if let &mut [o1, o2, o3, o4, o5] = permutation {
                for &n2 in polysts[o1].iter().filter(|&&n2| glueable(n1, n2)) {
                    for &n3 in polysts[o2].iter().filter(|&&n3| glueable(n2, n3)) {
                        for &n4 in polysts[o3].iter().filter(|&&n4| glueable(n3, n4)) {
                            for &n5 in polysts[o4].iter().filter(|&&n5| glueable(n4, n5)) {
                                for &n6 in polysts[o5]
                                    .iter()
                                    .filter(|&&n6| glueable(n5, n6) && glueable(n6, n1))
                                {
                                    return Control::Break(n1 + n2 + n3 + n4 + n5 + n6);
                                }
                            }
                        }
                    }
                }
            } else {
                unreachable!("Permutation of vector with 5 elements does not have 5 elements!")
            }
        }
        return Control::Continue;
    });
    match ans {
        Control::Break(ans) => ans.to_string(),
        Control::Continue => unreachable!("None of the permutations worked"),
    }
}
