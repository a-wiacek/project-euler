use std::f64::consts::PI;

struct Point(i64, i64);

struct Circle {
    center: Point,
    radius: i64,
}

const POINT_E: Point = Point(3000, 1500);

const CIRCLE_BOUND: Circle = Circle {
    center: POINT_E,
    radius: 19000,
};

impl Circle {
    fn lattice_points(&self) -> impl Iterator<Item = Point> {
        let Point(cx, cy) = self.center;
        let r = self.radius;
        (cx - r..=cx + r).flat_map(move |x| {
            let t = ((r * r - (cx - x).pow(2)) as f64).sqrt().floor() as i64;
            (cy - t..=cy + t).map(move |y| Point(x, y))
        })
    }
}

fn dist((a, b): (f64, f64), (x, y): (f64, f64)) -> f64 {
    ((a - x).powi(2) + (b - y).powi(2)).sqrt()
}

fn minus((a, b): (f64, f64), (x, y): (f64, f64)) -> (f64, f64) {
    (a - x, b - y)
}

fn dot((a, b): (f64, f64), (x, y): (f64, f64)) -> f64 {
    a * x + b * y
}

fn ellipse(t: f64) -> (f64, f64) {
    let (sin, cos) = t.sin_cos();
    (7500.0 * cos + 3000.0, 2500.0 * 5f64.sqrt() * sin + 1500.0)
}

impl Point {
    fn outside_of_ellipse(&self) -> bool {
        5 * (self.0 - 3000).pow(2) + 9 * (self.1 - 1500).pow(2) > 5 * 7500 * 7500
    }

    fn proper_angle(&self) -> bool {
        let &Point(x, y) = self;
        let u = ((5 * x * x + 9 * y * y - 30000 * x - 27000 * y - 216000000) as f64).sqrt();
        let s = (3 * y - 4500) as f64;
        let d = 5f64.sqrt() * (x + 4500) as f64;
        let z1 = ellipse(if d == 0.0 {
            PI
        } else {
            -2.0 * ((u - s) / d).atan()
        });
        let z2 = ellipse(if d == 0.0 {
            2.0 * (2500.0 * 5f64.sqrt() / (y - 1500) as f64).atan()
        } else {
            2.0 * ((u + s) / d).atan()
        });
        let p = (x as f64, y as f64);
        let cos_phi = dot(minus(z1, p), minus(z2, p)) / (dist(z1, p) * dist(z2, p));
        cos_phi < 0.5f64.sqrt()
    }
}

pub fn euler246() -> String {
    CIRCLE_BOUND
        .lattice_points()
        .into_iter()
        .filter(Point::outside_of_ellipse)
        .filter(Point::proper_angle)
        .count()
        .to_string()
}
