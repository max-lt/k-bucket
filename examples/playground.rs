use hex_conservative::DisplayHex;
use k_bucket::Distance;

fn main() {
    let left = [0, 0, 0, 0, 1, 0, 0, 1];
    let right = [0, 0, 0, 0, 45, 0, 0, 2];

    let result2 = left.distance(&right);

    println!("{:?}", result2.to_lower_hex_string());

    let sup = left > right;

    println!("{:?}", sup);
}
