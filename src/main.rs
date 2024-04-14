use std::ops::Range;

use plotters::coord::types::RangedCoordf32;
use plotters::prelude::*;

const DIMENSIONS: (u32, u32) = (640, 480);

/// Creates a chart with a polynomial, its shares and the secret.
/// The chart is saved to a file.
///
/// ## Arguments
///
/// * `filename` - The name of the file to save the chart to.
/// * `title` - The title of the chart.
/// * `dimensions` - The dimensions of the chart.
/// * `x_range` - The range of the x-axis.
/// * `y_range` - The range of the y-axis.
/// * `polynomial` - The polynomial to plot.
/// * `polynomial_str` - The string representation of the polynomial.
/// * `shares_x` - The x-coordinates of the shares.
/// * `secret` - Whether to plot the secret.
#[allow(clippy::too_many_arguments)]
fn create_chart(
    filename: &str,
    title: &str,
    dimensions: (u32, u32),
    x_range: Range<f32>,
    y_range: Range<f32>,
    polynomial: impl Fn(f32) -> f32,
    polynomial_str: &str,
    shares_x: &[f32],
    secret: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let root_area = SVGBackend::new(filename, dimensions).into_drawing_area();
    root_area.fill(&TRANSPARENT)?;

    let mut chart = ChartBuilder::on(&root_area)
        .caption(title, ("sans-serif", 32).into_font())
        .margin(5)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .build_cartesian_2d(x_range.clone(), y_range)?;

    chart
        .configure_mesh()
        .x_labels(5)
        .y_labels(5)
        .disable_mesh()
        .x_label_formatter(&|v| format!("{:.0}", v))
        .y_label_formatter(&|v| format!("{:.0}", v))
        .draw()?;

    draw_polynomial(&mut chart, &polynomial, polynomial_str, x_range)?;
    draw_shares(&mut chart, &polynomial, shares_x)?;
    if secret {
        draw_secret(&mut chart, &polynomial)?;
    }

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::LowerRight)
        .border_style(BLACK)
        .background_style(WHITE.mix(0.8))
        .legend_area_size(10)
        .draw()?;

    Ok(())
}

/// Draws a polynomial on a chart.
/// The polynomial is drawn as a line.
/// The chart is updated in place.
/// The polynomial is labeled in the legend, drawn in blue with
/// a width of 2 and stepsize of 1e-3.
fn draw_polynomial<F>(
    chart: &mut ChartContext<SVGBackend, Cartesian2d<RangedCoordf32, RangedCoordf32>>,
    polynomial: F,
    polynomial_str: &str,
    x_range: Range<f32>,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: Fn(f32) -> f32,
{
    let points: Vec<(f32, f32)> = x_range
        .step(1e-3)
        .values()
        .map(|x| (x, polynomial(x)))
        .collect();
    chart
        .draw_series(LineSeries::new(points.iter().copied(), BLUE))?
        .label(polynomial_str)
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], BLUE.stroke_width(2)));
    Ok(())
}

/// Draws shares on a chart.
/// The shares are drawn as points.
/// The chart is updated in place.
/// The shares are labeled in the legend, drawn in red with a size of 5.
fn draw_shares<F>(
    chart: &mut ChartContext<SVGBackend, Cartesian2d<RangedCoordf32, RangedCoordf32>>,
    polynomial: F,
    shares_x: &[f32],
) -> Result<(), Box<dyn std::error::Error>>
where
    F: Fn(f32) -> f32,
{
    let shares: Vec<(f32, f32)> = shares_x.iter().map(|&x| (x, polynomial(x))).collect();
    chart
        .draw_series(PointSeries::of_element(
            shares,
            5,
            RED.filled(),
            &|coord, size, style| {
                EmptyElement::at(coord)
                    + Circle::new((0, 0), size, style)
                    + Text::new(format!("{:?}", coord), (0, 10), ("sans-serif", 15))
            },
        ))?
        .label("Shares")
        .legend(|(x, y)| Circle::new((x, y), 5, RED.filled()));
    Ok(())
}

/// Draws the secret on a chart.
/// The secret is drawn as a point.
/// The chart is updated in place.
/// The secret is labeled in the legend, drawn in green with a size of 5.
fn draw_secret<F>(
    chart: &mut ChartContext<SVGBackend, Cartesian2d<RangedCoordf32, RangedCoordf32>>,
    polynomial: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: Fn(f32) -> f32,
{
    let secret = (0.0f32, polynomial(0.0));
    chart
        .draw_series(PointSeries::of_element(
            std::iter::once(secret),
            5,
            GREEN.filled(),
            &|coord, size, style| {
                EmptyElement::at(coord)
                    + Circle::new((0, 0), size, style)
                    + Text::new(format!("{:?}", coord), (0, 10), ("sans-serif", 15))
            },
        ))?
        .label("Secret")
        .legend(|(x, y)| Circle::new((x, y), 5, GREEN.filled()));

    Ok(())
}

/// Creates a chart with a polynomial, its shares and the secret.
///
/// The chosen polynomial is 2x³ - 3x² + 2x + 5.
fn full_plot() -> Result<(), Box<dyn std::error::Error>> {
    let filename = "full_plot.svg";

    create_chart(
        filename,
        "Shamir's Secret Sharing",
        DIMENSIONS,
        -2.1f32..2.1f32,
        -30.0f32..20.0f32,
        |x| 2.0 * x.powi(3) - 3.0 * x.powi(2) + 2.0 * x + 5.0,
        "2x³ - 3x² + 2x + 5",
        &[-2.0, -1.0, 1.0, 2.0],
        true,
    )?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    full_plot()
}
