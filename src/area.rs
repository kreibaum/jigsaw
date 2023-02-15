//! A module to calculate areas enclosed by a path using green's theorem.
//! Here we integrate over x dy.
//!
//! The difference in area between two jigsaw edges is our measure of how
//! different they are. This does not account for the edge length, which
//! we normalize. It also does not account for the possibility to "flip"
//! the edge.

use crate::intersection;

fn for_points(p1: (f32, f32), p2: (f32, f32)) -> f32 {
    (p1.0 + p2.0) * (p2.1 - p1.1) / 2.0
}

pub fn for_path(path: &[(f32, f32)]) -> f32 {
    let mut area = 0.0;
    for i in 0..(path.len() - 1) {
        let i: usize = i;
        // area += x * dy
        area += for_points(path[i], path[i + 1]);
    }
    // We pretended, that x = path[i+1].0 + path[i].0 which is not quite right.
    // We need to divide by 2 to get the correct area.
    area
}

/// Takes two path going from (0, 0) to (1, 0) and returns area that is enclosed
/// by the two paths. This considers all intersections by first splitting the
/// paths along the intersections.
pub fn between_normalized_paths(path1: &[(f32, f32)], path2: &[(f32, f32)]) -> f32 {
    let mut intersections = intersection::find_intersections(path1, path2);
    // Sort by the first entry, which is the index into path1.
    intersections.sort_by(|a, b| a.0.cmp(&b.0));
    let mut area = 0.0;

    // The current subsection under consideration goes from index a_1 to b_1 on path1
    // and from index a_2 to b_2 on path2. We start on (0, 0) and go to the first
    // intersection. Then we go from the first intersection to the second
    // intersection, and so on.

    let mut a_1 = 0;
    let mut a_2 = 0;

    for (b_1, b_2) in intersections {
        // Get the area between the two paths.
        // By construction, a_1 < b_1, but a_2 < b_2 is not guaranteed.
        // In that case, we need to actually subtract the area, otherwise we
        // would double count it.
        let additional_area = if a_2 <= b_2 {
            // We get minus on the path2[a_2..=b_2], because we actually want to
            // go from path2[b_2] to path2[a_2] and not the other way around.
            (for_path(&path1[a_1..=b_1]) + for_points(path1[b_1], path2[b_2])
                - for_path(&path2[a_2..=b_2])
                + for_points(path2[a_2], path1[a_1]))
            .abs()
        } else {
            -(for_path(&path1[a_1..=b_1])
                + for_points(path1[b_1], path2[b_2])
                + for_path(&path2[b_2..=a_2])
                + for_points(path2[a_2], path1[a_1]))
            .abs()
        };
        area += additional_area;
        a_1 = b_1;
        a_2 = b_2;
    }
    // Now we need to go from the last intersection to (1, 0).
    let additional_area = (for_path(&path1[a_1..]) - for_path(&path2[a_2..])
        + for_points(path2[a_2], path1[a_1]))
    .abs();
    area += additional_area;

    area
}
