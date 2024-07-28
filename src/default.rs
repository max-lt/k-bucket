use crate::Distance;
use crate::HasBitAt;

pub type Buf<const N: usize> = [u8; N];

impl<const N: usize> Distance for Buf<N> {
    /// XOR distance between two keys of size N
    fn distance(&self, right: &Buf<N>) -> [u8; N] {
        let mut result = [0; N];

        for i in 0..N {
            result[i] = self[i] ^ right[i];
        }

        result
    }
}

/// N key size in bits
impl<const N: usize> HasBitAt for Buf<N> {
    fn has_bit_at(&self, i: usize) -> bool {
        let byte = i >> 3;
        let bit = i % 8;

        // Sanity check
        if byte >= N {
            eprintln!("Index out of bounds {i} {byte} >= {N}");
            panic!("Index out of bounds");
        }

        self[byte] & (1 << (7 - bit)) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Distance;

    #[test]
    fn test_distance() {
        let left = [0, 0, 0, 0, 0, 0, 0, 1];
        let right = [0, 0, 0, 0, 1, 0, 0, 2];

        let result = left.distance(&right);
        assert_eq!(result, [0, 0, 0, 0, 1, 0, 0, 3]);
        assert_eq!(result, right.distance(&left));
    }

    #[test]
    fn test_has_bit_at() {
        let key = [1, 0, 0, 0, 0, 0, 0, 1];
        assert_eq!(key.has_bit_at(7), true);
        assert_eq!(key.has_bit_at(1), false);
    }
}
