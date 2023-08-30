
pub const SECT163k1_NAME :&str = "SECT163k1";
pub const SECT163r1_NAME :&str = "SECT163r1";
pub const SECT239k1_NAME :&str = "SECT239k1";
pub const SECT113r1_NAME :&str = "SECT113r1";
pub const SECT113r2_NAME :&str = "SECT113r2";
pub const SECP112r1_NAME :&str = "SECP112r1";
pub const SECP112r2_NAME :&str = "SECP112r2";
pub const SECP160r1_NAME :&str = "SECP160r1";
pub const SECP160k1_NAME :&str = "SECP160k1";
pub const SECP256k1_NAME :&str = "SECP256k1";
pub const SECT163r2_NAME :&str = "SECT163r2";
pub const SECT283k1_NAME :&str = "SECT283k1";
pub const SECT283r1_NAME :&str = "SECT283r1";
pub const SECT131r1_NAME :&str = "SECT131r1";
pub const SECT131r2_NAME :&str = "SECT131r2";
pub const SECT193r1_NAME :&str = "SECT193r1";
pub const SECT193r2_NAME :&str = "SECT193r2";
pub const SECT233k1_NAME :&str = "SECT233k1";
pub const SECT233r1_NAME :&str = "SECT233r1";
pub const SECP128r1_NAME :&str = "SECP128r1";
pub const SECP128r2_NAME :&str = "SECP128r2";
pub const SECP160r2_NAME :&str = "SECP160r2";
pub const SECP192k1_NAME :&str = "SECP192k1";
pub const SECP224k1_NAME :&str = "SECP224k1";
pub const SECP224r1_NAME :&str = "SECP224r1";
pub const SECP384r1_NAME :&str = "SECP384r1";
pub const SECP521r1_NAME :&str = "SECP521r1";
pub const SECT409k1_NAME :&str = "SECT409k1";
pub const SECT409r1_NAME :&str = "SECT409r1";
pub const SECT571k1_NAME :&str = "SECT571k1";
pub const SECT571r1_NAME :&str = "SECT571r1";
pub const PRIME192v1_NAME :&str = "PRIME192v1";

pub const SECT163k1_OID :&str = "1.3.132.0.1";
pub const SECT163r1_OID :&str = "1.3.132.0.2";
pub const SECT239k1_OID :&str = "1.3.132.0.3";
pub const SECT113r1_OID :&str = "1.3.132.0.4";
pub const SECT113r2_OID :&str = "1.3.132.0.5";
pub const SECP112r1_OID :&str = "1.3.132.0.6";
pub const SECP112r2_OID :&str = "1.3.132.0.7";
pub const SECP160r1_OID :&str = "1.3.132.0.8";
pub const SECP160k1_OID :&str = "1.3.132.0.9";
pub const SECP256k1_OID :&str = "1.3.132.0.10";
pub const SECT163r2_OID :&str = "1.3.132.0.15";
pub const SECT283k1_OID :&str = "1.3.132.0.16";
pub const SECT283r1_OID :&str = "1.3.132.0.17";
pub const SECT131r1_OID :&str = "1.3.132.0.22";
pub const SECT131r2_OID :&str = "1.3.132.0.23";
pub const SECT193r1_OID :&str = "1.3.132.0.24";
pub const SECT193r2_OID :&str = "1.3.132.0.25";
pub const SECT233k1_OID :&str = "1.3.132.0.26";
pub const SECT233r1_OID :&str = "1.3.132.0.27";
pub const SECP128r1_OID :&str = "1.3.132.0.28";
pub const SECP128r2_OID :&str = "1.3.132.0.29";
pub const SECP160r2_OID :&str = "1.3.132.0.30";
pub const SECP192k1_OID :&str = "1.3.132.0.31";
pub const SECP224k1_OID :&str = "1.3.132.0.32";
pub const SECP224r1_OID :&str = "1.3.132.0.33";
pub const SECP384r1_OID :&str = "1.3.132.0.34";
pub const SECP521r1_OID :&str = "1.3.132.0.35";
pub const SECT409k1_OID :&str = "1.3.132.0.36";
pub const SECT409r1_OID :&str = "1.3.132.0.37";
pub const SECT571k1_OID :&str = "1.3.132.0.38";
pub const SECT571r1_OID :&str = "1.3.132.0.39";

pub const PRIME192v1_OID :&str = "1.2.840.10045.3.1.1";

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


pub const EC_CODE_MASK :u8 = 0xfe;
pub const EC_CODE_YBIT :u8 = 0x1;
pub const EC_CODE_COMPRESSED : u8 = 0x2;
pub const EC_CODE_UNCOMPRESSED : u8 = 0x4;
pub const EC_CODE_HYBRID :u8 = 0x6;

pub (crate) const MAX_ITERATIONS :i32 = 50;
pub (crate) const MONT_BIT_SIZE : i64 = 0x40;

