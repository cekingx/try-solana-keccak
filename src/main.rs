use keccak_hash::{keccak};

fn merkle_verify(proof: Vec<[u8; 32]>, root: [u8; 32], leaf: [u8; 32]) -> bool {
    let mut computed_hash = leaf;
    for proof_element in proof.into_iter() {
        if computed_hash <= proof_element {
            computed_hash = keccak(&[computed_hash.clone(), proof_element.clone()].concat()).0;
        } else {
            computed_hash = keccak(&[proof_element.clone(), computed_hash.clone()].concat()).0;
        }
    }
    computed_hash == root
}

fn main() {
    let a_hash = keccak(b"a");
    let b_hash = keccak(b"b");
    println!("Hash a: {:?}", a_hash.0);
    println!("Hash b: {:?}", b_hash.0);
    println!("Concatenated: {:?}", [a_hash.0, b_hash.0].concat());
    let result1 = keccak(&[a_hash.0, b_hash.0].concat());
    println!("Hash: {:?}", result1.0);

    let root = [67, 65, 96, 77, 12, 29, 49, 210, 189, 136, 4, 47, 57, 6, 230, 36, 17, 249, 223, 25, 76, 239, 9, 121, 146, 192, 3, 192, 216, 153, 60, 250];
    let leaf = [172, 123, 213, 44, 18, 249, 65, 24, 170, 212, 197, 102, 173, 195, 144, 233, 89, 223, 52, 89, 86, 109, 106, 239, 134, 234, 216, 177, 26, 78, 219, 99];
    let proof = vec![
        [172, 123, 213, 44, 18, 249, 65, 24, 170, 212, 197, 102, 173, 195, 144, 233, 89, 223, 52, 89, 86, 109, 106, 239, 134, 234, 216, 177, 26, 78, 219, 99]
    ];

    let result2 = merkle_verify(proof, root, leaf);
    println!("Merkle verify: {:?}", result2);
}
