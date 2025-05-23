// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

//! An example of adding a trendline to a chart data series with a custom name.

use rust_xlsxwriter::{Chart, ChartTrendline, ChartTrendlineType, ChartType, Workbook, XlsxError};

fn main() -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Add some data for the chart.
    worksheet.write(0, 0, 11.1)?;
    worksheet.write(1, 0, 18.8)?;
    worksheet.write(2, 0, 33.2)?;
    worksheet.write(3, 0, 37.5)?;
    worksheet.write(4, 0, 52.1)?;
    worksheet.write(5, 0, 58.9)?;

    // Create a trendline.
    let mut trendline = ChartTrendline::new();
    trendline
        .set_type(ChartTrendlineType::Linear)
        .set_name("My trend name");

    // Create a new chart.
    let mut chart = Chart::new(ChartType::Line);

    // Add a data series with a trendline.
    chart
        .add_series()
        .set_values("Sheet1!$A$1:$A$6")
        .set_trendline(&trendline);

    // Add the chart to the worksheet.
    worksheet.insert_chart(0, 2, &chart)?;

    // Save the file.
    workbook.save("chart.xlsx")?;

    Ok(())
}
