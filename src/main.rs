use std::convert::identity;
use std::error::Error;
use std::ops::Range;
use std::path::{Path, PathBuf};

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
    filename: &PathBuf,
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
        .build_cartesian_2d(x_range.clone(), y_range.clone())?;

    let x_labels_count = shares_x.len() + secret as usize;
    chart
        .configure_mesh()
        .x_labels(x_labels_count)
        .y_labels(5)
        .disable_mesh()
        .x_label_formatter(&|v| format!("{:.0}", v))
        .y_label_formatter(&|v| format!("{:.0}", v))
        .draw()?;

    // add vertical line at x=0
    let vertical_line = LineSeries::new(vec![(0.0, y_range.start), (0.0, y_range.end)], BLACK);

    // Draw the line on the chart
    chart.draw_series(vertical_line)?;

    // add the polynomial, shares and secret to the chart
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
                    + Text::new(format!("{:?}", coord), (1, 10), ("sans-serif", 15))
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
                    + Text::new(format!("{:?}", coord), (1, 10), ("sans-serif", 15))
            },
        ))?
        .label("Secret")
        .legend(|(x, y)| Circle::new((x, y), 5, GREEN.filled()));

    Ok(())
}

/// Creates a chart with a simple line.
///
/// The chosen polynomial is x.
fn line() -> Result<(), Box<dyn Error>> {
    let filename = Path::new("plots").join("line.svg");

    create_chart(
        &filename,
        "Two Points are Uniquely Determined by a Line",
        DIMENSIONS,
        2.5f32..4.5f32,
        2.0f32..4.5f32,
        identity,
        "x",
        &[3.0, 4.0],
        false,
    )?;

    Ok(())
}

/// Creates a chart with a quadratic polynomial.
///
/// The chosen polynomial is x².
fn quadratic() -> Result<(), Box<dyn Error>> {
    let filename = Path::new("plots").join("quadratic.svg");

    create_chart(
        &filename,
        "Three Points are Uniquely Determined by a Parabola",
        DIMENSIONS,
        -5.1f32..5.1f32,
        -1f32..26f32,
        |x| x.powi(2),
        "x²",
        &[-4.0, 1.0, 4.0],
        false,
    )?;

    Ok(())
}

/// Creates a chart with a cubic polynomial.
///
/// The chosen polynomial is x³.
fn cubic() -> Result<(), Box<dyn Error>> {
    let filename = Path::new("plots").join("cubic.svg");

    create_chart(
        &filename,
        "Four Points are Uniquely Determined by a Cubic",
        DIMENSIONS,
        -2.5f32..2.5f32,
        -20.0f32..20.0f32,
        |x| x.powi(3),
        "x³",
        &[-2.0, -1.0, 1.0, 2.0],
        false,
    )?;

    Ok(())
}

/// Creates a chart with a polynomial, its shares and the secret.
///
/// The chosen polynomial is 2x³ - 3x² + 2x + 5.
fn shamir() -> Result<(), Box<dyn Error>> {
    let filename = Path::new("plots").join("shamir.svg");

    create_chart(
        &filename,
        "Shamir's Secret Sharing",
        DIMENSIONS,
        -2.1f32..2.4f32,
        -30.0f32..20.0f32,
        |x| 2.0 * x.powi(3) - 3.0 * x.powi(2) + 2.0 * x + 5.0,
        "2x³ - 3x² + 2x + 5",
        &[-2.0, -1.0, 1.0, 2.0],
        true,
    )?;

    Ok(())
}

/// Creates an alternate chart with a polynomial,
/// an alternate single share and the secret.
///
/// The chosen polynomial is 2x³ - 3x² + 2x + 5.
fn shamir_alternate_single() -> Result<(), Box<dyn Error>> {
    let filename = Path::new("plots").join("shamir_alternate_single.svg");

    create_chart(
        &filename,
        "Shamir's Secret Sharing: Alternate Single Share",
        DIMENSIONS,
        -1.1f32..3.4f32,
        -30.0f32..60.0f32,
        |x| 2.0 * x.powi(3) - 3.0 * x.powi(2) + 2.0 * x + 5.0,
        "2x³ - 3x² + 2x + 5",
        &[-1.0, 1.0, 2.0, 3.0],
        true,
    )?;

    Ok(())
}

/// Creates an alternate chart with a polynomial,
/// alternate multiple shares and the secret.
///
/// The chosen polynomial is 2x³ - 3x² + 2x + 5.
fn shamir_alternate_multiple() -> Result<(), Box<dyn Error>> {
    let filename = Path::new("plots").join("shamir_alternate_multiple.svg");

    create_chart(
        &filename,
        "Shamir's Secret Sharing: Alternate Multiple Shares",
        DIMENSIONS,
        -2.7f32..3.0f32,
        -70.0f32..60.0f32,
        |x| 2.0 * x.powi(3) - 3.0 * x.powi(2) + 2.0 * x + 5.0,
        "2x³ - 3x² + 2x + 5",
        &[-2.5, -1.5, 1.5, 2.5],
        true,
    )?;

    Ok(())
}

/// The main function.
/// Calls the functions to create the charts.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    line()?;
    quadratic()?;
    cubic()?;
    shamir()?;
    shamir_alternate_single()?;
    shamir_alternate_multiple()?;

    Ok(())
}
