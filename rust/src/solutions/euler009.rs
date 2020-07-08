pub fn euler009() -> String {
    for a in 3..334 {
        for b in (a + 1)..501 {
            let c = 1000 - a - b;
            if c > b && a * a + b * b == c * c {
                return (a * b * c).to_string();
            }
        }
    }
    unreachable!()
}
