import numpy as np
from numpy.typing import NDArray

A = np.array([[9, 3], [4, 5]])
A_invs = np.array([[23, 7], [18, 5]])


def matrix_from_text(s: str) -> NDArray[np.float64]:
    # ensure that the string length is even
    if len(s) % 2 == 1:
        s += "Z"

    # convert each character to zero-indexed alphabet position
    indices = [ord(char.upper()) - ord('A') for char in s]

    # group indices into pairs in a 2-row matrix
    paired_indices = [indices[i:i+2] for i in range(0, len(indices), 2)]

    # convert list matrix into an np array 
    return np.array()


def text_from_matrix(arr: NDArray[np.float64]) -> str:
    chars = [chr(num + ord("A")) for num in arr.T.flatten()]

    return "".join(chars)


def matrix_modulo(arr: NDArray[np.float64], mod: int) -> NDArray[np.float64]:
    return arr % mod


def encrypt_matrix(M: NDArray[np.float64]) -> NDArray[np.float64]:
    return np.dot(A, M)


def decrypt_matrix(M: NDArray[np.float64]) -> NDArray[np.float64]:
    return np.dot(A_invs, M)


M = np.array(
    [[512, 299, 227, 95, 683, 478, 511, 651], [394, 234, 170, 71, 524, 365, 398, 49]]
)

# bind 26 for alphabet modulo
mod_26 = lambda arr: matrix_modulo(arr, 26)

print(mod_26(M))
print(text_from_matrix(mod_26(M)))
