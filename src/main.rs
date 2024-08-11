use keccak_hash::{keccak, keccak256};

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

    let root = [128, 91, 33, 216, 70, 177, 137, 239, 174, 176, 55, 125, 107, 176, 210, 1, 179, 135, 42, 54, 62, 96, 124, 37, 8, 143, 2, 91, 12, 106, 225, 248];
    let leaf = [58, 194, 37, 22, 141, 245, 66, 18, 162, 92, 28, 1, 253, 53, 190, 191, 234, 64, 143, 218, 194, 227, 29, 221, 111, 128, 164, 187, 249, 165, 241, 203];
    let proof = vec![
        [181, 85, 61, 227, 21, 224, 237, 245, 4, 217, 21, 10, 248, 45, 175, 165, 196, 102, 127, 166, 24, 237, 10, 111, 25, 198, 155, 65, 22, 108, 85, 16]
    ];
    let leaf = [181, 85, 61, 227, 21, 224, 237, 245, 4, 217, 21, 10, 248, 45, 175, 165, 196, 102, 127, 166, 24, 237, 10, 111, 25, 198, 155, 65, 22, 108, 85, 16];
    let proof = vec![
        [58, 194, 37, 22, 141, 245, 66, 18, 162, 92, 28, 1, 253, 53, 190, 191, 234, 64, 143, 218, 194, 227, 29, 221, 111, 128, 164, 187, 249, 165, 241, 203]
    ];

    let result2 = merkle_verify(proof, root, leaf);
    println!("Merkle verify: {:?}", result2);
}
