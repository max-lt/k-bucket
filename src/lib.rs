
pub type Buf<const N: usize> = [u8; N];

pub trait Distance<const N: usize> {
    fn distance(&self, other: &Buf<N>) -> Buf<N>;  
}

impl<const N: usize> Distance<N> for Buf<N> {
    /// XOR distance between two keys of size N
    fn distance(&self, right: &Buf<N>) -> [u8; N] {
        let mut result = [0; N];

        for i in 0..N {
            result[i] = self[i] ^ right[i];
        }

        result
    }
}

struct KBucket<K> {
    keys: Vec<K>,
    values: Vec<u64>,
}

impl<K> KBucket<K> {
    fn new() -> Self {
        KBucket {
            keys: Vec::new(),
            values: Vec::new(),
        }
    }

    fn insert(&mut self, key: [u8; 8], value: u64) {
        unimplemented!();
    }

    fn get(&self, key: [u8; 8]) -> Option<u64> {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let left = [0, 0, 0, 0, 0, 0, 0, 1];
        let right = [0, 0, 0, 0, 1, 0, 0, 2];

        let result = left.distance(&right);
        assert_eq!(result, [0, 0, 0, 0, 1, 0, 0, 3]);
        assert_eq!(result, right.distance(&left));
    }
}
