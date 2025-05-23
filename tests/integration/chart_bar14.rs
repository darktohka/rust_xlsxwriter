// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Chart, ChartType, Format, Workbook, XlsxError};

// Create a rust_xlsxwriter file to compare against an Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let _ = workbook.add_worksheet();
    let worksheet = workbook.add_worksheet();

    // Add some test data for the chart(s).
    let data = [[1, 2, 3, 4, 5], [2, 4, 6, 8, 10], [3, 6, 9, 12, 15]];
    for (col_num, col_data) in data.iter().enumerate() {
        for (row_num, row_data) in col_data.iter().enumerate() {
            worksheet.write(row_num as u32, col_num as u16, *row_data)?;
        }
    }

    // Cerate a chart.
    let mut chart1 = Chart::new(ChartType::Bar);
    chart1.set_axis_ids(40294272, 40295808);
    chart1.add_series().set_values(("Sheet2", 0, 0, 4, 0));
    chart1.add_series().set_values(("Sheet2", 0, 1, 4, 1));
    chart1.add_series().set_values(("Sheet2", 0, 2, 4, 2));

    let mut chart2 = Chart::new(ChartType::Bar);
    chart2.set_axis_ids(40261504, 65749760);
    chart2.add_series().set_values(("Sheet2", 0, 0, 4, 0));

    let default_format = Format::default();
    worksheet.write_url_with_format(5, 0, "http://www.perl.com/", &default_format)?;

    worksheet.insert_chart(8, 4, &chart1)?;
    worksheet.insert_chart(24, 5, &chart2)?;

    let _ = workbook.add_worksheet();

    // Create a chartsheet and add the chart.
    let mut chart3 = Chart::new(ChartType::Column);
    chart3.set_axis_ids(65465728, 66388352);
    chart3.add_series().set_values(("Sheet2", 0, 0, 4, 0));
    chart3.add_series().set_values(("Sheet2", 0, 1, 4, 1));
    chart3.add_series().set_values(("Sheet2", 0, 2, 4, 2));

    let chartsheet = workbook.add_chartsheet();
    chartsheet.insert_chart(8, 4, &chart3)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_chart_bar14() {
    let test_runner = common::TestRunner::new()
        .set_name("chart_bar14")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
