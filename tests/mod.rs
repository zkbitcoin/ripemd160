use hex_literal::hex;
use ripemd::{Digest, Ripemd160};

#[test]
fn ripemd160_test() {
    // create a RIPEMD-160 hasher instance
    let mut hasher = Ripemd160::new();

    // process input message
    hasher.update(b"hello");

    // acquire hash digest in the form of GenericArray,
    // which in this case is equivalent to [u8; 20]
    let result = hasher.finalize();
    assert_eq!(result[..], hex!("108f07b8382412612c048d07d13f814118445acd"));
}
