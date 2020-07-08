pub fn euler115() -> String {
    let mut vec = vec![1u64; 201];
    for i in 50..201 {
        vec[i] += vec[i - 1];
        for j in 0..i - 50 {
            vec[i] += vec[j];
        }
    }
    vec.into_iter()
        .position(|x| x > 1000000)
        .unwrap()
        .to_string()
}
