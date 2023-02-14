//! A module to calculate areas enclosed by a path using green's theorem.
//! Here we integrate over x dy.
//!
//! The difference in area between two jigsaw edges is our measure of how
//! different they are. This does not account for the edge length, which
//! we normalize. It also does not account for the possibility to "flip"
//! the edge.

pub fn for_path(path: &[(f32, f32)]) -> f32 {
    let mut area = 0.0;
    for i in 0..(path.len() - 1) {
        let i: usize = i;
        // area += x * dy
        area += (path[i + 1].0 + path[i].0) * (path[i + 1].1 - path[i].1);
    }
    // We pretended, that x = path[i+1].0 + path[i].0 which is not quite right.
    // We need to divide by 2 to get the correct area.
    area / 2.0
}
