use std::collections::HashMap;
use num_traits::one;
use num_bigint::BigInt;
use crate::JacobianPoint;

#[derive(Debug)]
pub struct Curve {
    pub g: JacobianPoint,
    pub p: BigInt,
    pub n: BigInt,
}

impl Curve {
    fn new(params: &(&str, &str, &str, &str)) -> Curve {
        let bigint = |num: &str| -> BigInt { BigInt::parse_bytes(num.as_bytes(), 16).unwrap() };

        Curve {
            p: bigint(params.0),
            n: bigint(params.1),
            g: JacobianPoint {
                x: bigint(params.2),
                y: bigint(params.3),
                z: one(),
                fp: bigint(params.0),
            },
        }
    }

    fn get_curves() -> HashMap<String, (&'static str, &'static str, &'static str, &'static str)> {
        let mut curves = HashMap::new();

        curves.insert(
            String::from("secp256k1"),
            (
                "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
                "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",
                "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",
                "483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8",
            ),
        );

        curves
    }
}

pub fn get_curve(curve_name: &str) -> Curve {
    let curves = Curve::get_curves();

    let curve_params = curves.get(curve_name);

    Curve::new(curve_params.unwrap())
}
