use crate::Direction;
use crate::GetDirection;
use crate::GetDistance;
use crate::LeadingZeros;

pub type Buf<const N: usize> = [u8; N];

impl<const N: usize> GetDistance for Buf<N> {
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
impl<const N: usize> GetDirection for Buf<N> {
    fn direction(&self, i: usize) -> Direction {
        let byte = i >> 3;
        let bit = i % 8;

        // Sanity check
        if byte >= N {
            eprintln!("Index out of bounds {i} {byte} >= {N}");
            panic!("Index out of bounds");
        }

        match self[byte] & (1 << (7 - bit)) {
            0 => Direction::Left,
            _ => Direction::Right,
        }
    }
}

impl<const N: usize> LeadingZeros for Buf<N> {
    fn leading_zeros(&self) -> u8 {
        let mut result = 0;

        for i in 0..N {
            if self[i] == 0 {
                result += 8
            } else {
                return result + self[i].leading_zeros() as u8;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GetDistance;

    #[test]
    fn test_distance() {
        let left = [0, 0, 0, 0, 0, 0, 0, 1];
        let right = [0, 0, 0, 0, 1, 0, 0, 2];

        let result = left.distance(&right);
        assert_eq!(result, [0, 0, 0, 0, 1, 0, 0, 3]);
        assert_eq!(result, right.distance(&left));
    }

    #[test]
    fn test_direction() {
        let key = [1, 0, 0, 0, 0, 0, 0, 1];
        assert_eq!(key.direction(7), Direction::Right);
        assert_eq!(key.direction(1), Direction::Left);
    }
}
