

pub const SECP112r1_NAME :&str = "SECP112r1";
pub const SECP112r2_NAME :&str = "SECP112r2";


pub const SECP112r1_OID :&str = "1.3.132.0.6";
pub const SECP112r2_OID :&str = "1.3.132.0.7";

pub const EC_PUBLIC_KEY_OID :&str = "1.2.840.10045.2.1";
pub const ID_PRIME_FIELD_OID :&str = "1.2.840.10045.1.1";


pub const EC_COMPRESSED :&str = "compressed";
pub const EC_UNCOMPRESSED :&str = "uncompressed";
pub const EC_HYBRID :&str = "hybrid";
pub const EC_PARAMS_EXLICIT :&str = "explicit";

pub const EC_SSLEAY_TYPE :&str = "ssleay";
pub const EC_PKCS8_TYPE :&str = "pkcs8";

pub (crate) const EC_ENC_DATA_SIMPLE :u8 = 0x1;
pub (crate) const EC_ENC_DATA_MASK :u8 = 0x3f;
#[allow(dead_code)]
pub (crate) const EC_ENC_DATA_MASK_SHIFT :usize = 0;
pub (crate) const EC_ENC_DATA_SIZE_MASK :u8 = 0x3;
pub (crate) const EC_ENC_DATA_SIZE_SHIFT :usize = 0x6;

pub (crate) const EC_ENC_DATA_1_BYTE_MAX :usize = 0xff;
pub (crate) const EC_ENC_DATA_2_BYTE_MAX :usize = 0xffff;
pub (crate) const EC_ENC_DATA_3_BYTE_MAX :usize = 0xffffff;
pub (crate) const EC_ENC_DATA_4_BYTE_MAX :usize = 0xffffffff;