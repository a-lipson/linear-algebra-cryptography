extern crate nalgebra as na;
use na::DMatrix;
mod matrix;
mod modulo;
mod text;

fn main() {
    let ciphertext = "SONAFQCHMWPTVEVY";

    // let e = modulo::matrix_modulo(&text::matrix_from_text(&ciphertext), 26);
    // println!(
    //     "Encrypted and modded matrix:\n{}",
    //     modulo::matrix_modulo(&matrix::encrypt_matrix(&e), 26)
    // );
    // println!(
    //     "Encrypted text:\n{}",
    //     text::text_from_matrix(&modulo::matrix_modulo(&matrix::encrypt_matrix(&e), 26))
    // );
    //
    // let m = DMatrix::from_row_slice(
    //     2,
    //     8,
    //     &[
    //         512, 299, 227, 95, 683, 478, 511, 651, 394, 234, 170, 71, 524, 365, 398, 49,
    //     ],
    // );
    // println!("Modded M:\n{}", modulo::matrix_modulo(&m, 26));
    //
    // // Testing inverse
    // let message = "THISISATEST";
    // let e = modulo::matrix_modulo(
    //     &matrix::encrypt_matrix(&text::matrix_from_text(&message)),
    //     26,
    // );
    // println!(
    //     "Decrypted matrix:\n{}",
    //     modulo::matrix_modulo(&matrix::decrypt_matrix(&e), 26)
    // );
    // println!(
    //     "Decrypted text:\n{}",
    //     text::text_from_matrix(&modulo::matrix_modulo(&matrix::decrypt_matrix(&e), 26))
    // );
    //
    // println!("Inverse Modulo test:\n{}", modulo::modular_inverse(7, 26));
    //
    // println!(
    //     "Inverse Matrix test:\n{}",
    //     modulo::matrix_modular_inverse(&matrix::CipherMatrix2::new(43, 27, 120, 1123), 27)
    // );
}
