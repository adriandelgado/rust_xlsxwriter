// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{Format, Workbook, XlsxColor, XlsxError};

mod common;

// Test case to demonstrate creating a basic file with a font color.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    // Add an error color and duplicate to the test.
    let format = Format::new()
        .set_font_color(XlsxColor::RGB(0xFFEEEEEE)) // Error color.
        .set_font_color(XlsxColor::RGB(0xFF0000))
        .set_font_color(XlsxColor::RGB(0xFF0000)); // Duplicate.

    let worksheet = workbook.add_worksheet();
    worksheet.write_string(0, 0, "Hello", &format)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn bootstrap17_color_font() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap17")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
