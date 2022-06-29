use hex_literal::hex;
use ripemd::{Digest, Ripemd160};

#[test]
fn ripemd160_test() {
    // create a RIPEMD-160 hasher instance
    let mut hasher = Ripemd160::new();

    // process input message
    hasher.update(b"Hello world!");

    // acquire hash digest in the form of GenericArray,
    // which in this case is equivalent to [u8; 20]
    let result = hasher.finalize();
    assert_eq!(result[..], hex!("7f772647d88750add82d8e1a7a3e5c0902a346a3"));
}
