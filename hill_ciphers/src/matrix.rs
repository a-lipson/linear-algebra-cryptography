extern crate nalgebra as na;
use na::DMatrix;
use std::fmt;

pub struct CipherMatrix2 {
    pub matrix: DMatrix<i32>,
}

pub struct TextMatrix2 {
    pub matrix: DMatrix<i32>,
}

impl CipherMatrix2 {
    pub fn new(a: i32, b: i32, c: i32, d: i32) -> Self {
        CipherMatrix2 {
            matrix: DMatrix::from_row_slice(2, 2, &[a, b, c, d]),
        }
    }

    pub fn scale(&self, scalar: i32) -> Self {
        let scaled_matrix = self.matrix.map(|elem| elem * scalar);

        CipherMatrix2 {
            matrix: scaled_matrix,
        }
    }

    pub fn modulo(&self, modulus: i32) -> Self {
        let modulated_matrix = self.matrix.map(|elem| {
            let mut a = elem % modulus;
            // ensure the result is non-negative
            if a < 0 {
                a += modulus;
            }
            a
        });

        CipherMatrix2 {
            matrix: modulated_matrix,
        }
    }
}

impl fmt::Display for CipherMatrix2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.matrix)
    }
}

const A_INVS: CipherMatrix2 = CipherMatrix2::new(9, 3, 4, 5);
const A: CipherMatrix2 = CipherMatrix2::new(23, 7, 18, 5);

pub fn encrypt_matrix(message_matrix: &DMatrix<i32>) -> DMatrix<i32> {
    A.matrix * message_matrix
}

pub fn decrypt_matrix(message_matrix: &DMatrix<i32>) -> DMatrix<i32> {
    A_INVS.matrix * message_matrix
}
