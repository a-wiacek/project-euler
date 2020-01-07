pub fn euler005() -> String {
    (1..21).fold(1, num::integer::lcm).to_string()
}