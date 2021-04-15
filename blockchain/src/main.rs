use crypto_hash::{Algorithm, hex_digest};
use std::ops::Add;

#[derive(Debug)]
struct Block<'a> {
    pointer: Option<&'a Block<'a>>,
    prev_hash: Option<String>,
    data: String,
}


fn main() {
    let block1 = Block {
        pointer: None,
        prev_hash: None,
        data: String::from("primo"),
    };


    let mut data = String::from("secondo");
    let block2 = Block {
        pointer: Some(&block1),
        prev_hash: Some(hash(&block1.prev_hash, &mut data)),
        data,
    };

    let mut data2 = String::from("terzo");
    let block3 = Block {
        pointer: Some(&block2),
        prev_hash: Some(hash(&block2.prev_hash, &mut data2)),
        data: data2,
    };

    println!("{:?}", block3);
}

fn hash(prev_hash: &Option<String>, data: &mut String) -> String {
    return if prev_hash.is_some() {
        let prev = prev_hash.as_ref().unwrap();
        let mut tot = data.clone();
        tot.push_str(prev);
        hex_digest(Algorithm::SHA256, &tot.as_bytes())
    } else {
        hex_digest(Algorithm::SHA256, &data.as_bytes())
    };
}
