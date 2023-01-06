// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{Image, Workbook, XlsxError};

mod common;

// Test to demonstrate adding images to worksheets.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.set_row_height(1, 96)?;
    worksheet.set_column_width(2, 18)?;

    let mut image = Image::new("tests/input/images/issue32.png")?;
    image.set_alt_text("issue32.png");

    worksheet.insert_image(1, 2, &image)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_image17() {
    let test_runner = common::TestRunner::new()
        .set_name("image17")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
