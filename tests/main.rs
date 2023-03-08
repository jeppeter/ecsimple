#[cfg(test)]
mod tests {
    use num_bigint::BigInt;
    use ecsimple::{compress_point, curves, get_window_precomputes, precompute_points};

    pub fn get_test_vectors() -> [(&'static str, &'static str); 21] {
        return [
            (
                "80cb080a29daeab3862f5a3edf02f27f9d700da3c996130641b14afdacda3f60",
                "03d2bc5446c3ef8b8d2be6e62d33be132050cb307f4e872df8383fb803b646305f",
            ),
            (
                "e2d9842a238b8ad562d14c4752153e844ce6e9bf93bc18643aad39ad657419b3",
                "024c65aab27efe44842eee1bb65e8a806549bc2a3162179485caffe3c341b0998d",
            ),
            (
                "e6ad7b05f39843fd545ba2a43ffee83e5f7ead101d80e224049f2b2139595e07",
                "0265a49b0afaa6ac8a6e1096abe715a771972e85b73f8c4464e5e339f3cfee9663",
            ),
            (
                "49b35e6c82842227651c460d8ba4359ba26bf1d17e8fbcb04095da3bc174f86e",
                "034a291c7a5b20069362d4055cbb4a18be38bcc31759dcfa30f4b9549e2177ef54",
            ),
            (
                "807d61dd970513bf4c7789c35d0a4bf46075e8958a5a0f865b8dccdb447d919a",
                "024a0e3fe44613fedb13befda0740e83062d40106809b329284bb265c17e253717",
            ),
            (
                "904c271f08df65d2ca2154497af2414ddabe48f9e69aac6591349ec7313c3178",
                "03f6b3c72240fb93f446ee774f3f23f8c41785f5064d9b4e848d86e5b954d5325a",
            ),
            (
                "d48ade506f9757f642011aa322395bfa6640a876ae9b26044306e128f3758b85",
                "03b13e982ffd1565489c38e64f33452b476222b0f0f6e2f64dd981576c5bbfb40b",
            ),
            (
                "520a4b2a98013b94c7f427a267cffd1674f42603468cce0a08c41e47ac74a77a",
                "03bf6e9a808258ad4d77e4ca7467c9e3d43fdd2d1eca23c7885fbd402afd8df6ba",
            ),
            (
                "e7a1331123353bf52e7b8c6d953af6e622e5de7c70ff988944cd38e3c9d21646",
                "02d95fddfa27244c2c6f473e17ae8cfff66164a87a886f4a0372f6fe9c9db9a3c8",
            ),
            (
                "40b798397eeb99e62f2408abe10c0bd7ef82a52a4c19222dbc64197ac216df1c",
                "037d8238d24d8e7d382e629e3373c284ebb07f8198a28022e25d21090c484202bc",
            ),
            (
                "d83c53362cdaf6ddea098af18d70ec411e38cb0b0b21d7ec38e3cc6dfe91578d",
                "02988d161e8c8f24d9a45c1796548120d3828a7c556a50729762ae5a637f57d280",
            ),
            (
                "5b5472d4199fb686812bc4c68132461b574cdd951905734707f1fb2839a87777",
                "02fc5c950b1bda38e4c4881c8bcdddf60eab7722d9e616427e53a193d75d84ffad",
            ),
            (
                "03278fbac999526e5a2f090f1946a0d9c52d60e7716acebe8ea7102e85b178d8",
                "024a347fa287de51490250f4516abafd433e910eb1ebfec1cf6b55d74b87997863",
            ),
            (
                "c00683de18afef4af81413f3e5f4c684b07d04ace86b61f37b2288117ba94189",
                "031b77022af2018af87d6acbc9a46bc32333b1ebbbefc60dfc4a64c93be2bb1f99",
            ),
            (
                "3d9725771521bce8a2ae60025650c5e626814cb6aa244172b25fb7221e0059f3",
                "0229c12839bf6f432f9b825f80f4d2dc5f5208ef829f70ebdf5f6efedb234f8e69",
            ),
            (
                "47ae158e8d74e444aaea811367c19af2dbe01670f7608ac3fc63e0f9f076b622",
                "020d59f98639ad3398837a0323f751cb2b019c694fb935d0a156e3aae318d5ea58",
            ),
            (
                "46ad7b3a8cb742566fb62719a31fe6c63b9715a5ccf7990d9c27906480fe7550",
                "0233e1e6a69e1a6158bb64d5bdf5e4629580d4c8227377a3698d15afdb52af3b54",
            ),
            (
                "0ca76e8c7a1a48c638b98cc433de6aba6b7f94858893700032e3bb900de9570e",
                "031d035f3ab1690143953344bee9f971b82065df54e1e888c3891fd247250d7299",
            ),
            (
                "00d22e8eb2816d3ec1eb09ede996c1b75068408d84c39a474b943d858e8b897b",
                "0261fd6c959551374edcba2c7b2749088b813f494341c23b1f626b7a0f3ee6909d",
            ),
            (
                "4c76268bdf05a83d43fdec914483bd3c66cf84fb8589b6cb95aed0a68c18d61f",
                "0396ad5e8b8771335b789127203ac869fd2dc02adb021ccc80c25319ae3a81737f",
            ),
            (
                "cf243ad9de11d72c2842a069153c55a15f7380f423d16ec60d4b422f84b4b011",
                "030240085f4168e23f3f278c6c85f4362f3657551191df328c5e0291d97f156973",
            ),
        ];
    }

