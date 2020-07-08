pub fn euler117() -> String {
    let mut vec = vec![1u64; 51];
    vec[2] = 2;
    vec[3] = 4;
    for i in 4..51 {
        vec[i] = vec[i - 1] + vec[i - 2] + vec[i - 3] + vec[i - 4];
    }
    vec[50].to_string()
}
