// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{Format, Workbook, XlsxError, XlsxScript};

mod common;

// Test case to demonstrate cell font formatting.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let format1 = Format::new().set_font_script(XlsxScript::None);
    let format2 = Format::new().set_font_script(XlsxScript::Superscript);
    let format3 = Format::new().set_font_script(XlsxScript::Subscript);

    let worksheet = workbook.add_worksheet();
    worksheet.write_string(0, 0, "Rust", &format1)?;
    worksheet.write_string(1, 0, "Rust", &format2)?;
    worksheet.write_string(2, 0, "Rust", &format3)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn bootstrap22_super_sub_script() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap22")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
