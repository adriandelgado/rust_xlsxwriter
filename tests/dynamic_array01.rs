// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{Workbook, XlsxError};

mod common;

// Test case to test dynamic arrays/formulas.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.write_dynamic_array_formula_only(0, 0, 0, 0, "=AVERAGE(TIMEVALUE(B1:B2))")?;
    worksheet.write_string_only(0, 1, "12:00")?;
    worksheet.write_string_only(1, 1, "12:00")?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_dynamic_array01() {
    let test_runner = common::TestRunner::new()
        .set_name("dynamic_array01")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
