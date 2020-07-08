use permutohedron::LexicalPermutation;

pub fn euler024() -> String {
    let mut vec: Vec<char> = "0123456789".chars().collect();
    for _ in 0..999999 {
        vec.next_permutation();
    }
    vec.iter().collect()
}
