use hex_conservative::DisplayHex;
use k_bucket::Arbiter;
use k_bucket::Bucket;
use k_bucket::Direction;
use k_bucket::GetDirection;
use k_bucket::GetDistance;
use k_bucket::GetKey;
use k_bucket::LeadingZeros;

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
struct Key(u8);

impl std::fmt::Debug for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Key({})", self.0)
    }
}

impl GetDistance for Key {
    fn distance(&self, right: &Key) -> Key {
        Key(self.0 ^ right.0)
    }
}

impl GetDirection for Key {
    fn direction(&self, i: usize) -> Direction {
        let bit = i % 8;

        match self.0 & (1 << (7 - bit)) {
            0 => Direction::Left,
            _ => Direction::Right,
        }
    }
}

struct Item {
    value: u8,
}

impl std::fmt::Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:08b} ({:03})", self.value, self.value)
    }
}

impl GetKey<Key> for Item {
    fn get_key(&self) -> Key {
        Key(self.value)
        // Key((self.value >> 8) as u8)
    }
}

impl Arbiter for Item {
    fn arbitrate(&self, candidate: &Self) -> bool {
        // self.value > candidate.value // update if candidate is less
    self.value < candidate.value // update if candidate is greater
    // true // always update
    // false // never update
    }
}

impl LeadingZeros for Key {
    fn leading_zeros(&self) -> u8 {
        self.0.leading_zeros() as u8
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

    let key = Key(0);
    let mut bucket: Bucket<Key, Item, 3> = Bucket::new(key);
    println!("- {:?}", bucket);

    for i in 0..=0xff {
        bucket.put(Item { value: i });
    }

    println!("- {:?}", bucket);

    let value = Item { value: 7 };
    println!("Get {:?} {:?}", value, bucket.get(&value.get_key()));

    let value = Item { value: 8 };
    println!("Get {:?} {:?}", value, bucket.get(&value.get_key()));

    // println!("Del {:?}", bucket.del(&Key(0)));

    println!("{:?}", bucket);
    println!("has {} items", bucket.count());

    let closest_to = Key(0);
    println!(
        "Closest to {:?}: {:?}",
        closest_to,
        bucket.closest(&closest_to, 10)
    );
}
