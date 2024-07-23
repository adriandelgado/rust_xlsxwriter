// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Button, Note, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    // Worksheet 1.
    let worksheet1 = workbook.add_worksheet();

    let button = Button::new();
    worksheet1.insert_button(1, 2, &button)?;

    // Worksheet 2.
    let worksheet2 = workbook.add_worksheet();
    worksheet2.set_default_note_author("John");

    let note = Note::new("Foo").set_author_prefix(false);
    worksheet2.insert_note(0, 0, &note)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_button08() {
    let test_runner = common::TestRunner::new()
        .set_name("button08")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
