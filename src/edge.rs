use std::{
    cmp::{max, min},
    sync::Arc,
};

use crate::point::Point;
use plotters::{coord::types::RangedCoordf32, prelude::*};

/// For now, we'll only support a single edge type.
/// We can always refactor this later to support multiple edge types.
///
/// This first type implements
/// https://en.wikipedia.org/wiki/Centripetal_Catmull%E2%80%93Rom_spline
pub struct CatRomSpline {
    /// The control points for the spline.
    ///
    /// The first and last points are the endpoints of the spline.
    /// The middle points are the control points.
    pub points: Vec<Point>,
}

/// This is a helper function to generate a spline for testing.
/// It has seven points, and the first and last points are the endpoints.
/// It should roughly look like a jigsaw edge.
pub fn example_spline() -> CatRomSpline {
    CatRomSpline {
        points: vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 0.4, y: -0.03 },
            Point { x: 0.35, y: 0.25 },
            Point { x: 0.6, y: 0.4 },
            Point { x: 0.75, y: 0.2 },
            Point { x: 0.6, y: 0.0 },
            Point { x: 1.0, y: 0.0 },
        ],
    }
}

impl CatRomSpline {
    pub fn draw_control_points(
        &self,
        chart: &mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordf32, RangedCoordf32>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let pts = self.points.iter().map(|p| (p.x, p.y)).collect::<Vec<_>>();
        // This draws a few points on the chart
        chart.draw_series(PointSeries::of_element(pts, 5, &RED, &|c, s, st| {
            EmptyElement::at(c) + Circle::new((0, 0), s, st)
        }))?;

        Ok(())
    }
    pub fn draw_line(
        &self,
        chart: &mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordf32, RangedCoordf32>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // For the first iteration, we only draw the line between points 1 and 2.
        // We also use points 0 and 3 as control points.

        let pts = self.as_path();

        chart.draw_series(LineSeries::new(pts, &BLUE))?;

        Ok(())
    }

    pub fn as_path(&self) -> Vec<(f32, f32)> {
        let mut pts = Vec::new();
        for i in 0..(self.points.len() - 1) {
            let i: usize = i;
            let p: [Point; 4] = [
                self.points[i.saturating_sub(1)],
                self.points[i],
                self.points[i + 1],
                self.points[min(i + 2, self.points.len() - 1)],
            ];
            pts.extend(interpolate_section(p, 20));
        }
        pts
    }
}

fn interpolate_section(p: [Point; 4], steps_per_segment: usize) -> Vec<(f32, f32)> {
    let alpha = 0.5;
    // First, get "timing" values for the points.
    // t_{i+1} = t_i + ||p_i - p_{i+1}||^\alpha where alpha = 0.5
    let t_0: f32 = 0.0;
    let t_1: f32 = t_0 + (p[0] - p[1]).abs().powf(alpha);
    let t_2: f32 = t_1 + (p[1] - p[2]).abs().powf(alpha);
    let t_3: f32 = t_2 + (p[2] - p[3]).abs().powf(alpha);

    let ts = [t_0, t_1, t_2, t_3];

    // Now, we need to find the four points that we'll use to interpolate.

    // Copy the first four points into an array.

    let mut pts = Vec::new();

    // Now, we need to generate a bunch of points on the spline.
    // We'll use the "uniform parameterization" method.
    for i in 0..steps_per_segment {
        let t = ts[1] + i as f32 * (ts[2] - ts[1]) / (steps_per_segment as f32);

        let c = interpolate_one_point(p, ts, t);
        pts.push((c.x, c.y));
    }
    pts
}

fn interpolate_one_point(p: [Point; 4], ts: [f32; 4], t: f32) -> Point {
    let a_1 = interpolate_one_layer(p[0], p[1], ts[0], ts[1], t);
    let a_2 = interpolate_one_layer(p[1], p[2], ts[1], ts[2], t);
    let a_3 = interpolate_one_layer(p[2], p[3], ts[2], ts[3], t);

    // Next layer of interpolation:
    let b_1 = interpolate_one_layer(a_1, a_2, ts[0], ts[2], t);
    let b_2 = interpolate_one_layer(a_2, a_3, ts[1], ts[3], t);

    // Final layer of interpolation:
    interpolate_one_layer(b_1, b_2, ts[1], ts[2], t)
}

fn interpolate_one_layer(p_0: Point, p_1: Point, t_0: f32, t_1: f32, t: f32) -> Point {
    if p_0.basically_equal(p_1) {
        // This avoids a divide by zero problem.
        p_0
    } else {
        p_0 * ((t_1 - t) / (t_1 - t_0)) + p_1 * ((t - t_0) / (t_1 - t_0))
    }
}
