extern crate num_bigint_dig as num_bigint2;

use hex::FromHex;
use crate::jacobi::{PointJacobi,CurveFp};
use crate::*;
use crate::consts::*;
use lazy_static::lazy_static;
use std::collections::HashMap;
use num_bigint::{BigInt,Sign};
use num_traits::{one};

use std::error::Error;

ecsimple_error_class!{EcSimpleCurveError}

#[derive(Clone,Debug)]
pub struct ECCCurve {
	pub generator :PointJacobi,
	pub name :String,
	pub order :BigInt,
	pub curve :CurveFp,
}


impl std::cmp::PartialEq<ECCCurve> for ECCCurve {
    fn eq(&self,other :&Self) -> bool {
        return self.eq_curve(other);
    }

    fn ne(&self, other :&Self) -> bool {
        return  !self.eq(other);
    }	
}

impl ECCCurve {
	fn eq_curve(&self,other :&ECCCurve) -> bool {
		if self.generator != other.generator {
			return false;
		}

		if self.order != other.order {
			return false;
		}

		if self.curve != other.curve {
			return false;
		}
		return true;
	}

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

_p = int(remove_whitespace("0800000000000000000000000000000000000000C9"), 16)
# s = 00F50B02 8E4D696E 67687561 51752904 72783FB1
_a = int(remove_whitespace("000000000000000000000000000000000000000001"), 16)
_b = int(remove_whitespace("000000000000000000000000000000000000000001"), 16)
_Gx = int(remove_whitespace("02FE13C0537BBC11ACAA07D793DE4E6D5E5C94EEE8"), 16)
_Gy = int(remove_whitespace("0289070FB05D38FF58321F2E800536D538CCDAA3D9"), 16)
_r = int(remove_whitespace("04000000000000000000020108A2E0CC0D99F8A5EF"), 16)
_h = 2
curve_ansit163k1 = ellipticcurve.CurveFp(_p, _a, _b, _h)
generator_112r1 = ellipticcurve.PointJacobi(
    curve_112r1, _Gx, _Gy, 1, _r, generator=False
)


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

	v8 = Vec::from_hex("0800000000000000000000000000000000000000C9").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("000000000000000000000000000000000000000001").unwrap();
	a = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("000000000000000000000000000000000000000001").unwrap();
	b = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("02FE13C0537BBC11ACAA07D793DE4E6D5E5C94EEE8").unwrap();
	gx = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("0289070FB05D38FF58321F2E800536D538CCDAA3D9").unwrap();
	gy = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("04000000000000000000020108A2E0CC0D99F8A5EF").unwrap();
	r = BigInt::from_bytes_be(Sign::Plus,&v8);
	//v8 = Vec::from_hex("01").unwrap();
	h = ov.clone() + ov.clone();

	curve = CurveFp::new(&p,&a,&b,&h);
	japt = PointJacobi::new(&curve,&gx,&gy,&ov,Some(r.clone()),false);

	retv.insert(SECT163k1_NAME.to_string(),ECCCurve::new(SECT163k1_NAME,&japt));


	v8 = Vec::from_hex("0800000000000000000000000000000000000000C9").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("07B6882CAAEFA84F9554FF8428BD88E246D2782AE2").unwrap();
	a = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("0713612DCDDCB40AAB946BDA29CA91F73AF958AFD9").unwrap();
	b = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("0369979697AB43897789566789567F787A7876A654").unwrap();
	gx = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("00435EDB42EFAFB2989D51FEFCE3C80988F41FF883").unwrap();
	gy = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("03FFFFFFFFFFFFFFFFFFFF48AAB689C29CA710279B").unwrap();
	r = BigInt::from_bytes_be(Sign::Plus,&v8);
	//v8 = Vec::from_hex("01").unwrap();
	h = ov.clone() + ov.clone();

	curve = CurveFp::new(&p,&a,&b,&h);
	japt = PointJacobi::new(&curve,&gx,&gy,&ov,Some(r.clone()),false);

	retv.insert(SECT163r1_NAME.to_string(),ECCCurve::new(SECT163r1_NAME,&japt));



	v8 = Vec::from_hex("800000000000000000004000000000000000000000000000000000000001").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("000000000000000000000000000000000000000000000000000000000000").unwrap();
	a = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("000000000000000000000000000000000000000000000000000000000001").unwrap();
	b = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("29A0B6A887A983E9730988A68727A8B2D126C44CC2CC7B2A6555193035DC").unwrap();
	gx = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("76310804F12E549BDB011C103089E73510ACB275FC312A5DC6B76553F0CA").unwrap();
	gy = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("2000000000000000000000000000005A79FEC67CB6E91F1C1DA800E478A5").unwrap();
	r = BigInt::from_bytes_be(Sign::Plus,&v8);
	//v8 = Vec::from_hex("01").unwrap();
	h = ov.clone() + ov.clone() + ov.clone() + ov.clone();

	curve = CurveFp::new(&p,&a,&b,&h);
	japt = PointJacobi::new(&curve,&gx,&gy,&ov,Some(r.clone()),false);

	retv.insert(SECT239k1_NAME.to_string(),ECCCurve::new(SECT239k1_NAME,&japt));


	v8 = Vec::from_hex("020000000000000000000000000201").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("003088250CA6E7C7FE649CE85820F7").unwrap();
	a = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("00E8BEE4D3E2260744188BE0E9C723").unwrap();
	b = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("009D73616F35F4AB1407D73562C10F").unwrap();
	gx = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("00A52830277958EE84D1315ED31886").unwrap();
	gy = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("0100000000000000D9CCEC8A39E56F").unwrap();
	r = BigInt::from_bytes_be(Sign::Plus,&v8);
	//v8 = Vec::from_hex("01").unwrap();
	h = ov.clone() + ov.clone();

	curve = CurveFp::new(&p,&a,&b,&h);
	japt = PointJacobi::new(&curve,&gx,&gy,&ov,Some(r.clone()),false);

	retv.insert(SECT113r1_NAME.to_string(),ECCCurve::new(SECT113r1_NAME,&japt));



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
	retv.insert(SECT163k1_NAME.to_string(),SECT163k1_OID.to_string());
	retv.insert(SECT163r1_NAME.to_string(),SECT163r1_OID.to_string());
	retv.insert(SECT239k1_NAME.to_string(),SECT239k1_OID.to_string());
	retv.insert(SECT113r1_NAME.to_string(),SECT113r1_OID.to_string());
	retv.insert(SECP112r1_NAME.to_string(),SECP112r1_OID.to_string());
	retv.insert(SECP112r2_NAME.to_string(),SECP112r2_OID.to_string());

	retv
}

fn create_curve_name() -> HashMap<String,String> {
	let mut retv :HashMap<String,String> = HashMap::new();
	retv.insert(SECT163k1_OID.to_string(),SECT163k1_NAME.to_string());
	retv.insert(SECT163r1_OID.to_string(),SECT163r1_NAME.to_string());
	retv.insert(SECT239k1_OID.to_string(),SECT239k1_NAME.to_string());
	retv.insert(SECT113r1_OID.to_string(),SECT113r1_NAME.to_string());
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

