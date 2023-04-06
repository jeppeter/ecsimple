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


	v8 = Vec::from_hex("020000000000000000000000000201").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("00689918DBEC7E5A0DD6DFC0AA55C7").unwrap();
	a = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("0095E9A9EC9B297BD4BF36E059184F").unwrap();
	b = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("01A57A6A7B26CA5EF52FCDB8164797").unwrap();
	gx = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("00B3ADC94ED1FE674C06E695BABA1D").unwrap();
	gy = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("010000000000000108789B2496AF93").unwrap();
	r = BigInt::from_bytes_be(Sign::Plus,&v8);
	//v8 = Vec::from_hex("01").unwrap();
	h = ov.clone() + ov.clone();

	curve = CurveFp::new(&p,&a,&b,&h);
	japt = PointJacobi::new(&curve,&gx,&gy,&ov,Some(r.clone()),false);

	retv.insert(SECT113r2_NAME.to_string(),ECCCurve::new(SECT113r2_NAME,&japt));



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


	v8 = Vec::from_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF7FFFFFFF").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF7FFFFFFC").unwrap();
	a = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("1C97BEFC54BD7A8B65ACF89F81D4D4ADC565FA45").unwrap();
	b = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("4A96B5688EF573284664698968C38BB913CBFC82").unwrap();
	gx = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("23A628553168947D59DCC912042351377AC5FB32").unwrap();
	gy = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("0100000000000000000001F4C8F927AED3CA752257").unwrap();
	r = BigInt::from_bytes_be(Sign::Plus,&v8);
	//v8 = Vec::from_hex("4").unwrap();
	//h = BigInt::from_bytes_be(Sign::Plus,&v8);
	h = ov.clone();

	curve = CurveFp::new(&p,&a,&b,&h);
	japt = PointJacobi::new(&curve,&gx,&gy,&ov,Some(r.clone()),true);

	retv.insert(SECP160r1_NAME.to_string(),ECCCurve::new(SECP160r1_NAME,&japt));


	v8 = Vec::from_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFAC73").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("0000000000000000000000000000000000000000").unwrap();
	a = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("0000000000000000000000000000000000000007").unwrap();
	b = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("3B4C382CE37AA192A4019E763036F4F5DD4D7EBB").unwrap();
	gx = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("938CF935318FDCED6BC28286531733C3F03C4FEE").unwrap();
	gy = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("0100000000000000000001B8FA16DFAB9ACA16B6B3").unwrap();
	r = BigInt::from_bytes_be(Sign::Plus,&v8);
	//v8 = Vec::from_hex("4").unwrap();
	//h = BigInt::from_bytes_be(Sign::Plus,&v8);
	h = ov.clone();

	curve = CurveFp::new(&p,&a,&b,&h);
	japt = PointJacobi::new(&curve,&gx,&gy,&ov,Some(r.clone()),true);

	retv.insert(SECP160k1_NAME.to_string(),ECCCurve::new(SECP160k1_NAME,&japt));


	v8 = Vec::from_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("0000000000000000000000000000000000000000000000000000000000000000").unwrap();
	a = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("0000000000000000000000000000000000000000000000000000000000000007").unwrap();
	b = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798").unwrap();
	gx = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8").unwrap();
	gy = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141").unwrap();
	r = BigInt::from_bytes_be(Sign::Plus,&v8);
	//v8 = Vec::from_hex("4").unwrap();
	//h = BigInt::from_bytes_be(Sign::Plus,&v8);
	h = ov.clone();

	curve = CurveFp::new(&p,&a,&b,&h);
	japt = PointJacobi::new(&curve,&gx,&gy,&ov,Some(r.clone()),true);

	retv.insert(SECP256k1_NAME.to_string(),ECCCurve::new(SECP256k1_NAME,&japt));


	v8 = Vec::from_hex("0800000000000000000000000000000000000000C9").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("000000000000000000000000000000000000000001").unwrap();
	a = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("020A601907B8C953CA1481EB10512F78744A3205FD").unwrap();
	b = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("03F0EBA16286A2D57EA0991168D4994637E8343E36").unwrap();
	gx = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("00D51FBC6C71A0094FA2CDD545B11C5C0C797324F1").unwrap();
	gy = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("040000000000000000000292FE77E70C12A4234C33").unwrap();
	r = BigInt::from_bytes_be(Sign::Plus,&v8);
	//v8 = Vec::from_hex("4").unwrap();
	//h = BigInt::from_bytes_be(Sign::Plus,&v8);
	h = ov.clone() + ov.clone();

	curve = CurveFp::new(&p,&a,&b,&h);
	japt = PointJacobi::new(&curve,&gx,&gy,&ov,Some(r.clone()),true);

	retv.insert(SECT163r2_NAME.to_string(),ECCCurve::new(SECT163r2_NAME,&japt));

	v8 = Vec::from_hex("0800000000000000000000000000000000000000000000000000000000000000000010A1").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("000000000000000000000000000000000000000000000000000000000000000000000000").unwrap();
	a = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("000000000000000000000000000000000000000000000000000000000000000000000001").unwrap();
	b = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("0503213F78CA44883F1A3B8162F188E553CD265F23C1567A16876913B0C2AC2458492836").unwrap();
	gx = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("01CCDA380F1C9E318D90F95D07E5426FE87E45C0E8184698E45962364E34116177DD2259").unwrap();
	gy = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("01FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFE9AE2ED07577265DFF7F94451E061E163C61").unwrap();
	r = BigInt::from_bytes_be(Sign::Plus,&v8);
	//v8 = Vec::from_hex("4").unwrap();
	//h = BigInt::from_bytes_be(Sign::Plus,&v8);
	h = ov.clone() + ov.clone() + ov.clone() + ov.clone();

	curve = CurveFp::new(&p,&a,&b,&h);
	japt = PointJacobi::new(&curve,&gx,&gy,&ov,Some(r.clone()),true);

	retv.insert(SECT283k1_NAME.to_string(),ECCCurve::new(SECT283k1_NAME,&japt));

	v8 = Vec::from_hex("0800000000000000000000000000000000000000000000000000000000000000000010A1").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("000000000000000000000000000000000000000000000000000000000000000000000001").unwrap();
	a = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("027B680AC8B8596DA5A4AF8A19A0303FCA97FD7645309FA2A581485AF6263E313B79A2F5").unwrap();
	b = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("05F939258DB7DD90E1934F8C70B0DFEC2EED25B8557EAC9C80E2E198F8CDBECD86B12053").unwrap();
	gx = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("03676854FE24141CB98FE6D4B20D02B4516FF702350EDDB0826779C813F0DF45BE8112F4").unwrap();
	gy = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("03FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEF90399660FC938A90165B042A7CEFADB307").unwrap();
	r = BigInt::from_bytes_be(Sign::Plus,&v8);
	//v8 = Vec::from_hex("4").unwrap();
	//h = BigInt::from_bytes_be(Sign::Plus,&v8);
	h = ov.clone() + ov.clone() ;

	curve = CurveFp::new(&p,&a,&b,&h);
	japt = PointJacobi::new(&curve,&gx,&gy,&ov,Some(r.clone()),true);

	retv.insert(SECT283r1_NAME.to_string(),ECCCurve::new(SECT283r1_NAME,&japt));

	v8 = Vec::from_hex("080000000000000000000000000000010D").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("07A11B09A76B562144418FF3FF8C2570B8").unwrap();
	a = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("0217C05610884B63B9C6C7291678F9D341").unwrap();
	b = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("0081BAF91FDF9833C40F9C181343638399").unwrap();
	gx = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("078C6E7EA38C001F73C8134B1B4EF9E150").unwrap();
	gy = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("0400000000000000023123953A9464B54D").unwrap();
	r = BigInt::from_bytes_be(Sign::Plus,&v8);
	//v8 = Vec::from_hex("4").unwrap();
	//h = BigInt::from_bytes_be(Sign::Plus,&v8);
	h = ov.clone() + ov.clone() ;

	curve = CurveFp::new(&p,&a,&b,&h);
	japt = PointJacobi::new(&curve,&gx,&gy,&ov,Some(r.clone()),true);

	retv.insert(SECT131r1_NAME.to_string(),ECCCurve::new(SECT131r1_NAME,&japt));

	v8 = Vec::from_hex("080000000000000000000000000000010D").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("03E5A88919D7CAFCBF415F07C2176573B2").unwrap();
	a = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("04B8266A46C55657AC734CE38F018F2192").unwrap();
	b = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("0356DCD8F2F95031AD652D23951BB366A8").unwrap();
	gx = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("0648F06D867940A5366D9E265DE9EB240F").unwrap();
	gy = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("0400000000000000016954A233049BA98F").unwrap();
	r = BigInt::from_bytes_be(Sign::Plus,&v8);
	//v8 = Vec::from_hex("4").unwrap();
	//h = BigInt::from_bytes_be(Sign::Plus,&v8);
	h = ov.clone() + ov.clone() ;

	curve = CurveFp::new(&p,&a,&b,&h);
	japt = PointJacobi::new(&curve,&gx,&gy,&ov,Some(r.clone()),true);

	retv.insert(SECT131r2_NAME.to_string(),ECCCurve::new(SECT131r2_NAME,&japt));

	v8 = Vec::from_hex("02000000000000000000000000000000000000000000008001").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("0017858FEB7A98975169E171F77B4087DE098AC8A911DF7B01").unwrap();
	a = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("00FDFB49BFE6C3A89FACADAA7A1E5BBC7CC1C2E5D831478814").unwrap();
	b = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("01F481BC5F0FF84A74AD6CDF6FDEF4BF6179625372D8C0C5E1").unwrap();
	gx = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("0025E399F2903712CCF3EA9E3A1AD17FB0B3201B6AF7CE1B05").unwrap();
	gy = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("01000000000000000000000000C7F34A778F443ACC920EBA49").unwrap();
	r = BigInt::from_bytes_be(Sign::Plus,&v8);
	//v8 = Vec::from_hex("4").unwrap();
	//h = BigInt::from_bytes_be(Sign::Plus,&v8);
	h = ov.clone() + ov.clone() ;

	curve = CurveFp::new(&p,&a,&b,&h);
	japt = PointJacobi::new(&curve,&gx,&gy,&ov,Some(r.clone()),true);

	retv.insert(SECT193r1_NAME.to_string(),ECCCurve::new(SECT193r1_NAME,&japt));

	v8 = Vec::from_hex("02000000000000000000000000000000000000000000008001").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("0163F35A5137C2CE3EA6ED8667190B0BC43ECD69977702709B").unwrap();
	a = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("00C9BB9E8927D4D64C377E2AB2856A5B16E3EFB7F61D4316AE").unwrap();
	b = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("00D9B67D192E0367C803F39E1A7E82CA14A651350AAE617E8F").unwrap();
	gx = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("01CE94335607C304AC29E7DEFBD9CA01F596F927224CDECF6C").unwrap();
	gy = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("010000000000000000000000015AAB561B005413CCD4EE99D5").unwrap();
	r = BigInt::from_bytes_be(Sign::Plus,&v8);
	//v8 = Vec::from_hex("4").unwrap();
	//h = BigInt::from_bytes_be(Sign::Plus,&v8);
	h = ov.clone() + ov.clone() ;

	curve = CurveFp::new(&p,&a,&b,&h);
	japt = PointJacobi::new(&curve,&gx,&gy,&ov,Some(r.clone()),true);

	retv.insert(SECT193r2_NAME.to_string(),ECCCurve::new(SECT193r2_NAME,&japt));

	v8 = Vec::from_hex("020000000000000000000000000000000000000004000000000000000001").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("000000000000000000000000000000000000000000000000000000000000").unwrap();
	a = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("000000000000000000000000000000000000000000000000000000000001").unwrap();
	b = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("017232BA853A7E731AF129F22FF4149563A419C26BF50A4C9D6EEFAD6126").unwrap();
	gx = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("01DB537DECE819B7F70F555A67C427A8CD9BF18AEB9B56E0C11056FAE6A3").unwrap();
	gy = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("008000000000000000000000000000069D5BB915BCD46EFB1AD5F173ABDF").unwrap();
	r = BigInt::from_bytes_be(Sign::Plus,&v8);
	//v8 = Vec::from_hex("4").unwrap();
	//h = BigInt::from_bytes_be(Sign::Plus,&v8);
	h = ov.clone() + ov.clone() +ov.clone() + ov.clone() ;

	curve = CurveFp::new(&p,&a,&b,&h);
	japt = PointJacobi::new(&curve,&gx,&gy,&ov,Some(r.clone()),true);

	retv.insert(SECT233k1_NAME.to_string(),ECCCurve::new(SECT233k1_NAME,&japt));

	v8 = Vec::from_hex("020000000000000000000000000000000000000004000000000000000001").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("000000000000000000000000000000000000000000000000000000000001").unwrap();
	a = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("0066647EDE6C332C7F8C0923BB58213B333B20E9CE4281FE115F7D8F90AD").unwrap();
	b = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("00FAC9DFCBAC8313BB2139F1BB755FEF65BC391F8B36F8F8EB7371FD558B").unwrap();
	gx = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("01006A08A41903350678E58528BEBF8A0BEFF867A7CA36716F7E01F81052").unwrap();
	gy = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("01000000000000000000000000000013E974E72F8A6922031D2603CFE0D7").unwrap();
	r = BigInt::from_bytes_be(Sign::Plus,&v8);
	//v8 = Vec::from_hex("4").unwrap();
	//h = BigInt::from_bytes_be(Sign::Plus,&v8);
	h = ov.clone() + ov.clone();

	curve = CurveFp::new(&p,&a,&b,&h);
	japt = PointJacobi::new(&curve,&gx,&gy,&ov,Some(r.clone()),true);

	retv.insert(SECT233r1_NAME.to_string(),ECCCurve::new(SECT233r1_NAME,&japt));

	curve = CurveFp::new(&p,&a,&b,&h);
	japt = PointJacobi::new(&curve,&gx,&gy,&ov,Some(r.clone()),true);

	retv.insert(SECT233k1_NAME.to_string(),ECCCurve::new(SECT233k1_NAME,&japt));

	v8 = Vec::from_hex("FFFFFFFDFFFFFFFFFFFFFFFFFFFFFFFF").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("FFFFFFFDFFFFFFFFFFFFFFFFFFFFFFFC").unwrap();
	a = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("E87579C11079F43DD824993C2CEE5ED3").unwrap();
	b = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("161FF7528B899B2D0C28607CA52C5B86").unwrap();
	gx = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("CF5AC8395BAFEB13C02DA292DDED7A83").unwrap();
	gy = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("FFFFFFFE0000000075A30D1B9038A115").unwrap();
	r = BigInt::from_bytes_be(Sign::Plus,&v8);
	//v8 = Vec::from_hex("4").unwrap();
	//h = BigInt::from_bytes_be(Sign::Plus,&v8);
	h = ov.clone();

	curve = CurveFp::new(&p,&a,&b,&h);
	japt = PointJacobi::new(&curve,&gx,&gy,&ov,Some(r.clone()),true);

	retv.insert(SECP128r1_NAME.to_string(),ECCCurve::new(SECP128r1_NAME,&japt));

	v8 = Vec::from_hex("FFFFFFFDFFFFFFFFFFFFFFFFFFFFFFFF").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("D6031998D1B3BBFEBF59CC9BBFF9AEE1").unwrap();
	a = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("5EEEFCA380D02919DC2C6558BB6D8A5D").unwrap();
	b = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("7B6AA5D85E572983E6FB32A7CDEBC140").unwrap();
	gx = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("27B6916A894D3AEE7106FE805FC34B44").unwrap();
	gy = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("3FFFFFFF7FFFFFFFBE0024720613B5A3").unwrap();
	r = BigInt::from_bytes_be(Sign::Plus,&v8);
	//v8 = Vec::from_hex("4").unwrap();
	//h = BigInt::from_bytes_be(Sign::Plus,&v8);
	h = ov.clone() + ov.clone() + ov.clone() + ov.clone();

	curve = CurveFp::new(&p,&a,&b,&h);
	japt = PointJacobi::new(&curve,&gx,&gy,&ov,Some(r.clone()),true);

	retv.insert(SECP128r2_NAME.to_string(),ECCCurve::new(SECP128r2_NAME,&japt));

	v8 = Vec::from_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFAC73").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFAC70").unwrap();
	a = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("B4E134D3FB59EB8BAB57274904664D5AF50388BA").unwrap();
	b = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("52DCB034293A117E1F4FF11B30F7199D3144CE6D").unwrap();
	gx = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("FEAFFEF2E331F296E071FA0DF9982CFEA7D43F2E").unwrap();
	gy = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("0100000000000000000000351EE786A818F3A1A16B").unwrap();
	r = BigInt::from_bytes_be(Sign::Plus,&v8);
	//v8 = Vec::from_hex("4").unwrap();
	//h = BigInt::from_bytes_be(Sign::Plus,&v8);
	h = ov.clone();

	curve = CurveFp::new(&p,&a,&b,&h);
	japt = PointJacobi::new(&curve,&gx,&gy,&ov,Some(r.clone()),true);

	retv.insert(SECP160r2_NAME.to_string(),ECCCurve::new(SECP160r2_NAME,&japt));

	v8 = Vec::from_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFEE37").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("000000000000000000000000000000000000000000000000").unwrap();
	a = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("000000000000000000000000000000000000000000000003").unwrap();
	b = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("DB4FF10EC057E9AE26B07D0280B7F4341DA5D1B1EAE06C7D").unwrap();
	gx = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("9B2F2F6D9C5628A7844163D015BE86344082AA88D95E2F9D").unwrap();
	gy = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("FFFFFFFFFFFFFFFFFFFFFFFE26F2FC170F69466A74DEFD8D").unwrap();
	r = BigInt::from_bytes_be(Sign::Plus,&v8);
	//v8 = Vec::from_hex("4").unwrap();
	//h = BigInt::from_bytes_be(Sign::Plus,&v8);
	h = ov.clone();

	curve = CurveFp::new(&p,&a,&b,&h);
	japt = PointJacobi::new(&curve,&gx,&gy,&ov,Some(r.clone()),true);

	retv.insert(SECP192k1_NAME.to_string(),ECCCurve::new(SECP192k1_NAME,&japt));

	v8 = Vec::from_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFE56D").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("00000000000000000000000000000000000000000000000000000000").unwrap();
	a = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("00000000000000000000000000000000000000000000000000000005").unwrap();
	b = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("A1455B334DF099DF30FC28A169A467E9E47075A90F7E650EB6B7A45C").unwrap();
	gx = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("7E089FED7FBA344282CAFBD6F7E319F7C0B0BD59E2CA4BDB556D61A5").unwrap();
	gy = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("010000000000000000000000000001DCE8D2EC6184CAF0A971769FB1F7").unwrap();
	r = BigInt::from_bytes_be(Sign::Plus,&v8);
	//v8 = Vec::from_hex("4").unwrap();
	//h = BigInt::from_bytes_be(Sign::Plus,&v8);
	h = ov.clone();

	curve = CurveFp::new(&p,&a,&b,&h);
	japt = PointJacobi::new(&curve,&gx,&gy,&ov,Some(r.clone()),true);

	retv.insert(SECP224k1_NAME.to_string(),ECCCurve::new(SECP224k1_NAME,&japt));

	v8 = Vec::from_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF000000000000000000000001").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFFFFFFFFFFFFFFFFFFFE").unwrap();
	a = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("B4050A850C04B3ABF54132565044B0B7D7BFD8BA270B39432355FFB4").unwrap();
	b = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("B70E0CBD6BB4BF7F321390B94A03C1D356C21122343280D6115C1D21").unwrap();
	gx = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("BD376388B5F723FB4C22DFE6CD4375A05A07476444D5819985007E34").unwrap();
	gy = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFF16A2E0B8F03E13DD29455C5C2A3D").unwrap();
	r = BigInt::from_bytes_be(Sign::Plus,&v8);
	//v8 = Vec::from_hex("4").unwrap();
	//h = BigInt::from_bytes_be(Sign::Plus,&v8);
	h = ov.clone();

	curve = CurveFp::new(&p,&a,&b,&h);
	japt = PointJacobi::new(&curve,&gx,&gy,&ov,Some(r.clone()),true);

	retv.insert(SECP224r1_NAME.to_string(),ECCCurve::new(SECP224r1_NAME,&japt));

	v8 = Vec::from_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFFFF0000000000000000FFFFFFFF").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFFFF0000000000000000FFFFFFFC").unwrap();
	a = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("B3312FA7E23EE7E4988E056BE3F82D19181D9C6EFE8141120314088F5013875AC656398D8A2ED19D2A85C8EDD3EC2AEF").unwrap();
	b = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("AA87CA22BE8B05378EB1C71EF320AD746E1D3B628BA79B9859F741E082542A385502F25DBF55296C3A545E3872760AB7").unwrap();
	gx = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("3617DE4A96262C6F5D9E98BF9292DC29F8F41DBD289A147CE9DA3113B5F0B8C00A60B1CE1D7E819D7A431D7C90EA0E5F").unwrap();
	gy = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFC7634D81F4372DDF581A0DB248B0A77AECEC196ACCC52973").unwrap();
	r = BigInt::from_bytes_be(Sign::Plus,&v8);
	//v8 = Vec::from_hex("4").unwrap();
	//h = BigInt::from_bytes_be(Sign::Plus,&v8);
	h = ov.clone();

	curve = CurveFp::new(&p,&a,&b,&h);
	japt = PointJacobi::new(&curve,&gx,&gy,&ov,Some(r.clone()),true);

	retv.insert(SECP384r1_NAME.to_string(),ECCCurve::new(SECP384r1_NAME,&japt));


	v8 = Vec::from_hex("01FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("01FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFC").unwrap();
	a = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("0051953EB9618E1C9A1F929A21A0B68540EEA2DA725B99B315F3B8B489918EF109E156193951EC7E937B1652C0BD3BB1BF073573DF883D2C34F1EF451FD46B503F00").unwrap();
	b = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("00C6858E06B70404E9CD9E3ECB662395B4429C648139053FB521F828AF606B4D3DBAA14B5E77EFE75928FE1DC127A2FFA8DE3348B3C1856A429BF97E7E31C2E5BD66").unwrap();
	gx = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("011839296A789A3BC0045C8A5FB42C7D1BD998F54449579B446817AFBD17273E662C97EE72995EF42640C550B9013FAD0761353C7086A272C24088BE94769FD16650").unwrap();
	gy = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("01FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFA51868783BF2F966B7FCC0148F709A5D03BB5C9B8899C47AEBB6FB71E91386409").unwrap();
	r = BigInt::from_bytes_be(Sign::Plus,&v8);
	//v8 = Vec::from_hex("4").unwrap();
	//h = BigInt::from_bytes_be(Sign::Plus,&v8);
	h = ov.clone();

	curve = CurveFp::new(&p,&a,&b,&h);
	japt = PointJacobi::new(&curve,&gx,&gy,&ov,Some(r.clone()),true);

	retv.insert(SECP521r1_NAME.to_string(),ECCCurve::new(SECP521r1_NAME,&japt));

	v8 = Vec::from_hex("02000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000001").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap();
	a = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001").unwrap();
	b = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("0060F05F658F49C1AD3AB1890F7184210EFD0987E307C84C27ACCFB8F9F67CC2C460189EB5AAAA62EE222EB1B35540CFE9023746").unwrap();
	gx = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("01E369050B7C4E42ACBA1DACBF04299C3460782F918EA427E6325165E9EA10E3DA5F6C42E9C55215AA9CA27A5863EC48D8E0286B").unwrap();
	gy = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("007FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFE5F83B2D4EA20400EC4557D5ED3E3E7CA5B4B5C83B8E01E5FCF").unwrap();
	r = BigInt::from_bytes_be(Sign::Plus,&v8);
	//v8 = Vec::from_hex("4").unwrap();
	//h = BigInt::from_bytes_be(Sign::Plus,&v8);
	h = ov.clone() + ov.clone() + ov.clone() + ov.clone();

	curve = CurveFp::new(&p,&a,&b,&h);
	japt = PointJacobi::new(&curve,&gx,&gy,&ov,Some(r.clone()),true);

	retv.insert(SECT409k1_NAME.to_string(),ECCCurve::new(SECT409k1_NAME,&japt));

	v8 = Vec::from_hex("02000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000001").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001").unwrap();
	a = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("0021A5C2C8EE9FEB5C4B9A753B7B476B7FD6422EF1F3DD674761FA99D6AC27C8A9A197B272822F6CD57A55AA4F50AE317B13545F").unwrap();
	b = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("015D4860D088DDB3496B0C6064756260441CDE4AF1771D4DB01FFE5B34E59703DC255A868A1180515603AEAB60794E54BB7996A7").unwrap();
	gx = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("0061B1CFAB6BE5F32BBFA78324ED106A7636B9C5A7BD198D0158AA4F5488D08F38514F1FDF4B4F40D2181B3681C364BA0273C706").unwrap();
	gy = BigInt::from_bytes_be(Sign::Plus,&v8);
	v8 = Vec::from_hex("010000000000000000000000000000000000000000000000000001E2AAD6A612F33307BE5FA47C3C9E052F838164CD37D9A21173").unwrap();
	r = BigInt::from_bytes_be(Sign::Plus,&v8);
	//v8 = Vec::from_hex("4").unwrap();
	//h = BigInt::from_bytes_be(Sign::Plus,&v8);
	h = ov.clone() + ov.clone() ;

	curve = CurveFp::new(&p,&a,&b,&h);
	japt = PointJacobi::new(&curve,&gx,&gy,&ov,Some(r.clone()),true);

	retv.insert(SECT409r1_NAME.to_string(),ECCCurve::new(SECT409r1_NAME,&japt));

	retv
}

