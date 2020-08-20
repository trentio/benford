use plotters::prelude::*;

pub fn create_graph(data: Vec<usize>) -> Result<(), Box<dyn std::error::Error>> {
    let root =
        BitMapBackend::new("./histogram.png", (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;
    let len = data.len() as isize;
    let y_max_val = (len * 3) / 5;


    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Distribution", ("sans-serif", 50.0).into_font())
        .build_ranged(0isize..10isize, 0isize..y_max_val)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .line_style_1(&WHITE.mix(0.3))
        .x_label_offset(30)
        .y_desc("Count")
        .x_desc("Digit")
        .axis_desc_style(("sans-serif", 15).into_font())
        .draw()?;


    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(data.iter().map(|x| (*x as isize, 1)))

    )?;

    Ok(())
}
