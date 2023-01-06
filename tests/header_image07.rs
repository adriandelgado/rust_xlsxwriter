// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{Image, Workbook, XlsxError, XlsxImagePosition};

mod common;

// Test to demonstrate adding header/footer images to worksheets.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    let mut image1 = Image::new("tests/input/images/red.jpg")?;
    image1.set_alt_text("red.jpg");

    let image2 = Image::new("tests/input/images/blue.jpg")?;

    worksheet.set_header("&L&G");
    worksheet.set_header_image(&image2, XlsxImagePosition::Left)?;

    worksheet.insert_image(2, 1, &image1)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_header_image07() {
    let test_runner = common::TestRunner::new()
        .set_name("header_image07")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
