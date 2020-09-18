#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
struct Rect {
    lower: Point,
    upper: Point,
}

impl Point {
    fn split(self) -> Rect {
        let Point { x, y } = self;
        let mid = (x + y) / 2;
        Rect {
            lower: Point { x, y: mid },
            upper: Point { x: mid + 1, y },
        }
    }
}

fn f(n: u32) -> usize {
    let mut ans = 0;
    let d = 2i32.pow(n - 1);
    let rect = Rect {
        lower: Point { x: 0, y: d - 1 },
        upper: Point { x: d, y: d + d - 1 },
    };
    let is_black = |x, y| {
        let a = (x - d) as i64;
        let b = (y - d) as i64;
        let e = d as i64;
        (a * a + b * b <= e * e) as u8
    };
    let mut queue = vec![(rect, rect)];
    while !queue.is_empty() {
        ans += queue.len();
        queue = queue
            .into_iter()
            .flat_map(
                |(
                    Rect {
                        lower: l1,
                        upper: u1,
                    },
                    Rect {
                        lower: l2,
                        upper: u2,
                    },
                )| {
                    let mut vec = vec![];
                    for (p1, p2) in &[(l1, u1), (l2, u1), (l1, u2), (l2, u2)] {
                        let corners_count = is_black(p1.x, p1.y)
                            + is_black(p2.x, p1.y)
                            + is_black(p1.x, p2.y)
                            + is_black(p2.x, p2.y);
                        match corners_count {
                            0 | 4 => ans += 2,
                            _ => vec.push((p1.split(), p2.split())),
                        }
                    }
                    vec
                },
            )
            .collect();
    }
    ans
}

pub fn euler287() -> String {
    f(24).to_string()
}
