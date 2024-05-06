extern crate nalgebra as na;
use na::DMatrix;
use std::char;

// Convert a string to a matrix of zero-indexed alphabet positions
pub fn matrix_from_text(s: &str) -> DMatrix<i32> {
    let mut chars = s.chars().collect::<Vec<char>>();

    if chars.len() % 2 == 1 {
        chars.push('Z'); // add dummy char to ensure even char count
    }

    let indices = chars
        .iter()
        .map(|c| (*c as i32 - 'A' as i32))
        .collect::<Vec<i32>>();

    DMatrix::from_row_slice(chars.len() / 2, 2, &indices).transpose()
}

// Convert a matrix back to a string
pub fn text_from_matrix(matrix: &DMatrix<i32>) -> String {
    matrix
        .iter()
        .map(|&num| char::from_u32(num as u32 + 'A' as u32).unwrap())
        .collect()
}
