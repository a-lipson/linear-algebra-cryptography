extern crate nalgebra as na;
use crate::matrix::ModularMatrix;
mod matrix;
mod modulo;

fn main() {
    let cipher_digraph1 = "KH";
    let cipher_digraph2 = "XW";

    let plain_digraph1 = "TH";
    let plain_digraph2 = "HE";

    let cipher_matrix1 = matrix::TextMatrix2::new(cipher_digraph1);
    let cipher_matrix2 = matrix::TextMatrix2::new(cipher_digraph2);

    let plain_matrix1 = matrix::TextMatrix2::new(plain_digraph1);
    let plain_matrix2 = matrix::TextMatrix2::new(plain_digraph2);

    let ciphertext = "SONAFQCHMWPTVEVY";

    let text_matrix = matrix::TextMatrix2::new(ciphertext);

    println!("Encrypted Message Matrix: \n{}", text_matrix);

    let decrypted_message = text_matrix
        .multiply(&matrix::decrypt_matrix1())
        .modulo(26)
        .text();

    println!("Decrypted Message: \n{}", decrypted_message);

    let encrypt_matrix26 = matrix::CipherMatrix2::new(3, 11, 4, 15);
    let encrypt_matrix29 = matrix::CipherMatrix2::new(10, 15, 5, 9);

    let message = "SEND";

    let cipher_matrix = matrix::TextMatrix2::new(message);

    println!("Message Matrix: \n{}", cipher_matrix);

    let encrypted_matrix = cipher_matrix
        .multiply(&encrypt_matrix26)
        .modulo(26)
        .multiply(&encrypt_matrix29)
        .modulo(29);

    println!("Encrypted Message Matrix: \n{}", encrypted_matrix);

    let encrypted_message = encrypted_matrix.text();

    println!("Encrypted Message: \n{}", encrypted_message);

    let decrypt_matrix26 = encrypt_matrix26.inverse(26);
    let decrypt_matrix29 = encrypt_matrix29.inverse(29);

    let encrypted_message = "ZMOY";

    let encrypted_matrix = matrix::TextMatrix2::new(encrypted_message);

    println!("Encrypted Message Matrix: \n{}", encrypted_matrix);

    let decrypted_matrix = encrypted_matrix
        .multiply(&decrypt_matrix29)
        .modulo(29)
        .multiply(&decrypt_matrix26)
        .modulo(26);

    println!("Decrypted Message Matrix: \n{}", decrypted_matrix);

    let decrypted_message = decrypted_matrix.text();

    println!("Decrypted Message: \n{}", decrypted_message);
}
