mod area;
mod edge;
mod point;
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Sets up the backend drawing area.
    let root = BitMapBackend::new("0.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    // Sets up our coordinate system. We are focused on "unit jigsaw edges"
    // which start at (0,0) and end at (1,0).
    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-0.1f32..1.1f32, -0.3f32..1f32)?;

    // This draws the coordinate system & axis
    chart.configure_mesh().draw()?;

    // This draws a few points on the chart
    edge::example_spline().draw_control_points(&mut chart)?;
    edge::example_spline().draw_line(&mut chart)?;
    println!(
        "Area: {}",
        area::for_path(&edge::example_spline().as_path())
    );

    root.present()?;

    Ok(())
}
