
use mimc_rs::Mimc7;
use mimc_rs::Constants;
extern crate num_bigint;
use num_bigint::{BigInt};
use core::fmt::Write;
fn main() {

    let b78: BigInt = BigInt::parse_bytes(b"78", 10).unwrap();
    let b41: BigInt = BigInt::parse_bytes(b"41", 10).unwrap();

    let mut big_arr2: Vec<BigInt> = Vec::new();
    big_arr2.push(b78.clone());
    big_arr2.push(b41.clone());

    let mimc7 = Mimc7::new();
    let h2 = mimc7.hash(big_arr2).unwrap();
    let (_, h2_bytes) = h2.to_bytes_be();
    let h2_bytes_hex =encode_hex(&h2_bytes);
    println!("h2: {:?}", h2_bytes_hex);
}

pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}