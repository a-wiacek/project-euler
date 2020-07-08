pub fn euler007() -> String {
    primal::StreamingSieve::nth_prime(10001).to_string()
}
