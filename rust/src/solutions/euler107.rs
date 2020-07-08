use crate::utils::collections::find_union::FindUnion;
use crate::utils::input::get_input;

pub fn euler107() -> String {
    let coincidence_matrix: Vec<Vec<u32>> = get_input(107)
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<u32>().unwrap_or(0))
                .collect()
        })
        .collect();
    let graph_size = coincidence_matrix.len();
    let mut edges: Vec<((usize, usize), u32)> = (0..graph_size)
        .flat_map(|i| (i + 1..graph_size).map(move |j| (i, j)))
        .flat_map(|(i, j)| {
            if coincidence_matrix[i][j] > 0 {
                Some(((i, j), coincidence_matrix[i][j]))
            } else {
                None
            }
        })
        .collect();
    edges.sort_by_key(|edge| edge.1);
    let mut find_union = FindUnion::new(graph_size);
    let mut saved = 0;
    for ((x, y), w) in edges {
        if find_union.find(x) == find_union.find(y) {
            saved += w;
        } else {
            find_union.union(x, y);
        }
    }
    saved.to_string()
}
