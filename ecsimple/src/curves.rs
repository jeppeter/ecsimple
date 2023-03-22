extern crate num_bigint_dig as num_bigint2;

use hex::FromHex;
use crate::jacobi::{PointJacobi,CurveFp};
use crate::*;
use lazy_static::lazy_static;
use std::collections::HashMap;
use num_bigint::{BigInt,Sign};
use num_traits::{one};

use std::error::Error;

ecsimple_error_class!{EcSimpleCurveError}

const SECP112r1_NAME :&str = "SECP112r1";
const SECP112r2_NAME :&str = "SECP112r2";


const SECP112r1_OID :&str = "1.3.132.0.6";
const SECP112r2_OID :&str = "1.3.132.0.6";

pub (crate) const EC_PUBLIC_KEY_OID :&str = "1.2.840.10045.2.1";
pub (crate) const ID_PRIME_FIELD_OID :&str = "1.2.840.10045.1.1";

#[derive(Clone,Debug)]
pub struct ECCCurve {
	pub generator :PointJacobi,
	pub name :String,
	pub order :BigInt,
	pub curve :CurveFp,
}

impl ECCCurve {
	pub fn new(name :&str,generator :&PointJacobi) -> ECCCurve {
		ECCCurve {
			generator : generator.clone(),
			name : name.to_string(),
			order : generator.order(),
			curve :generator.curve(),
		}
	}
}

/*
_p = int(remove_whitespace("DB7C 2ABF62E3 5E668076 BEAD208B"), 16)
# s = 00F50B02 8E4D696E 67687561 51752904 72783FB1
_a = int(remove_whitespace("DB7C 2ABF62E3 5E668076 BEAD2088"), 16)
_b = int(remove_whitespace("659E F8BA0439 16EEDE89 11702B22"), 16)
_Gx = int(remove_whitespace("09487239 995A5EE7 6B55F9C2 F098"), 16)
_Gy = int(remove_whitespace("A89C E5AF8724 C0A23E0E 0FF77500"), 16)
_r = int(remove_whitespace("DB7C 2ABF62E3 5E7628DF AC6561C5"), 16)
_h = 1
curve_112r1 = ellipticcurve.CurveFp(_p, _a, _b, _h)
generator_112r1 = ellipticcurve.PointJacobi(
    curve_112r1, _Gx, _Gy, 1, _r, generator=False
)


_p = int(remove_whitespace("DB7C 2ABF62E3 5E668076 BEAD208B"), 16)
# s = 022757A1 114D69E 67687561 51755316 C05E0BD4
_a = int(remove_whitespace("6127 C24C05F3 8A0AAAF6 5C0EF02C"), 16)
_b = int(remove_whitespace("51DE F1815DB5 ED74FCC3 4C85D709"), 16)
_Gx = int(remove_whitespace("4BA30AB5 E892B4E1 649DD092 8643"), 16)
_Gy = int(remove_whitespace("ADCD 46F5882E 3747DEF3 6E956E97"), 16)
_r = int(remove_whitespace("36DF 0AAFD8B8 D7597CA1 0520D04B"), 16)
_h = 4
curve_112r2 = ellipticcurve.CurveFp(_p, _a, _b, _h)
generator_112r2 = ellipticcurve.PointJacobi(
    curve_112r2, _Gx, _Gy, 1, _r, generator=True
)

*/

