use num_bigint::BigInt;
use signer::curves::get_curve;
use signer::ecdsa::{sign, verify};
use std::{env, process};

use signer::{
    compress_point, ecdsa_affine, ecdsa_windowed, ecdsa_wnaf, get_window_precomputes,
    precompute_points,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!(
            "\nError: {}. Please supply a private key and a message to sign.\n",
            err
        );

        process::exit(1);
    });

    let private_key = BigInt::parse_bytes(&config.private_key.as_bytes(), 16).unwrap();

    println!("===========================================");
    println!("AFFINE POINTS");
    println!("===========================================");

    let secp256k1 = get_curve("secp256k1");

    let mut public_key = secp256k1.g.to_affine().multiply(private_key.clone());

    let mut signature =
        ecdsa_affine::sign(&config.message, private_key.clone(), Some(BigInt::from(19)));

    println!("\nPublic key:\n\n{}\n", compress_point(&public_key));

    let result = ecdsa_affine::verify(&signature, &config.message, public_key);

    println!("Signature:\n\nr: {:x}\ns: {:x}\n", signature.r, signature.s);

    println!("Signature valid: {}\n", result);

    println!("===========================================");
    println!("JACOBIAN POINTS");
    println!("===========================================");

    let secp256k1 = get_curve("secp256k1");

    public_key = secp256k1.g.multiply(private_key.clone());

    signature = sign(&config.message, private_key.clone(), Some(BigInt::from(19)));

    println!("\nPublic key:\n\n{}\n", compress_point(&public_key));

    let result = verify(&signature, &config.message, public_key);

    println!("Signature:\n\nr: {:x}\ns: {:x}\n", signature.r, signature.s);

    println!("Signature valid: {}\n", result);

    println!("===========================================");
    println!("WINDOWED METHOD");
    println!("===========================================");

    let secp256k1 = get_curve("secp256k1");

    let precomputes = get_window_precomputes(&secp256k1.g.to_affine(), None);

    let public_key = secp256k1
        .g
        .to_affine()
        .multiply_with_windowed_method(private_key.clone(), &precomputes);

    let signature = ecdsa_windowed::sign(
        &config.message,
        private_key.clone(),
        Some(BigInt::from(19)),
        &precomputes,
    );

    println!("\nPublic key:\n\n{}\n", compress_point(&public_key));

    println!("Signature:\n\nr: {:x}\ns: {:x}\n", signature.r, signature.s);

    let result = ecdsa_windowed::verify(&signature, &config.message, public_key, precomputes);

    println!("Signature valid: {}\n", result);

    println!("===========================================");
    println!("WNAF METHOD");
    println!("===========================================");

    const WIDTH: u32 = 4;

    let generator = secp256k1.g.to_affine();

    let precomputes = precompute_points(generator, WIDTH);

    let public_key = secp256k1.g.to_affine().multiply_with_non_adjacent_form(
        private_key.clone(),
        WIDTH,
        &precomputes,
    );

    let signature = ecdsa_wnaf::sign(
        &config.message,
        private_key.clone(),
        Some(BigInt::from(19)),
        &precomputes,
        WIDTH,
    );

    println!("\nPublic key:\n\n{}\n", compress_point(&public_key));

    println!("Signature:\n\nr: {:x}\ns: {:x}\n", signature.r, signature.s);

    let result = ecdsa_wnaf::verify(&signature, &config.message, public_key, precomputes, WIDTH);

    println!("Signature valid: {}\n", result);

    println!("\n");
}

struct Config {
    private_key: String,
    message: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let private_key = args[1].clone();
        let message = args[2].clone();

        Ok(Config {
            private_key,
            message,
        })
    }
}
