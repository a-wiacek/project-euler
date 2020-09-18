use num::integer::Roots;
use ordered_float::OrderedFloat;
use std::f64::{consts::PI, INFINITY};

#[derive(Copy, Clone)]
struct Coords {
    x: f64,
    y: f64,
    z: f64,
}

fn triples(r: i64) -> Vec<Coords> {
    let mut ans = vec![];
    for a in 0..=r {
        for b in 0..=r {
            let c2 = r * r - a * a - b * b;
            if c2 >= 0 {
                let c = c2.sqrt();
                if c * c == c2 {
                    ans.push(Coords {
                        x: a as f64,
                        y: b as f64,
                        z: c as f64,
                    });
                }
            }
        }
    }
    ans
}

fn risk(a: Coords, b: Coords) -> f64 {
    let raw_dist = ((a.x - b.x).powi(2) + (a.y - b.y).powi(2) + (a.z - b.z).powi(2)).sqrt();
    let phi = (raw_dist / 2.0 / (a.x.powi(2) + a.y.powi(2) + a.z.powi(2)).sqrt()).asin();
    (2.0 * phi / PI).powi(2)
}

fn reflect_risk(a: Coords) -> f64 {
    let mut b = a;
    b.z = -b.z;
    risk(a, b)
}

fn dijkstra(coords: Vec<Coords>) -> f64 {
    let l = coords.len();
    let mut dist = vec![OrderedFloat(INFINITY); l];
    let mut visited = vec![false; l];
    dist[0] = OrderedFloat(0.0);
    while let Some(i) = (0..l).filter(|&i| !visited[i]).min_by_key(|&i| dist[i]) {
        visited[i] = true;
        for j in 0..l {
            if i != j {
                dist[j] = std::cmp::min(
                    dist[j],
                    OrderedFloat(dist[i].0 + risk(coords[i], coords[j])),
                );
            }
        }
    }
    (0..l)
        .map(|i| 2.0 * dist[i].0 + reflect_risk(coords[i]))
        .min_by_key(|&p| OrderedFloat(p))
        .unwrap()
}

pub fn euler353() -> String {
    let ans = std::iter::successors(Some(2), |&x| Some(x + x))
        .take(15)
        .map(|x| dijkstra(triples(x - 1)))
        .sum::<f64>();
    format!("{:.10}", ans)
}
