use cached::proc_macro::cached;
use std::cmp::Ordering;

fn walls(bound: usize) -> Vec<Vec<usize>> {
    #[cached]
    fn walls_(bound: usize, n: usize) -> Vec<Vec<usize>> {
        match n.cmp(&bound) {
            Ordering::Less => {
                let mut _2 = walls_(bound, n + 2);
                for row in &mut _2 {
                    row.push(n + 2);
                }
                let mut _3 = walls_(bound, n + 3);
                for row in &mut _3 {
                    row.push(n + 3);
                }
                _2.append(&mut _3);
                _2
            }
            Ordering::Equal => vec![vec![]],
            Ordering::Greater => vec![],
        }
    }
    let mut ans = walls_(bound, 0);
    for row in &mut ans {
        row.reverse();
        row.pop();
    }
    ans
}

fn crack(mut wall_1: &[usize], mut wall_2: &[usize]) -> bool {
    while !wall_1.is_empty() && !wall_2.is_empty() {
        match wall_1[0].cmp(&wall_2[0]) {
            Ordering::Less => wall_1 = &wall_1[1..],
            Ordering::Equal => return false,
            Ordering::Greater => wall_2 = &wall_2[1..],
        }
    }
    true
}

fn compute(bound: usize, max_h: usize) -> usize {
    let walls = walls(bound);
    let correct: Vec<Vec<usize>> = (0..walls.len())
        .map(|row| {
            (0..walls.len())
                .filter(|&i| crack(&walls[row][..], &walls[i][..]))
                .collect()
        })
        .collect();
    let mut bricks: Vec<Vec<usize>> = vec![vec![0; walls.len()]; max_h];
    for x in 0..walls.len() {
        bricks[0][x] = 1;
    }
    for h in 1..max_h {
        for x in 0..walls.len() {
            for &i in &correct[x] {
                bricks[h][x] += bricks[h - 1][i];
            }
        }
    }
    bricks[max_h - 1].iter().sum()
}

pub fn euler215() -> String {
    compute(32, 10).to_string()
}
