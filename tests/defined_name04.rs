// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{Workbook, XlsxError};

mod common;

// Test to demonstrate rich strings.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let _worksheet = workbook.add_worksheet();

    workbook.define_name("\\__", "=Sheet1!$A$1")?;
    workbook.define_name("a3f6", "=Sheet1!$A$2")?;
    workbook.define_name("afoo.bar", "=Sheet1!$A$3")?;
    workbook.define_name("étude", "=Sheet1!$A$4")?;
    workbook.define_name("eésumé", "=Sheet1!$A$5")?;
    workbook.define_name("a", "=Sheet1!$A$6")?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_defined_name04() {
    let test_runner = common::TestRunner::new()
        .set_name("defined_name04")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
