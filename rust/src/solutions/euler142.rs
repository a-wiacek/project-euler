use num::integer::Roots;

pub fn euler142() -> String {
    let bound = 1000i32;
    for a in 1..bound {
        for b in 1..bound {
            if (a - b) % 2 == 0 {
                let x = (a * a + b * b) / 2;
                let y = a * a - x;
                for c in 1..bound {
                    let z = c * c - x;
                    if [x - y, y - z, z].iter().all(|&x| x > 0)
                        && [x + z, x - z, y + z, y - z]
                            .iter()
                            .all(|&x| x.sqrt().pow(2) == x)
                    {
                        return (x + y + z).to_string();
                    }
                }
            }
        }
    }
    unreachable!()
}
