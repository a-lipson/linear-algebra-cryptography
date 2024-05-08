extern crate nalgebra as na;
use na::DMatrix;
use std::char;
use std::fmt;
use std::ops::Rem;

struct Matrix2 {
    matrix: DMatrix<i32>,
}

pub struct CipherMatrix2 {
    pub matrix: DMatrix<i32>,
}

pub struct TextMatrix2 {
    pub matrix: DMatrix<i32>,
}

trait ModularMatrix /*<T>*/ {
    fn new(matrix: DMatrix<i32>) -> Self;
    // {
    //     T { matrix }
    // }

    fn matrix(&self) -> &DMatrix<i32> {
        &self.matrix
    }

    fn modulo(&self, modulus: i32) -> Self
    where
        Self: Sized,
    {
        let modulated_matrix = self.matrix().map(|elem| {
            let mut a = elem % modulus;
            // ensure the result is non-negative
            if a < 0 {
                a += modulus;
            }
            a
        });

        Self::new(modulated_matrix)
    }
}

impl ModularMatrix for CipherMatrix2 {
    fn new(matrix: DMatrix<i32>) -> Self {
        CipherMatrix2 { matrix }
    }

    fn matrix(&self) -> &DMatrix<i32> {
        &self.matrix
    }
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

    // pub fn modulo(&self, modulus: i32) -> Self {
    //     let modulated_matrix = self.matrix.map(|elem| {
    //         let mut a = elem % modulus;
    //         // ensure the result is non-negative
    //         if a < 0 {
    //             a += modulus;
    //         }
    //         a
    //     });
    //
    //     CipherMatrix2 {
    //         matrix: modulated_matrix,
    //     }
    // }
}

impl ModularMatrix for TextMatrix2 {}

impl TextMatrix2 {
    pub fn new(&self) -> Self {
        TextMatrix2 {
            matrix: DMatrix::from_row_slice(2, 2, &[0, 0, 0, 0]),
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

        TextMatrix2 {
            matrix: modulated_matrix,
        }
    }

    pub fn text(&self) -> String {
        self.matrix
            .iter()
            .map(|&num| char::from_u32(num as u32 + 'A' as u32).unwrap())
            .collect()
    }
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

// const A_INVS: CipherMatrix2 = CipherMatrix2::new(9, 3, 4, 5);
// const A: CipherMatrix2 = CipherMatrix2::new(23, 7, 18, 5);

pub fn encrypt_matrix(message_matrix: &DMatrix<i32>) -> DMatrix<i32> {
    A.matrix * message_matrix
}

pub fn decrypt_matrix(message_matrix: &DMatrix<i32>) -> DMatrix<i32> {
    A_INVS.matrix * message_matrix
}
