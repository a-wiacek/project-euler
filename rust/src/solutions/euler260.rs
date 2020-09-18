fn sort(arr: [usize; 3]) -> [usize; 3] {
    let mut x = arr.clone();
    x.sort();
    x.reverse();
    x
}

pub fn euler260() -> String {
    let n = 1000;
    let mut arr = vec![vec![vec![false; n + 1]; n + 1]; n + 1];
    for x in 0..=n {
        for y in 0..=x {
            for z in 0..=y {
                if !arr[x][y][z] {
                    for d in 1..=n - x {
                        arr[x + d][y][z] = true;
                    }
                    for d in 1..=n - y {
                        let [xx, yy, zz] = sort([x, y + d, z]);
                        arr[xx][yy][zz] = true;
                    }
                    for d in 1..=n - z {
                        let [xx, yy, zz] = sort([x, y, z + d]);
                        arr[xx][yy][zz] = true;
                    }
                    for d in 1..=n - x {
                        arr[x + d][y + d][z] = true;
                    }
                    for d in 1..=n - x {
                        let [xx, yy, zz] = sort([x + d, y, z + d]);
                        arr[xx][yy][zz] = true;
                    }
                    for d in 1..=n - y {
                        let [xx, yy, zz] = sort([x, y + d, z + d]);
                        arr[xx][yy][zz] = true;
                    }
                    for d in 1..=n - x {
                        arr[x + d][y + d][z + d] = true;
                    }
                }
            }
        }
    }
    let mut ans = 0;
    for x in 0..=n {
        for y in 0..=x {
            for z in 0..=y {
                if !arr[x][y][z] {
                    ans += x + y + z;
                }
            }
        }
    }
    ans.to_string()
}
