pub fn euler033() -> String {
    let mut num = 1;
    let mut den = 1;
    for a in 11..100 {
        for b in a + 1..100 {
            if b % 10 != 0 && a % 10 == b / 10 && a * (b % 10) == b * (a / 10) {
                num *= a;
                den *= b;
            }
        }
    }
    (den / num::integer::gcd(den, num)).to_string()
}
