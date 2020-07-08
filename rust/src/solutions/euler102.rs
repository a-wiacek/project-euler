use crate::utils::input::get_input;

type Point = (i64, i64);

fn area((xa, ya): Point, (xb, yb): Point, (xc, yc): Point) -> i64 {
    (xa * (yb - yc) + xb * (yc - ya) + xc * (ya - yb)).abs()
}

fn verify(a: Point, b: Point, c: Point) -> bool {
    let o = (0, 0);
    area(a, b, c) == area(a, b, o) + area(a, o, c) + area(o, b, c)
}

pub fn euler102() -> String {
    get_input(102)
        .lines()
        .filter(|line| {
            match line
                .split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()[..]
            {
                [q, w, e, r, t, y] => verify((q, w), (e, r), (t, y)),
                _ => panic!("Line should have 6 numbers"),
            }
        })
        .count()
        .to_string()
}
