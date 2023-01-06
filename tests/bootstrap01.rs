// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use std::path::{Path, PathBuf};

use rust_xlsxwriter::{Workbook, XlsxError};

mod common;

// Test case to demonstrate creating a basic file with 1 worksheet and no data.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    _ = workbook.add_worksheet();

    workbook.save(filename)?;

    Ok(())
}

// Test case to demonstrate creating a basic file with 1 worksheet and no data.
// Has an implicit add_worksheet.
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    workbook.save(filename)?;

    Ok(())
}

// Test case to demonstrate creating a basic file from a path.
fn create_new_xlsx_file_3(filename: &str) -> Result<(), XlsxError> {
    let path = Path::new(filename);

    let mut workbook = Workbook::new();
    _ = workbook.add_worksheet();

    workbook.save(&path)?;

    Ok(())
}

// Test case to demonstrate creating a basic file from a pathbuf.
fn create_new_xlsx_file_4(filename: &str) -> Result<(), XlsxError> {
    let mut path = PathBuf::new();
    path.push(filename);

    let mut workbook = Workbook::new();
    _ = workbook.add_worksheet();

    workbook.save(&path)?;

    Ok(())
}

// Test case to demonstrate creating a basic file to a buffer.
fn create_new_xlsx_file_5(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    _ = workbook.add_worksheet();

    let buf = workbook.save_to_buffer()?;

    // Write the buffer out to a file.
    let mut file = std::fs::File::create(filename)?;
    std::io::Write::write_all(&mut file, &buf)?;

    Ok(())
}

// Test case for multiple saves.
fn create_new_xlsx_file_6(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    workbook.save(filename)?;
    workbook.save(filename)?;
    workbook.save(filename)?;

    Ok(())
}

#[test]
fn bootstrap01_single_worksheet() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap01")
        .set_function(create_new_xlsx_file_1)
        .unique("1")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn bootstrap01_add_default_worksheet() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap01")
        .set_function(create_new_xlsx_file_2)
        .unique("2")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn bootstrap01_new_from_path() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap01")
        .set_function(create_new_xlsx_file_3)
        .unique("3")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn bootstrap01_new_from_pathbuf() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap01")
        .set_function(create_new_xlsx_file_4)
        .unique("4")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn bootstrap01_new_from_buffer() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap01")
        .set_function(create_new_xlsx_file_5)
        .unique("5")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn bootstrap01_multi_save1() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap01")
        .set_function(create_new_xlsx_file_6)
        .unique("6")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
