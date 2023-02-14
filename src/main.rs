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

    // This draws a line from an explicit list of points (generated on the fly)
    chart
        .draw_series(LineSeries::new(
            (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x * x, x)),
            &RED,
        ))?
        .label("y = x^2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    // This draws a few points on the chart
    chart.draw_series(PointSeries::of_element(
        vec![(0.0, 0.0), (1.0, 0.0), (0.5, 0.4)],
        5,
        &RED,
        &|c, s, st| EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()),
    ))?;

    // This draws the legend on the chart.
    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}