    #[test]
    fn test_jacobian_points() {
        let test_vectors: [(&str, &str); 21] = get_test_vectors();

        for i in 0..test_vectors.len() {
            let secp256k1 = curves::get_curve("secp256k1");

            let (private_key, public_key) = test_vectors[i];

            let k = BigInt::parse_bytes(private_key.as_bytes(), 16).unwrap();

            let p = secp256k1.g.multiply(k.clone());

            let pub_key = compress_point(&p);

            assert_eq!(public_key, pub_key);
        }
    }

    #[test]
    fn test_affine_points() {
        let test_vectors: [(&str, &str); 21] = get_test_vectors();

        for i in 0..test_vectors.len() {
            let secp256k1 = curves::get_curve("secp256k1");

            let (private_key, public_key) = test_vectors[i];

            let k = BigInt::parse_bytes(private_key.as_bytes(), 16).unwrap();

            let p = secp256k1.g.to_affine().multiply(k.clone());

            let affine_pub_key = compress_point(&p);

            assert_eq!(public_key, affine_pub_key);
        }
    }

    #[test]
    fn test_windowed_method() {
        let test_vectors: [(&str, &str); 21] = get_test_vectors();

        for i in 0..test_vectors.len() {
            let secp256k1 = curves::get_curve("secp256k1");

            let (private_key, public_key) = test_vectors[i];

            let k = BigInt::parse_bytes(private_key.as_bytes(), 16).unwrap();

            let precomputes = get_window_precomputes(&secp256k1.g.to_affine(), None);

            let p = secp256k1
                .g
                .to_affine()
                .multiply_with_windowed_method(k.clone(), &precomputes);

            let windowed_public_key = compress_point(&p);

            assert_eq!(public_key, windowed_public_key);
        }
    }

    #[test]
    fn test_wnaf_method() {
        let test_vectors: [(&str, &str); 21] = get_test_vectors();

        for i in 0..test_vectors.len() {
            const WIDTH: u32 = 4;

            let secp256k1 = curves::get_curve("secp256k1");

            let (private_key, public_key) = test_vectors[i];

            let k = BigInt::parse_bytes(private_key.as_bytes(), 16).unwrap();

            let precomputes = precompute_points(secp256k1.g.to_affine().clone(), WIDTH);

            let p = secp256k1.g.to_affine().multiply_with_non_adjacent_form(
                k.clone(),
                WIDTH,
                &precomputes,
            );

            let windowed_public_key = compress_point(&p);

            assert_eq!(public_key, windowed_public_key);
        }
    }
}
