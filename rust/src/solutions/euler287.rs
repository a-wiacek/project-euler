#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug)]
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

fn go<F>(is_black: &F, rect1: Rect, rect2: Rect) -> usize
where
    F: Fn(i32, i32) -> u8,
{
    let Rect {
        lower: l1,
        upper: u1,
    } = rect1;
    let Rect {
        lower: l2,
        upper: u2,
    } = rect2;
    1 + count(is_black, l1, l2)
        + count(is_black, l1, u2)
        + count(is_black, u1, l2)
        + count(is_black, u1, u2)
}

fn count<F>(is_black: &F, p1: Point, p2: Point) -> usize
where
    F: Fn(i32, i32) -> u8,
{
    let corners_count =
        is_black(p1.x, p2.x) + is_black(p1.x, p2.y) + is_black(p1.y, p2.x) + is_black(p1.y, p2.y);
    match corners_count {
        0 | 4 => 2,
        _ => go(is_black, p1.split(), p2.split()),
    }
}

fn f(n: u32) -> usize {
    let d = 2i32.pow(n - 1);
    let rect = Rect {
        lower: Point { x: 0, y: d - 1 },
        upper: Point { x: d, y: d + d - 1 },
    };
    let is_black = |x: i32, y: i32| {
        let a = (x - d) as i64;
        let b = (y - d) as i64;
        let e = d as i64;
        (a * a + b * b <= e * e) as u8
    };
    go(&is_black, rect, rect)
}

pub fn euler287() -> String {
    f(24).to_string()
}
