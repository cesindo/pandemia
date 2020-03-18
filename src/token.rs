//! Utility untuk kebutuhan pembuatan dan validasi token seperti akses token.
//!
//!

use crate::crypto;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use hex;
use rand::{thread_rng, Rng, RngCore};
use std::io::Cursor;

/// Generate random token
pub fn generate_u64() -> u64 {
    // let mut idgen = SnowflakeIdGenerator::new(1);
    // idgen.generate() as u64
    let mut bytes = Cursor::new(rand_bytes(8));
    bytes.read_u64::<LittleEndian>().expect("Can't generate u64")
}

/// Generate random bytes
pub fn rand_bytes(size: usize) -> Vec<u8> {
    let mut bytes: Vec<u8> = vec![0; size];
    // rng().fill(&mut bytes).map_err(|e| e.to_string())?;
    // Ok(bytes)
    thread_rng().fill_bytes(&mut bytes);
    bytes
}

/// Menggenerasikan kode unik untuk akses token pada API.
pub fn generate_access_token() -> String {
    let token_u64 = generate_u64();
    let mut wtr = vec![];
    wtr.write_u64::<BigEndian>(token_u64).unwrap();
    let bytes = crypto::sha512_hash_raw(wtr.as_slice());
    hex::encode(&bytes.to_vec())
}

/// Sama dengan `generate_access_token` bedanya ini general purpose
/// dengan menggunakan sha256 dengan ukuran token yang lebih pendek.
pub fn generate_token() -> String {
    let token_u64 = generate_u64();
    let mut wtr = vec![];
    wtr.write_u64::<BigEndian>(token_u64).unwrap();
    hex::encode(&crypto::sha256_hash_raw(wtr.as_slice()))
}

/// Menggenerasikan kode aktifasi yang biasa digunakan untuk register user baru.
pub fn generate_activation_code() -> String {
    format!("{}", thread_rng().gen_range(100_000, 999_999))
}

#[cfg(test)]
mod tests {
    use hex;
    use std::collections::HashMap;

    #[test]
    fn test_generate_activation_code() {
        let codes = (0..100).map(|_| super::generate_activation_code());
        for code in codes {
            assert_eq!(code.len(), 6);
        }
        // assert_eq! (super::generate_activation_code(), "test");
    }

    #[test]
    fn test_generate_access_token() {
        // let _ = ::sodiumoxide::init();

        let access_tokens: Vec<String> = (0..10).map(|_| super::generate_access_token()).collect();
        let access_tokens2 = access_tokens.clone();

        for at1 in access_tokens {
            let mut map = HashMap::new();
            for at2 in access_tokens2.iter() {
                if &at1 == at2 {
                    if !map.contains_key(at2) {
                        map.insert(at2.clone(), 1);
                    }
                    if map.get(at2) == Some(&1) {
                        continue;
                    }
                }
                assert_ne!(&at1, at2);
            }
        }
    }

    #[test]
    fn test_random_bytes() {
        let random_data1: Vec<Vec<u8>> = (0..20).map(|_| super::rand_bytes(32)).collect();
        let random_data2 = random_data1.clone();
        let mut map = HashMap::new();
        for rd1 in random_data1 {
            println!("rd: {}", hex::encode(&rd1));
            for rd2 in random_data2.iter().as_ref() {
                let (a, b) = (hex::encode(&rd1), hex::encode(&rd2));
                if a == b && map.get(&a) == None {
                    map.insert(a.clone(), 1);
                    continue;
                }
                assert_ne!(a, b);
            }
        }
    }
}
