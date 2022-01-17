use crypto::digest::Digest;
use crypto::sha2::Sha256;
use num_bigint::{BigInt, RandBigInt, ToBigInt};

use crate::*;

// note this is not a secure source of entropy!
// TODO: implement deterministic signatures use the RFC 6979 standard,
// which describes how you can generate a secure value for k based on
// the private key and message (or hash).
//
// @see: https://tools.ietf.org/html/rfc6979
fn get_entropy() -> BigInt {
    let mut rng = rand::thread_rng();
    let a = rng.gen_bigint(1000);
    let low = -10000.to_bigint().unwrap();
    let high = 10000.to_bigint().unwrap();
    let b = rng.gen_bigint_range(&low, &high);

    a * b
}

fn get_message_hash(message: &str, p: BigInt) -> BigInt {
    let mut sha = Sha256::new();

    sha.input_str(message);

    modulo(
        &BigInt::parse_bytes(sha.result_str().as_bytes(), 16).unwrap(),
        &p,
    )
}

// https://www.maximintegrated.com/en/design/technical-documents/tutorials/5/5767.html
// https://www.javacardos.com/tools/ecdsa-sign-verify
// https://csrc.nist.gov/publications/detail/fips/186/4/final
pub fn sign(
    message: &str,
    d: BigInt,
    k: Option<BigInt>,
    precomputes: &Vec<AffinePoint>,
    width: u32,
) -> Signature {
    let secp256k1 = curves::get_curve("secp256k1");

    let k = k.unwrap_or(modulo(&get_entropy(), &secp256k1.p));

    let p = secp256k1
        .g
        .to_affine()
        .multiply_with_non_adjacent_form(k.clone(), width, &precomputes);

    let r = modulo(&p.x, &secp256k1.p);

    let m = get_message_hash(message, secp256k1.p);

    let s1 = &d * &r + &m;

    let mod_inv = modular_multiplicative_inverse(&secp256k1.n, k, None, None);

    let s = modulo(&(&s1 * mod_inv), &secp256k1.n);

    Signature { r: r, s: s }
}

// P = S^-1 * z * G + S^-1 * R * Qa
pub fn verify(
    signature: &Signature,
    message: &str,
    public_key: AffinePoint,
    mut precomputes: Vec<AffinePoint>,
    width: u32,
) -> bool {
    let secp256k1 = curves::get_curve("secp256k1");

    let n = secp256k1.n;

    let z = get_message_hash(message, secp256k1.p);

    let r = &signature.r;
    let s = signature.s.clone();

    let w = modulo(&modular_multiplicative_inverse(&n, s, None, None), &n);

    let u1 = modulo(&(z * &w), &n);
    let u2 = modulo(&(r * w), &n);

    let u1_point = secp256k1
        .g
        .to_affine()
        .multiply_with_non_adjacent_form(u1, width, &precomputes);

    precomputes = precompute_points(public_key.clone(), width);

    let u2_point = public_key.multiply_with_non_adjacent_form(u2, width, &precomputes);

    let result = u1_point.add(&u2_point);

    result.x.eq(&r)
}
