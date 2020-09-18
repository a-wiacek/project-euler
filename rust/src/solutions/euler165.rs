use ordered_float::OrderedFloat;
type Point = (f64, f64);
type OrderedPoint = (OrderedFloat<f64>, OrderedFloat<f64>);
type Line = (Point, Point);

const EPS: f64 = 10e-12;

fn line_through_points(((xb, yb), (xe, ye)): Line) -> (f64, f64, f64) {
    (ye - yb, xb - xe, yb * xe - xb * ye)
}

fn point_in_line(((xb, yb), (xe, ye)): Line, (xm, ym): Point) -> bool {
    let in01 = |x| 0. < x && x < 1.;
    if xb == xe {
        if yb == ye {
            false
        } else {
            xm == xe && in01((ym - ye) / (yb - ye))
        }
    } else {
        let tx = (xm - xe) / (xb - xe);
        in01(tx)
            && if yb == ye {
                ym == ye
            } else {
                (tx - (ym - ye) / (yb - ye)).abs() < EPS
            }
    }
}

fn intersection(line1: Line, line2: Line) -> Option<Point> {
    let (a1, b1, c1) = line_through_points(line1);
    let (a2, b2, c2) = line_through_points(line2);
    let w = a1 * b2 - a2 * b1;
    if w == 0. {
        None
    } else {
        let wx = b1 * c2 - b2 * c1;
        let wy = c1 * a2 - c2 * a1;
        let p = (wx / w, wy / w);
        if point_in_line(line1, p) && point_in_line(line2, p) {
            Some(p)
        } else {
            None
        }
    }
}

pub fn euler165() -> String {
    let mut s = vec![290797u64];
    for i in 0..20000 {
        s.push(s[i] * s[i] % 50515093);
    }
    s.remove(0);
    let lines: Vec<Line> = s
        .chunks(4)
        .map(|chunk| {
            let f = |n| (chunk[n] % 500) as f64;
            ((f(0), f(1)), (f(2), f(3)))
        })
        .collect();
    let mut points = Vec::<OrderedPoint>::new();
    for i in 0..lines.len() {
        for j in i + 1..lines.len() {
            if let Some(point) = intersection(lines[i], lines[j]) {
                points.push((OrderedFloat(point.0), OrderedFloat(point.1)));
            }
        }
    }
    points.sort();
    points.dedup();
    points.len().to_string()
}
