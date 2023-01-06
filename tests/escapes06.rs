// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{Format, Workbook, XlsxError};

mod common;

// Test strings that need XML escaping.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();
    let num_format = Format::new().set_num_format(r#"[Red]0.0%\ "a""#);

    worksheet.set_column_width(0, 14)?;

    worksheet.write_number(0, 0, 123, &num_format)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_escapes06() {
    let test_runner = common::TestRunner::new()
        .set_name("escapes06")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
