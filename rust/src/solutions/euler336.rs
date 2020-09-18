use itertools::Itertools;

fn permutations(carriages: u8) -> impl Iterator<Item = Vec<char>> {
    (0..carriages)
        .map(|d| ('A' as u8 + d) as char)
        .permutations(carriages as usize)
}

fn next(c: char) -> char {
    (c as u8 + 1) as char
}

fn count_rotations(arrangement: &Vec<char>) -> usize {
    fn count(i: char, arr: &mut Vec<char>) -> usize {
        if let Some(&h) = arr.first() {
            if h == i {
                arr.remove(0);
                count(next(i), arr)
            } else {
                let i_pos = arr.iter().position(|&c| c == i).unwrap_or(arr.len());
                if i_pos == arr.len() - 1 {
                    arr.pop();
                    arr.reverse();
                    1 + count(next(i), arr)
                } else {
                    arr[i_pos..].reverse();
                    arr.pop();
                    arr.reverse();
                    2 + count(next(i), arr)
                }
            }
        } else {
            0
        }
    }
    count('A', &mut arrangement.clone())
}

fn find_arrangement(n: usize, carriages: u8) -> Vec<char> {
    permutations(carriages)
        .filter(|arrangement| count_rotations(arrangement) == 2 * carriages as usize - 3)
        .nth(n - 1)
        .unwrap()
}

pub fn euler336() -> String {
    find_arrangement(2011, 11).into_iter().collect()
}
