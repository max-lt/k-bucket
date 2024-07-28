use hex_conservative::DisplayHex;
use k_bucket::Bucket;
use k_bucket::Distance;
use k_bucket::HasBitAt;
use k_bucket::GetKey;

#[derive(PartialEq, Clone, Copy)]
struct Key(u8);

impl std::fmt::Debug for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Key({})", self.0)
    }
}

impl Distance for Key {
    fn distance(&self, right: &Key) -> Key {
        Key(self.0 ^ right.0)
    }
}

impl HasBitAt for Key {
    fn has_bit_at(&self, i: usize) -> bool {
        let bit = i % 8;

        self.0 & (1 << (7 - bit)) != 0
    }
}

struct Item {
    value: u16,
}

impl std::fmt::Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:08b} ({:03})", self.value, self.value)
    }
}

impl GetKey<Key> for Item {
    fn get_key(&self) -> Key {
        Key((self.value | 256) as u8)
    }
}

fn main() {
    if !std::env::var("RUST_LOG").is_ok() {
        std::env::set_var("RUST_LOG", "debug");
    }

    env_logger::init();

    let left = [0, 0, 0, 0, 1, 0, 0, 1];
    let right = [0, 0, 0, 0, 45, 0, 0, 2];

    let result2 = left.distance(&right);

    println!("{:?}", result2.to_lower_hex_string());

    let sup = left > right;

    println!("{:?}", sup);

    let key = Key(77);
    let mut bucket: Bucket<Key, Item, 1> = Bucket::new(key);
    println!("- {:?}", bucket);

    for i in 0..255 {
        bucket.put(Item { value: i });
    }

    println!("- {:?}", bucket);

    let value = Item { value: 7 };
    println!("Get {:?} {:?}", value, bucket.get(&value.get_key()));

    let value = Item { value: 8 };
    println!("Get {:?} {:?}", value, bucket.get(&value.get_key()));

    println!("Del {:?}", bucket.del(&Key(8)));

    println!("{:?}", bucket);

}
