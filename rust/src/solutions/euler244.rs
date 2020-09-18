use std::collections::{HashMap, VecDeque};

/*
-------------
| 0| 1| 2| 3|
-------------
| 4| 5| 6| 7|
-------------
| 8| 9|10|11|
-------------
|12|13|14|15|
-------------

nodes ~ positions             positions of blank
vertices ~ moves               ||
Number of vertices in graph is 16 * binom(15, 7) = 102960
Each node has at most 4 edges            ||
(directions to move for blank)           positions of red tiles
*/

#[derive(Debug)]
struct Position {
    blank: u32,
    reds: [u32; 7], // it is not required for position to be sorted
                    // blues: remaining ones
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn to_ascii_code(&self) -> u64 {
        match &self {
            Direction::Left => 76,
            Direction::Right => 82,
            Direction::Up => 85,
            Direction::Down => 68,
        }
    }
}

// Encoding of position as index:
// first 4 bits are used to represent position of blank,
// next 16 bits are used to represent positions of reds,

impl Position {
    fn to_index(&self) -> u32 {
        self.blank + (self.reds.iter().fold(0, |acc, &r| acc + (1 << r)) << 4)
    }

    fn neighbours(&self) -> Vec<(Direction, Position)> {
        let move_left = if self.blank & 3 == 3 {
            None
        } else {
            let blank = self.blank + 1;
            let mut reds = self.reds;
            for i in 0..7 {
                if reds[i] == blank {
                    reds[i] = blank - 1;
                }
            }
            Some((Direction::Left, Position { blank, reds }))
        };
        let move_right = if self.blank & 3 == 0 {
            None
        } else {
            let blank = self.blank - 1;
            let mut reds = self.reds;
            for i in 0..7 {
                if reds[i] == blank {
                    reds[i] = blank + 1;
                }
            }
            Some((Direction::Right, Position { blank, reds }))
        };
        let move_up = if self.blank >= 12 {
            None
        } else {
            let blank = self.blank + 4;
            let mut reds = self.reds;
            for i in 0..7 {
                if reds[i] == blank {
                    reds[i] = blank - 4;
                }
            }
            Some((Direction::Up, Position { blank, reds }))
        };
        let move_down = if self.blank < 4 {
            None
        } else {
            let blank = self.blank - 4;
            let mut reds = self.reds;
            for i in 0..7 {
                if reds[i] == blank {
                    reds[i] = blank + 4;
                }
            }
            Some((Direction::Down, Position { blank, reds }))
        };
        vec![move_left, move_right, move_up, move_down]
            .into_iter()
            .filter_map(|x| x)
            .collect()
    }

    fn init_position() -> Position {
        Position {
            blank: 0,
            reds: [1, 4, 5, 8, 9, 12, 13],
        }
    }

    fn final_position() -> Position {
        Position {
            blank: 0,
            reds: [2, 5, 7, 8, 10, 13, 15],
        }
    }
}

pub fn euler244() -> String {
    let mut queue = VecDeque::<Position>::new();
    queue.push_back(Position::init_position());
    let mut shortest_paths = HashMap::<u32, (usize, Vec<Vec<Direction>>)>::new();
    shortest_paths.insert(Position::init_position().to_index(), (0, vec![vec![]]));
    let final_index = Position::final_position().to_index();
    let mut final_state_visited = false;
    while !queue.is_empty() {
        let pos = queue.pop_front().unwrap();
        let pos_index = pos.to_index();
        let (shortest_path_len, paths) = shortest_paths[&pos_index].clone();
        if pos_index == final_index {
            final_state_visited = true;
        }
        for (direction, neighbour) in pos.neighbours() {
            let neighbour_index = neighbour.to_index();
            if !shortest_paths.contains_key(&neighbour_index) && !final_state_visited {
                queue.push_back(neighbour);
            }
            let entry = shortest_paths
                .entry(neighbour_index)
                .or_insert((shortest_path_len + 1, vec![]));
            if shortest_path_len + 1 == entry.0 {
                for path in &paths {
                    let mut new_path = path.clone();
                    new_path.push(direction);
                    entry.1.push(new_path);
                }
            }
        }
    }
    let (_, paths_to_final_position) = shortest_paths.get(&final_index).unwrap();
    paths_to_final_position
        .into_iter()
        .map(|path| {
            path.into_iter().fold(0, |checksum, direction| {
                (checksum * 243 + direction.to_ascii_code()) % 100_000_007
            })
        })
        .sum::<u64>()
        .to_string()
}
