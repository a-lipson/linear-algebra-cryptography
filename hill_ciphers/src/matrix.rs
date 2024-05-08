extern crate nalgebra as na;
use crate::modulo::modular_inverse;
use na::DMatrix;
use std::char;
use std::fmt;

pub struct CipherMatrix2 {
    matrix: DMatrix<i32>,
}

pub struct TextMatrix2 {
    matrix: DMatrix<i32>,
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

    // modular inverse
    pub fn inverse(&self, modulus: i32) -> Self {
        let a = self.matrix;

        let det = a[(0, 0)] * a[(1, 1)] - a[(0, 1)] * a[(1, 0)];

        let det_inv = modular_inverse(det, modulus);

        let adj = CipherMatrix2::new(a[(1, 1)], -a[(0, 1)], -a[(1, 0)], a[(0, 0)]);

        adj.scale(det_inv).modulo(modulus)
    }
}

impl TextMatrix2 {
    pub fn modulo(&self, modulus: i32) -> Self {
        let modulated_matrix = self.matrix.map(|elem| {
            let mut a = elem % modulus;
            // ensure the result is non-negative
            if a < 0 {
                a += modulus;
            }
            a
        });

        TextMatrix2 {
            matrix: modulated_matrix,
        }
    }

    pub fn encrypt(&self) -> Self {
        TextMatrix2 {
            matrix: self.matrix * encrypt_matrix().matrix,
        }
    }

    pub fn decrypt(&self) -> Self {
        TextMatrix2 {
            matrix: self.matrix * decrypt_matrix().matrix,
        }
    }

    pub fn text(&self) -> String {
        self.matrix
            .iter()
            .map(|&num| char::from_u32(num as u32 + 'A' as u32).unwrap())
            .collect()
    }

    // constructor to convert a string to a matrix of zero-indexed alphabet positions
    pub fn matrix_from_text(s: &str) -> Self {
        let mut chars = s.chars().collect::<Vec<char>>();

        if chars.len() % 2 == 1 {
            chars.push('Z'); // add dummy char to ensure even char count
        }

        let indices = chars
            .iter()
            .map(|c| (*c as i32 - 'A' as i32))
            .collect::<Vec<i32>>();

        TextMatrix2 {
            matrix: DMatrix::from_row_slice(chars.len() / 2, 2, &indices).transpose(),
        }
    }

    // Convert a matrix back to a string
    pub fn text_from_matrix(matrix: &DMatrix<i32>) -> String {
        matrix
            .iter()
            .map(|&num| char::from_u32(num as u32 + 'A' as u32).unwrap())
            .collect()
    }
}

impl fmt::Display for CipherMatrix2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.matrix)
    }
}

impl fmt::Display for TextMatrix2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.matrix)
    }
}

// move both to main

fn test() {
    TextMatrix2::matrix_from_text("THISISATEST");
}

pub fn encrypt_matrix() -> CipherMatrix2 {
    CipherMatrix2::new(9, 3, 4, 5)
}

pub fn decrypt_matrix() -> CipherMatrix2 {
    CipherMatrix2::new(23, 7, 18, 5)
}
