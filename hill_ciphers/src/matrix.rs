extern crate nalgebra as na;
use na::DMatrix;

// const A: Matrix2<i32> = Matrix2::new(23, 7, 18, 5);
// const A_INVS: Matrix2<i32> = Matrix2::new(9, 3, 4, 5);
pub fn A_INVS() -> DMatrix<i32> {
    DMatrix::from_row_slice(2, 2, &[9, 3, 4, 5])
}

pub fn A() -> DMatrix<i32> {
    DMatrix::from_row_slice(2, 2, &[23, 7, 18, 5])
}

// Encryption function
pub fn encrypt_matrix(m: &DMatrix<i32>) -> DMatrix<i32> {
    A() * m
}

// Decryption function
pub fn decrypt_matrix(m: &DMatrix<i32>) -> DMatrix<i32> {
    A_INVS() * m
}
