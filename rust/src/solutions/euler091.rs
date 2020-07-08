use itertools::Itertools;
use std::ops::{Div, Mul, Sub};

#[derive(PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

const O: &Point = &Point { x: 0, y: 0 };

impl<'a> Sub<&'a Point> for &'a Point {
    type Output = Point;
    fn sub(self, rhs: &'a Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<'a> Mul<&'a Point> for &'a Point {
    type Output = i64;
    fn mul(self, rhs: &'a Point) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

fn right_triangle(a: &Point, b: &Point, c: &Point) -> bool {
    let x = &(a - b);
    let y = &(b - c);
    let z = &(c - a);
    [x * y, y * z, z * x].contains(&0)
}

pub fn euler091() -> String {
    let points = (0..=50)
        .cartesian_product(0..=50)
        .map(|(x, y)| Point { x, y })
        .collect_vec();
    points
        .iter()
        .cartesian_product(points.iter())
        .filter(|&(a, b)| a != b && a != O && b != O && right_triangle(a, b, O))
        .count()
        .div(2)
        .to_string()
}