fn create_curve_oid() -> HashMap<String,String> {
	let mut retv :HashMap<String,String> = HashMap::new();	
	retv.insert(SECT163k1_NAME.to_string(),SECT163k1_OID.to_string());
	retv.insert(SECT163r1_NAME.to_string(),SECT163r1_OID.to_string());
	retv.insert(SECT239k1_NAME.to_string(),SECT239k1_OID.to_string());
	retv.insert(SECT113r1_NAME.to_string(),SECT113r1_OID.to_string());
	retv.insert(SECT113r2_NAME.to_string(),SECT113r2_OID.to_string());
	retv.insert(SECP112r1_NAME.to_string(),SECP112r1_OID.to_string());
	retv.insert(SECP112r2_NAME.to_string(),SECP112r2_OID.to_string());
	retv.insert(SECP160r1_NAME.to_string(),SECP160r1_OID.to_string());
	retv.insert(SECP160k1_NAME.to_string(),SECP160k1_OID.to_string());
	retv.insert(SECP256k1_NAME.to_string(),SECP256k1_OID.to_string());
	retv.insert(SECT163r2_NAME.to_string(),SECT163r2_OID.to_string());
	retv.insert(SECT283k1_NAME.to_string(),SECT283k1_OID.to_string());
	retv.insert(SECT283r1_NAME.to_string(),SECT283r1_OID.to_string());
	retv.insert(SECT131r1_NAME.to_string(),SECT131r1_OID.to_string());
	retv.insert(SECT131r2_NAME.to_string(),SECT131r2_OID.to_string());
	retv.insert(SECT193r1_NAME.to_string(),SECT193r1_OID.to_string());
	retv.insert(SECT193r2_NAME.to_string(),SECT193r2_OID.to_string());
	retv.insert(SECT233k1_NAME.to_string(),SECT233k1_OID.to_string());
	retv.insert(SECT233r1_NAME.to_string(),SECT233r1_OID.to_string());
	retv.insert(SECP128r1_NAME.to_string(),SECP128r1_OID.to_string());
	retv.insert(SECP128r2_NAME.to_string(),SECP128r2_OID.to_string());
	retv.insert(SECP160r2_NAME.to_string(),SECP160r2_OID.to_string());
	retv.insert(SECP192k1_NAME.to_string(),SECP192k1_OID.to_string());
	retv.insert(SECP224k1_NAME.to_string(),SECP224k1_OID.to_string());
	retv.insert(SECP224r1_NAME.to_string(),SECP224r1_OID.to_string());
	retv.insert(SECP384r1_NAME.to_string(),SECP384r1_OID.to_string());
	retv.insert(SECP521r1_NAME.to_string(),SECP521r1_OID.to_string());
	retv.insert(SECT409k1_NAME.to_string(),SECT409k1_OID.to_string());
	retv.insert(SECT409r1_NAME.to_string(),SECT409r1_OID.to_string());

	retv
}

