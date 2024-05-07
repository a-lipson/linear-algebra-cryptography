extern crate nalgebra as na;
use na::DMatrix;
use prime_factorization::Factorization;
use std::collections::HashSet;

trait RemoveDuplicates<T> {
    fn remove_duplicates(&self) -> Vec<T>;
}

// TODO: is there a better way to do this?
impl<T: Eq + std::hash::Hash + Clone> RemoveDuplicates<T> for Vec<T> {
    fn remove_duplicates(&self) -> Vec<T> {
        self.iter()
            .collect::<HashSet<&T>>() // Collect into a HashSet to remove duplicates
            .into_iter() // Convert HashSet back into an Iterator
            .cloned() // Clone the items to move from &T to T
            .collect() // Collect into Vec<T>
    }

    // fn remove_duplicates(&self) -> Vec<T> {
    //     let set: HashSet<&T> = self.iter().collect();
    //     set.into_iter().cloned().collect()
    // }
}

// modulo operation on matrices
pub fn matrix_modulo(matrix: &DMatrix<i32>, modulus: i32) -> DMatrix<i32> {
    matrix.map(|x| x % modulus)
}

// https://en.wikipedia.org/wiki/Euler%27s_totient_function
fn euler_totient(n: u64) -> u64 {
    let prime_factorization = Factorization::run(n);

    let mut result = n as f64;

    for factor in prime_factorization.factors.remove_duplicates() {
        result *= 1.0 - (1.0 / factor as f64);
    }

    result as u64
}

fn modular_exponentiation(base: i32, exponent: i32, modulus: i32) -> i32 {
    // convert to u64 to handle negative values
    let mut base = base as u32;
    let mut exponent = exponent as u32;
    let modulus = modulus as u32;
    let mut result = 1u32;

    // ensure the base is within the modulus to start
    base = base % modulus;

    while exponent > 0 {
        // if the exponent is odd, multiply the current base with the result
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }
        // square the base
        base = (base * base) % modulus;
        // divide the exponent by 2
        exponent >>= 1;
    }

    // convert the result back to i64
    result as i32
}

// https://en.wikipedia.org/wiki/Euler%27s_theorem
pub fn modular_inverse(a: i32, modulus: i32) -> i32 {
    println!(
        "{}^{} (mod {})",
        a,
        euler_totient(modulus as u64) as i32 - 1,
        modulus
    );
    modular_exponentiation(a, euler_totient(modulus as u64) as i32 - 1, modulus)
}

pub fn modulo_matrix_inverse(a: DMatrix<i32>, modulus: i32) -> DMatrix<i32> {
    // ensure the matrix is 2x2
    assert_eq!(a.nrows(), 2);
    assert_eq!(a.ncols(), 2);

    // compute the determinant
    let det = a[(0, 0)] * a[(1, 1)] - a[(0, 1)] * a[(1, 0)];

    // compute the modular inverse of the determinant
    let det_inv = modular_inverse(det, modulus);

    // calculate the adjugate matrix
    let adj = DMatrix::from_row_slice(2, 2, &[a[(1, 1)], -a[(0, 1)], -a[(1, 0)], a[(0, 0)]]);

    // multiply by the modular inverse and apply the modulus and ensure positivity
    adj.map(|elem| {
        let mut result = (det_inv * elem) % modulus;
        // Ensure the result is non-negative
        if result < 0 {
            // do we need a while here?
            result += modulus;
        }
        result
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euler_totient() {
        assert_eq!(euler_totient(26), 12);
        assert_eq!(euler_totient(29), 28);
    }

    #[test]
    fn test_modular_inverse() {
        assert_eq!(modular_inverse(7, 26), 15);
        assert_eq!(modular_inverse(3, 25), 17);
        assert_eq!(modular_inverse(24, 29), 23);
    }

    #[test]
    fn test_modulo_matrix_inverse() {}
}
