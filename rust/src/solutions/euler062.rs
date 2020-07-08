use crate::utils::numeric::digits::Digits;
use std::collections::HashMap;

pub fn euler062() -> String {
    let mut cubes: HashMap<Vec<usize>, Vec<usize>> = HashMap::new();
    for n in 400usize..20000 {
        let mut cube_digits = (n * n * n).digits();
        cube_digits.sort();
        cubes.entry(cube_digits).or_insert(Vec::new()).push(n);
    }
    cubes
        .into_iter()
        .flat_map(|(_, preimage)| {
            if preimage.len() == 5 {
                Some(preimage[0])
            } else {
                None
            }
        })
        .min()
        .unwrap()
        .pow(3)
        .to_string()
}
