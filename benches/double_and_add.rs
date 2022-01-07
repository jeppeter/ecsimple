#![feature(test)]

extern crate test;

use test::Bencher;

use num_bigint::BigInt;
use signer::curves::get_curve;
use signer::ecdsa::{sign, verify};

const K: &str = "80cb080a29daeab3862f5a3edf02f27f9d700da3c996130641b14afdacda3f60";
const M: &str = "message to sign";

#[bench]
fn benchmark(b: &mut Bencher) {
    b.iter(|| {
        let private_key = BigInt::parse_bytes(K.as_bytes(), 16).unwrap();

        let secp256k1 = get_curve("secp256k1");

        let public_key = secp256k1.g.to_affine().multiply(private_key.clone());

        let signature = sign(M, private_key, Some(BigInt::from(19)));

        verify(&signature, M, public_key);
    });
}
