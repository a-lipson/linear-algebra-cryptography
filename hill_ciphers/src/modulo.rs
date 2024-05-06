extern crate nalgebra as na;
use na::DMatrix;
use prime_factorization::Factorization;
use std::collections::HashSet;

trait RemoveDuplicates<T> {
    fn remove_duplicates(&self) -> Vec<T>;
}

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
    if prime_factorization.is_prime {
        return n;
    }

    let mut result = n as f64;
    for factor in prime_factorization.factors.remove_duplicates() {
        result *= 1.0 - (1.0 / factor as f64);
    }

    result as u64
}

fn modular_exponentiation(base: i32, exponent: i32, modulus: i32) -> i32 {
    // Convert to u64 to handle negative values gracefully
    let mut base = base as u32;
    let mut exponent = exponent as u32;
    let modulus = modulus as u32;
    let mut result = 1u32;

    // Ensure the base is within the modulus to start
    base = base % modulus;

    while exponent > 0 {
        // If the exponent is odd, multiply the current base with the result
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }
        // Square the base
        base = (base * base) % modulus;
        // Divide the exponent by 2
        exponent >>= 1;
    }

    // Convert the result back to i64
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
    DMatrix::from_row_slice(2, 2, &[])
}

// fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
//     if a == 0 {
//         (b, 0, 1)
//     } else {
//         let (gcd, x, y) = extended_gcd(b % a, a);
//         (gcd, y - (b / a) * x, x)
//     }
// }
//
// pub fn modular_inverse(x: i32) -> i32 {
//     let (gcd, x, _) = extended_gcd(a, m);
//     if gcd == 1 {
//         Some((x % m + m) % m) // Ensure the result is non-negative
//     } else {
//         None
//     }
// }
