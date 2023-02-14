//! Module for finding intersections between paths.
//!
//! Starts with a very naive implementation that won't be fast.
//! We can optimize later once it works.

/// Local minimum doesn't mean that the curves actually intersect. This constant
/// is used to determine if the distance between two points is close enough to
/// be considered an intersection.
const CLOSE_ENOUGH: f32 = 0.01;

/// Given two paths, find all intersections between them.
/// Returns a list of indices into the two paths.
pub fn find_intersections(path1: &[(f32, f32)], path2: &[(f32, f32)]) -> Vec<(usize, usize)> {
    // For two indices i, j into path1 and path2, respectively, we have a
    // distance between the two points on the paths. So we can treat this as
    // a 2d array of distances, where we want to find all local minima.
    // We can do this by looking at the 8 neighbors of each point.
    // If the point is a local minimum, then we have an intersection.

    let w = path1.len();
    let h = path2.len();
    let mut intersections: Vec<(usize, usize)> = Vec::new();
    let mut distances = vec![0.0; w * h];
    for i in 0..path1.len() {
        for j in 0..path2.len() {
            let i: usize = i;
            let j: usize = j;
            let d = (path1[i].0 - path2[j].0).powi(2) + (path1[i].1 - path2[j].1).powi(2);
            distances[i * w + j] = d;
        }
    }

    // Now, we need to find the local minima. We don't need to check the edges
    // for now.
    for i in 1..(w - 1) {
        for j in 1..(h - 1) {
            let i: usize = i;
            let j: usize = j;
            let d = distances[i * w + j];
            if d < CLOSE_ENOUGH.powi(2)
                && d <= distances[(i - 1) * w + j - 1]
                && d <= distances[(i - 1) * w + j]
                && d <= distances[(i - 1) * w + j + 1]
                && d <= distances[i * w + j - 1]
                && d <= distances[i * w + j + 1]
                && d <= distances[(i + 1) * w + j - 1]
                && d <= distances[(i + 1) * w + j]
                && d <= distances[(i + 1) * w + j + 1]
            {
                intersections.push((i, j));
            }
        }
    }

    intersections
}
