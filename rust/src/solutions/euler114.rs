pub fn euler114() -> String {
    let mut vec = vec![1u64; 51];
    for i in 3..51 {
        vec[i] += vec[i - 1];
        for j in 0..i - 3 {
            vec[i] += vec[j];
        }
    }
    vec[50].to_string()
}
