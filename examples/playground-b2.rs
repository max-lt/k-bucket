use k_bucket::Bucket;
use k_bucket::GetKey;
use k_bucket::HasBitAt;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Key([u8; 2]);

struct Item {
    value: u16,
}

impl std::fmt::Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl GetKey<Key> for Item {
    fn get_key(&self) -> Key {
        Key([self.value as u8, (self.value >> 8) as u8])
    }
}

impl HasBitAt for Key {
    fn has_bit_at(&self, i: usize) -> bool {
        let byte = i >> 3;
        let bit = i % 8;

        // Sanity check
        if byte >= 2 {
            panic!("Index out of bounds");
        }

        self.0[byte] & (1 << (7 - bit)) != 0
    }
}

fn main() {
    if !std::env::var("RUST_LOG").is_ok() {
        std::env::set_var("RUST_LOG", "debug");
    }

    env_logger::init();

    let key = Key([0, 0]);
    let mut bucket: Bucket<Key, Item, 3> = Bucket::new(key);
    println!("-  {:?}", bucket);

    for i in 0..0xffff {
        bucket.put(Item { value: i });
    }

    println!("{:?}", bucket);
}
