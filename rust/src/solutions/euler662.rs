use crate::utils::numeric::sequences::fibonacci::Fibonacci;
use num::integer::Roots;

fn squares(n: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..=n).filter_map(move |x| {
        let y = (n * n - x * x).sqrt();
        if x * x + y * y == n * n {
            Some((x, y))
        } else {
            None
        }
    })
}

fn plus(a: &mut u32, b: u32) {
    let p = 1_000_000_007;
    *a += b;
    if *a >= p {
        *a -= p
    }
}

fn f(width: usize, height: usize) -> u32 {
    let paths = Fibonacci::new(1, 2)
        .take_while(|fib| fib < &(width + height))
        .flat_map(squares)
        .collect::<Vec<_>>();
    let mut arr = vec![vec![0u32; height + 1]; width + 1];
    arr[0][0] = 1;
    for x in 0..=width {
        for y in 0..=height {
            for &(dx, dy) in &paths {
                if x >= dx && y >= dy {
                    let b = arr[x - dx][y - dy];
                    plus(arr.get_mut(x).unwrap().get_mut(y).unwrap(), b);
                }
            }
        }
    }
    arr[width][height]
}

pub fn euler662() -> String {
    f(10_000, 10_000).to_string()
}
