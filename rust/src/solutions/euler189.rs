use itertools::Itertools;

// Strategy: Construct 4x4 triangles and match 4 of them

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
enum Colour {
    Red,
    Green,
    Blue,
}

struct Row([Colour; 4]);

impl Row {
    // The other row is inverted!
    fn matches(&self, other: &Row) -> bool {
        let f = |i| self.0[i] != other.0[3 - i];
        (0..4).all(f)
    }
}

/*              ^
               / \
              / 0 \
             /     \
            ---------
           / \     / \
          / 1 \ 2 / 3 \
         /     \ /     \
        -----------------
       / \     / \     / \
      / 4 \ 5 / 6 \ 7 / 8 \
     /     \ /     \ /     \
    -------------------------
   / \     / \     / \     / \
  / 9 \ A / B \ C / D \ E / F \
 /     \ /     \ /     \ /     \
---------------------------------
Rotate by 180 degrees for bottom view.
*/
struct Triangle([Colour; 16]);

impl Triangle {
    fn left_row(&self) -> Row {
        Row([self.0[0], self.0[1], self.0[4], self.0[9]])
    }

    fn right_row(&self) -> Row {
        Row([self.0[0], self.0[3], self.0[8], self.0[15]])
    }

    fn bottom_row(&self) -> Row {
        Row([self.0[9], self.0[11], self.0[13], self.0[15]])
    }
}

fn all_triangles() -> Vec<Triangle> {
    let edges = [
        (0, 2),
        (1, 2),
        (2, 3),
        (1, 5),
        (3, 7),
        (4, 5),
        (5, 6),
        (6, 7),
        (7, 8),
        (4, 10),
        (6, 12),
        (8, 14),
        (9, 10),
        (10, 11),
        (11, 12),
        (12, 13),
        (13, 14),
        (14, 15),
    ];
    (0..16)
        .map(|_| vec![Colour::Red, Colour::Green, Colour::Blue])
        .multi_cartesian_product()
        .filter_map(|v| {
            let mut colours = [Colour::Red; 16];
            colours.copy_from_slice(&v[..]);
            let distinct = |&(i, j)| colours[i] != colours[j];
            if edges.iter().all(distinct) {
                Some(Triangle(colours))
            } else {
                None
            }
        })
        .collect()
}

pub fn euler189() -> String {
    let triangles = all_triangles();
    triangles
        .iter()
        .map(|triangle| {
            let matching_on_side = |row_extractor: fn(&Triangle) -> Row| {
                let the_row = row_extractor(triangle);
                triangles
                    .iter()
                    .map(row_extractor)
                    .filter(|other_row| the_row.matches(other_row))
                    .count()
            };
            let matching_upper = matching_on_side(Triangle::bottom_row);
            let matching_left = matching_on_side(Triangle::right_row);
            let matching_right = matching_on_side(Triangle::left_row);
            matching_upper * matching_left * matching_right
        })
        .sum::<usize>()
        .to_string()
}
