extern crate nalgebra as na;
use na::DMatrix;
use std::char;

// Constants
// const A: Matrix2<i32> = Matrix2::new(23, 7, 18, 5);
// const A_INVS: Matrix2<i32> = Matrix2::new(9, 3, 4, 5);
fn A_INVS() -> DMatrix<i32> {
    DMatrix::from_row_slice(2, 2, &[9, 3, 4, 5])
}

fn A() -> DMatrix<i32> {
    DMatrix::from_row_slice(2, 2, &[23, 7, 18, 5])
}

// Convert a string to a matrix of zero-indexed alphabet positions
fn matrix_from_text(s: &str) -> DMatrix<i32> {
    let mut chars = s.chars().collect::<Vec<char>>();
    if chars.len() % 2 == 1 {
        chars.push('Z');
    }
    let indices = chars
        .iter()
        .map(|c| (*c as i32 - 'A' as i32))
        .collect::<Vec<i32>>();
    let matrix = DMatrix::from_row_slice(chars.len() / 2, 2, &indices);
    matrix.transpose() // To match Python's transpose
}

// Convert a matrix back to a string
fn text_from_matrix(matrix: &DMatrix<i32>) -> String {
    matrix
        .iter()
        .map(|&num| char::from_u32(num as u32 + 'A' as u32).unwrap())
        .collect()
}

// Modulo operation on matrices
fn matrix_modulo(matrix: &DMatrix<i32>, modulus: i32) -> DMatrix<i32> {
    matrix.map(|x| x % modulus)
}

// Encryption function
fn encrypt_matrix(m: &DMatrix<i32>) -> DMatrix<i32> {
    A() * m
}

// Decryption function
fn decrypt_matrix(m: &DMatrix<i32>) -> DMatrix<i32> {
    A_INVS() * m
}

fn main() {
    let ciphertext = "SONAFQCHMWPTVEVY";
    let e = matrix_modulo(&matrix_from_text(ciphertext), 26);
    println!(
        "Encrypted and modded matrix:\n{}",
        matrix_modulo(&encrypt_matrix(&e), 26)
    );
    println!(
        "Encrypted text:\n{}",
        text_from_matrix(&matrix_modulo(&encrypt_matrix(&e), 26))
    );

    let m = DMatrix::from_row_slice(
        2,
        8,
        &[
            512, 299, 227, 95, 683, 478, 511, 651, 394, 234, 170, 71, 524, 365, 398, 49,
        ],
    );
    println!("Modded M:\n{}", matrix_modulo(&m, 26));

    // Testing inverse
    let message = "THISISATEST";
    let e = matrix_modulo(&encrypt_matrix(&matrix_from_text(message)), 26);
    println!(
        "Decrypted matrix:\n{}",
        matrix_modulo(&decrypt_matrix(&e), 26)
    );
    println!(
        "Decrypted text:\n{}",
        text_from_matrix(&matrix_modulo(&decrypt_matrix(&e), 26))
    );
}
