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
            Point { x: 0.4, y: 0.0 },
            Point { x: 0.3, y: 0.2 },
            Point { x: 0.5, y: 0.3 },
            Point { x: 0.7, y: 0.2 },
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
            EmptyElement::at(c) + Circle::new((0, 0), s, st.filled())
        }))?;

        Ok(())
    }
    pub fn draw_line(
        &self,
        chart: &mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordf32, RangedCoordf32>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // For the first iteration, we only draw the line between points 1 and 2.
        // We also use points 0 and 3 as control points.

        // First, get "timing" values for the points.
        // t_{i+1} = t_i + ||p_i - p_{i+1}||^\alpha where alpha = 0.5
        let t_0: f32 = 0.0;
        let t_1: f32 = t_0 + (self.points[0] - self.points[1]).abs().powf(0.5);
        let t_2: f32 = t_1 + (self.points[1] - self.points[2]).abs().powf(0.5);
        let t_3: f32 = t_2 + (self.points[2] - self.points[3]).abs().powf(0.5);

        let mut pts = Vec::new();

        // Now, we need to generate a bunch of points on the spline.
        // We'll use the "uniform parameterization" method.
        for i in 0..10 {
            let t = t_1 + i as f32 * (t_2 - t_1) / 10.0;

            // Now, we need to find the four points that we'll use to interpolate.
            let p_0 = self.points[0];
            let p_1 = self.points[1];
            let p_2 = self.points[2];
            let p_3 = self.points[3];

            // Next layer of interpolation:
            let a_1 = p_0 * ((t_1 - t) / (t_1 - t_0)) + p_1 * ((t - t_0) / (t_1 - t_0));
            let a_2 = p_1 * ((t_2 - t) / (t_2 - t_1)) + p_2 * ((t - t_1) / (t_2 - t_1));
            let a_3 = p_2 * ((t_3 - t) / (t_3 - t_2)) + p_3 * ((t - t_2) / (t_3 - t_2));

            // Next layer of interpolation:
            let b_1 = a_1 * ((t_2 - t) / (t_2 - t_0)) + a_2 * ((t - t_0) / (t_2 - t_0));
            let b_2 = a_2 * ((t_3 - t) / (t_3 - t_1)) + a_3 * ((t - t_1) / (t_3 - t_1));

            // Final layer of interpolation:
            let c = b_1 * ((t_2 - t) / (t_2 - t_1)) + b_2 * ((t - t_1) / (t_2 - t_1));

            pts.push((c.x, c.y));
        }

        chart.draw_series(PointSeries::of_element(pts, 5, &BLUE, &|c, s, st| {
            EmptyElement::at(c) + Circle::new((0, 0), s, st.filled())
        }))?;

        Ok(())
    }
}
