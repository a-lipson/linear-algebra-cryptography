extern crate nalgebra as na;
use na::DMatrix;
mod matrix;
mod text;

fn main() {
    let ciphertext = "SONAFQCHMWPTVEVY";
    let e = matrix::matrix_modulo(&text::matrix_from_text(ciphertext), 26);
    println!(
        "Encrypted and modded matrix:\n{}",
        matrix::matrix_modulo(&matrix::encrypt_matrix(&e), 26)
    );
    println!(
        "Encrypted text:\n{}",
        text::text_from_matrix(&matrix::matrix_modulo(&matrix::encrypt_matrix(&e), 26))
    );

    let m = DMatrix::from_row_slice(
        2,
        8,
        &[
            512, 299, 227, 95, 683, 478, 511, 651, 394, 234, 170, 71, 524, 365, 398, 49,
        ],
    );
    println!("Modded M:\n{}", matrix::matrix_modulo(&m, 26));

    // Testing inverse
    let message = "THISISATEST";
    let e = matrix::matrix_modulo(
        &matrix::encrypt_matrix(&text::matrix_from_text(message)),
        26,
    );
    println!(
        "Decrypted matrix:\n{}",
        matrix::matrix_modulo(&matrix::decrypt_matrix(&e), 26)
    );
    println!(
        "Decrypted text:\n{}",
        text::text_from_matrix(&matrix::matrix_modulo(&matrix::decrypt_matrix(&e), 26))
    );
}