fn create_jacobi() -> HashMap<String,ECCCurve> {
	let mut retv :HashMap<String,ECCCurve> = HashMap::new();
	let mut p :BigInt;
	let mut a :BigInt;
	let mut b :BigInt;
	let mut gx :BigInt;
	let mut gy :BigInt;
	let mut r :BigInt;
	let mut h :BigInt;
	let mut v8 :Vec<u8>;
	let mut curve :CurveFp;
	let mut japt :PointJacobi;
	let ov :BigInt = one::<BigInt>();

	v8 = Vec::from_hex("DB7C2ABF62E35E668076BEAD208B").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("DB7C2ABF62E35E668076BEAD2088").unwrap();
	a = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("659EF8BA043916EEDE8911702B22").unwrap();
	b = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("09487239995A5EE76B55F9C2F098").unwrap();
	gx = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("A89CE5AF8724C0A23E0E0FF77500").unwrap();
	gy = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("DB7C2ABF62E35E7628DFAC6561C5").unwrap();
	r = BigInt::from_bytes_be(Sign::Plus,&v8);
	//v8 = Vec::from_hex("01").unwrap();
	h = ov.clone();

	curve = CurveFp::new(&p,&a,&b,&h);
	japt = PointJacobi::new(&curve,&gx,&gy,&ov,Some(r.clone()),false);

	retv.insert(SECP112r1_NAME.to_string(),ECCCurve::new(SECP112r1_NAME,&japt));


	v8 = Vec::from_hex("DB7C2ABF62E35E668076BEAD208B").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("6127C24C05F38A0AAAF65C0EF02C").unwrap();
	a = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("51DEF1815DB5ED74FCC34C85D709").unwrap();
	b = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("4BA30AB5E892B4E1649DD0928643").unwrap();
	gx = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("ADCD46F5882E3747DEF36E956E97").unwrap();
	gy = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("36DF0AAFD8B8D7597CA10520D04B").unwrap();
	r = BigInt::from_bytes_be(Sign::Plus,&v8);
	//v8 = Vec::from_hex("4").unwrap();
	//h = BigInt::from_bytes_be(Sign::Plus,&v8);
	h = (&ov) + (&ov) + (&ov) + (&ov);

	curve = CurveFp::new(&p,&a,&b,&h);
	japt = PointJacobi::new(&curve,&gx,&gy,&ov,Some(r.clone()),true);

	retv.insert(SECP112r2_NAME.to_string(),ECCCurve::new(SECP112r2_NAME,&japt));
	retv
}

fn create_curve_oid() -> HashMap<String,String> {
	let mut retv :HashMap<String,String> = HashMap::new();
	retv.insert(SECP112r1_NAME.to_string(),SECP112r1_OID.to_string());
	retv.insert(SECP112r2_NAME.to_string(),SECP112r2_OID.to_string());

	retv
}

fn create_curve_name() -> HashMap<String,String> {
	let mut retv :HashMap<String,String> = HashMap::new();
	retv.insert(SECP112r1_OID.to_string(),SECP112r1_NAME.to_string());
	retv.insert(SECP112r2_OID.to_string(),SECP112r2_NAME.to_string());

	retv
}

lazy_static ! {
	static ref ECC_CURVES :HashMap<String,ECCCurve> = {
		create_jacobi()	
	};

	static ref ECC_CURVE_OIDS :HashMap<String,String> = {
		create_curve_oid()
	};

	static ref ECC_CURVE_NAMES :HashMap<String,String> = {
		create_curve_name()
	};
}

pub fn get_ecc_curve_by_name(name :&str) -> Result<ECCCurve,Box<dyn Error>> {
	match ECC_CURVES.get(name) {
		Some(pv) => {
			return Ok(pv.clone());
		},
		_ => {
			ecsimple_new_error!{EcSimpleCurveError,"can not find [{}]",name}
		}
	}
}

pub fn get_ecc_oid_by_name(name :&str) -> Result<String,Box<dyn Error>> {
	match ECC_CURVE_OIDS.get(name) {
		Some(pv) => {
			return Ok(format!("{}",pv));
		}
		_ => {
			ecsimple_new_error!{EcSimpleCurveError,"can not find [{}]", name}
		}
	}
}

pub fn get_ecc_name_by_oid(oid :&str) -> Result<String,Box<dyn Error>> {
	match ECC_CURVE_NAMES.get(oid) {
		Some(pv) => {
			return Ok(format!("{}",pv));
		}
		_ => {
			ecsimple_new_error!{EcSimpleCurveError,"can not find [{}]", oid}
		}
	}
}

