// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

//! An example of creating merged ranges in a worksheet using the
//! `rust_xlsxwriter` library.

use rust_xlsxwriter::{Format, FormatAlign, FormatBorder, Workbook, XlsxColor, XlsxError};

fn main() -> Result<(), XlsxError> {
    // Create a new Excel file object.
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Write some merged cells with centering.
    let format = Format::new().set_align(FormatAlign::Center);

    worksheet.merge_range(1, 1, 1, 2, "Merged cells", &format)?;

    // Write some merged cells with centering and a border.
    let format = Format::new()
        .set_align(FormatAlign::Center)
        .set_border(FormatBorder::Thin);

    worksheet.merge_range(3, 1, 3, 2, "Merged cells", &format)?;

    // Write some merged cells with a number by overwriting the first cell in
    // the string merge range with the formatted number.
    worksheet.merge_range(5, 1, 5, 2, "", &format)?;
    worksheet.write_number_with_format(5, 1, 12345.67, &format)?;

    // Example with a more complex format and larger range.
    let format = Format::new()
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
        .set_border(FormatBorder::Thin)
        .set_background_color(XlsxColor::Silver);

    worksheet.merge_range(7, 1, 8, 3, "Merged cells", &format)?;

    // Save the file to disk.
    workbook.save("merge_range.xlsx")?;

    Ok(())
}
