// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Chart, ChartLayout, ChartType, Workbook, XlsxError};

// Create a rust_xlsxwriter file to compare against an Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Add some test data for the chart(s).
    let data = [[1, 8, 3], [2, 7, 6], [3, 6, 9], [4, 8, 12], [5, 10, 15]];
    for (row_num, row_data) in data.iter().enumerate() {
        for (col_num, col_data) in row_data.iter().enumerate() {
            worksheet.write_number(row_num as u32, col_num as u16, *col_data)?;
        }
    }

    let mut chart = Chart::new(ChartType::Area);
    chart.set_axis_ids(43495808, 43497728);
    chart
        .add_series()
        .set_categories(("Sheet1", 0, 0, 4, 0))
        .set_values(("Sheet1", 0, 1, 4, 1));

    chart
        .add_series()
        .set_categories(("Sheet1", 0, 0, 4, 0))
        .set_values(("Sheet1", 0, 2, 4, 2));

    let layout1 = ChartLayout::new().set_offset(0.346203193350831, 0.850902595508894);

    let layout2 = ChartLayout::new().set_offset(0.213888888888888, 0.263499198016914);

    chart.x_axis().set_name("XXX").set_label_layout(&layout1);
    chart.y_axis().set_name("YYY").set_label_layout(&layout2);

    worksheet.insert_chart(8, 4, &chart)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_chart_layout05() {
    let test_runner = common::TestRunner::new()
        .set_name("chart_layout05")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
