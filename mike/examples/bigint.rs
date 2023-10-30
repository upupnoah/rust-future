extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigUint;
use num_traits::One;

fn main() {
    let mut a: BigUint = One::one();
    let b: BigUint = BigUint::parse_bytes(b"100000000000000000000000000000", 10).unwrap();
    println!("a = {}", a);
    println!("b = {}", b);
    a = a + b;
    println!("a = {}", a);
}