fn create_curve_name() -> HashMap<String,String> {
	let mut retv :HashMap<String,String> = HashMap::new();
	retv.insert(SECT163k1_OID.to_string(),SECT163k1_NAME.to_string());
	retv.insert(SECT163r1_OID.to_string(),SECT163r1_NAME.to_string());
	retv.insert(SECT239k1_OID.to_string(),SECT239k1_NAME.to_string());
	retv.insert(SECT113r1_OID.to_string(),SECT113r1_NAME.to_string());
	retv.insert(SECT113r2_OID.to_string(),SECT113r2_NAME.to_string());
	retv.insert(SECP112r1_OID.to_string(),SECP112r1_NAME.to_string());
	retv.insert(SECP112r2_OID.to_string(),SECP112r2_NAME.to_string());
	retv.insert(SECP160r1_OID.to_string(),SECP160r1_NAME.to_string());
	retv.insert(SECP160k1_OID.to_string(),SECP160k1_NAME.to_string());
	retv.insert(SECP256k1_OID.to_string(),SECP256k1_NAME.to_string());
	retv.insert(SECT163r2_OID.to_string(),SECT163r2_NAME.to_string());
	retv.insert(SECT283k1_OID.to_string(),SECT283k1_NAME.to_string());
	retv.insert(SECT283r1_OID.to_string(),SECT283r1_NAME.to_string());
	retv.insert(SECT131r1_OID.to_string(),SECT131r1_NAME.to_string());
	retv.insert(SECT131r2_OID.to_string(),SECT131r2_NAME.to_string());
	retv.insert(SECT193r1_OID.to_string(),SECT193r1_NAME.to_string());
	retv.insert(SECT193r2_OID.to_string(),SECT193r2_NAME.to_string());
	retv.insert(SECT233k1_OID.to_string(),SECT233k1_NAME.to_string());
	retv.insert(SECT233r1_OID.to_string(),SECT233r1_NAME.to_string());
	retv.insert(SECP128r1_OID.to_string(),SECP128r1_NAME.to_string());
	retv.insert(SECP128r2_OID.to_string(),SECP128r2_NAME.to_string());
	retv.insert(SECP160r2_OID.to_string(),SECP160r2_NAME.to_string());
	retv.insert(SECP192k1_OID.to_string(),SECP192k1_NAME.to_string());
	retv.insert(SECP224k1_OID.to_string(),SECP224k1_NAME.to_string());
	retv.insert(SECP224r1_OID.to_string(),SECP224r1_NAME.to_string());
	retv.insert(SECP384r1_OID.to_string(),SECP384r1_NAME.to_string());
	retv.insert(SECP521r1_OID.to_string(),SECP521r1_NAME.to_string());
	retv.insert(SECT409k1_OID.to_string(),SECT409k1_NAME.to_string());
	retv.insert(SECT409r1_OID.to_string(),SECT409r1_NAME.to_string());

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

