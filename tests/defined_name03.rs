// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{Workbook, XlsxError};

mod common;

// Test to demonstrate defined names.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let _worksheet = workbook.add_worksheet().set_name("sheet One")?;

    workbook.define_name("Sales", r"='sheet One'!G1:H10")?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_defined_name03() {
    let test_runner = common::TestRunner::new()
        .set_name("defined_name03")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
