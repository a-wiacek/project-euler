pub fn euler073() -> String {
    let mut ans = -2; // -2, since this counts 1/2 and 1/3
    for den in 2..=12000 {
        for num in (den + 2) / 3..=den / 2 {
            if num::integer::gcd(den, num) == 1 {
                ans += 1;
            }
        }
    }
    ans.to_string()
}
