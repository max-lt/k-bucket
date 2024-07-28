use k_bucket::Arbiter;
use k_bucket::Bucket;
use k_bucket::Direction;
use k_bucket::GetDistance;
use k_bucket::GetDirection;
use k_bucket::GetKey;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
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

impl Arbiter for Item {
    fn arbitrate(&self, candidate: &Self) -> bool {
      self.value > candidate.value
    }
}

impl GetDirection for Key {
    fn direction(&self, i: usize) -> Direction {
        let byte = i >> 3;
        let bit = i % 8;

        // Sanity check
        if byte >= 2 {
            panic!("Index out of bounds");
        }

        match self.0[byte] & (1 << (7 - bit)) {
            0 => Direction::Left,
            _ => Direction::Right,
        }
    }
}

impl GetDistance for Key {
    fn distance(&self, right: &Self) -> Key {
        Key(self.0.distance(&right.0))
    }
}

fn main() {
    if !std::env::var("RUST_LOG").is_ok() {
        std::env::set_var("RUST_LOG", "debug");
    }

    env_logger::init();

    let key = Key([0xfe, 0x00]);
    let mut bucket: Bucket<Key, Item, 5> = Bucket::new(key);
    println!("-  {:?}", bucket);

    for i in 0..0xffff {
        bucket.put(Item { value: i });
    }

    println!("{:?}", bucket);
    println!("has {} items", bucket.count());

    let closest_to = Key([0xff, 0x00]);
    println!("Closest to {:?}: {:?}", closest_to, bucket.closest(&closest_to, 10));
}
