
use num_bigint::{BigInt};
use num_traits::{zero,one};

/// Performs a modulo operation of a under b.
///
/// # Example
///
/// In Rust, the % operator performs the remainder operation, not the modulus operation.
/// These operations have different results for negative numbers, e.g.:
///
/// -21 modulus 4 => 3
/// -21 remainder 4 => -1
///
/// println!("{}", -21 % 4); // -1
///
/// This helper performs a modulus operation, as opposed to a simple remainder operation.
///
/// ```
/// use num_bigint::BigInt;
/// let a = BigInt::from(218753);
/// let b = BigInt::from(96461);
/// let expected = BigInt::from(25831);
/// let actual = ecsimple::modulo(&a, &b);
/// assert_eq!(actual, expected);
/// ```
#[allow(dead_code)]
pub fn modulo(a: &BigInt, b: &BigInt) -> BigInt {
    let result = a % b;

    if result >= zero() {
        result
    } else {
        b + result
    }
}

// https://extendedeuclideanalgorithm.com/multiplicative_inverse.php
#[allow(dead_code)]
pub fn modular_multiplicative_inverse(
    n: &BigInt,
    mut b: BigInt,
    t1: Option<BigInt>,
    t2: Option<BigInt>,
) -> BigInt {
    let t1 = t1.unwrap_or(zero());
    let t2 = t2.unwrap_or(one());

    if n == &zero() || b == zero() {
        return zero();
    }

    if b < zero() {
        b = modulo(&b, n);
    }

    let q = n / &b;
    let r = modulo(n, &b);

    let t3 = t1 - &q * &t2;

    if r == zero() && b != one() {
        return zero();
    }

    if r == zero() {
        t2
    } else {
        modular_multiplicative_inverse(&b, r, Some(t2), Some(t3))
    }
}