use k_bucket::Arbiter;
use k_bucket::Bucket;
use k_bucket::GetKey;

struct Item {
    value: u16,
}

impl GetKey<[u8; 2]> for Item {
    fn get_key(&self) -> [u8; 2] {
        [self.value as u8, (self.value >> 8) as u8]
    }
}

impl Arbiter for Item {
    fn arbitrate(&self, candidate: &Self) -> bool {
      self.value > candidate.value
    }
}

impl std::fmt::Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}


fn main() {
    if !std::env::var("RUST_LOG").is_ok() {
        std::env::set_var("RUST_LOG", "debug");
    }

    env_logger::init();

    let key = [0, 0];
    let mut bucket: Bucket<[u8; 2], Item, 3> = Bucket::new(key);
    println!("-  {:?}", bucket);

    for i in 0..0xffff {
        bucket.put(Item { value: i });
    }

    println!("{:?}", bucket);

}
