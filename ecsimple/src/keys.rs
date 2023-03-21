
use crate::*;
use num_bigint::{BigInt,Sign};
use crate::arithmetics::*;
use crate::utils::*;
use crate::jacobi::{PointJacobi,ECCPoint};
use crate::curves::*;
use crate::signature::*;
use std::error::Error;
use num_traits::{zero,one};
use crate::logger::*;

//use rand::RngCore;

#[allow(unused_imports)]
use asn1obj_codegen::{asn1_choice,asn1_obj_selector,asn1_sequence,asn1_int_choice};
use asn1obj::complex::*;
use asn1obj::strop::*;
use asn1obj::base::*;
use asn1obj::*;
use asn1obj::asn1impl::*;
use std::io::Write;


ecsimple_error_class!{EccKeyError}

#[derive(Clone)]
#[asn1_sequence()]
pub struct ECPublicKeyObjElem {
	pub types :Asn1Object,
	pub ectypes :Asn1Object,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct ECPublicKeyObj {
	pub elem :Asn1Seq<ECPublicKeyObjElem>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct ECPublicKeyAbbrevElem {
	pub types :ECPublicKeyObj,
	pub coords :Asn1BitData,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct ECPublicKeyAbbrev {
	pub elem :Asn1Seq<ECPublicKeyAbbrevElem>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct ECPublicKeyFieldIDElem {
	pub types :Asn1Object,
	pub primenum :Asn1BigNum,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct ECPublicKeyFieldID {
	pub elem :Asn1Seq<ECPublicKeyFieldIDElem>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct ECPublicKeyCurveElem {
	pub a :Asn1BigNum,
	pub b :Asn1BigNum,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct ECPublicKeyCurve {
	pub elem :Asn1Seq<ECPublicKeyCurveElem>,
}


#[derive(Clone)]
#[asn1_sequence()]
pub struct ECPublicKeyParamsElem {
	pub version :Asn1Integer,
	pub fieldid :ECPublicKeyFieldID,
	pub curve :ECPublicKeyCurve,
	pub basecoords :Asn1OctData,
	pub order :Asn1BigNum,
	pub cofactor :Asn1BigNum,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct ECPublicKeyParams {
	pub elem :Asn1Seq<ECPublicKeyParamsElem>,
}


#[derive(Clone)]
#[asn1_sequence()]
pub struct ECPublicKeyTotalElem {
	pub types :Asn1Object,
	pub ecparams :ECPublicKeyParams,
	pub coords :Asn1BitData,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct ECPublicKeyTotal {
	pub elem :Asn1Seq<ECPublicKeyTotalElem>,
}

#[derive(Clone)]
#[asn1_int_choice(selector=typei,abbrev=1,total=2)]
pub struct ECPublicKeyChoiceElem {
	pub typei :i32,
	pub abbrev :ECPublicKeyAbbrev,
	pub total :ECPublicKeyTotal,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct ECPublicKeyChoice {
	pub elem :Asn1Seq<ECPublicKeyChoiceElem>,
}

#[derive(Clone,Debug)]
pub struct PublicKey {
	pub curve :ECCCurve,
	pub pubkey :PointJacobi,
}

#[allow(non_snake_case)]
impl PublicKey {
	pub fn new(curve :&ECCCurve, pt :&ECCPoint) -> Result<Self,Box<dyn Error>> {
		Ok(PublicKey {
			curve :curve.clone(),
			pubkey : PointJacobi::from_affine(pt,false),
		})
	}

	fn _to_der_compressed(&self) -> Result<Vec<u8>,Box<dyn Error>> {
		Ok(Vec::new())
	}

	fn _to_der_uncompressed(&self) -> Result<Vec<u8>,Box<dyn Error>> {
		Ok(Vec::new())
	}

	fn _to_der_hybrid(&self) -> Result<Vec<u8>,Box<dyn Error>> {
		Ok(Vec::new())
	}

	pub fn to_der(&self,types :&str) -> Result<Vec<u8>,Box<dyn Error>> {
		if types == "compressed" {
			return self._to_der_compressed();
		} else if types == "uncompressed" {
			return self._to_der_uncompressed();
		} else if types == "hybrid" {
			return self._to_der_hybrid();
		} 
		ecsimple_new_error!{EccKeyError,"not valid types [{}]",types}		
	}

	pub fn from_der(v8 :&[u8]) -> Result<Self,Box<dyn Error>> {
		let mut curveb :ECCCurve = get_ecc_by_name("SECP112r1")?;
		let ov :BigInt = one();
		Ok(PublicKey {
			curve : curveb.clone(),
			pubkey : curveb.generator.mul_int(&ov),
		})
	}

	pub fn verify(&self,hashcode :&[u8],sig :&ECCSignature) -> bool {
		let mut G :PointJacobi = self.curve.generator.clone();
		let n :BigInt = G.order();
		let r :BigInt = sig.r.clone();
		let s :BigInt = sig.s.clone();
		let hash :BigInt = BigInt::from_bytes_be(Sign::Plus,hashcode);
		let mut pubkey :PointJacobi = self.pubkey.clone();

		if r < one() || r >= n {
			return false;
		}

		if s < one() || s >= n {
			return false;
		}
		let c :BigInt = inverse_mod(&s,&n);
		let u1 :BigInt = ((&hash) * (&c)) % (&n);
		let u2 :BigInt = ((&r) * (&c)) % (&n);
		let u1g :PointJacobi = G.mul_int(&u1);
		let u2g :PointJacobi = pubkey.mul_int(&u2);
		let xy :PointJacobi = u1g.add_jacobi(&u2g);
		let v :BigInt = xy.x() % (&n);
		if v == r {
			return true;
		}
		return false;
	}
}


#[derive(Clone)]
pub struct PrivateKey {
	curve :ECCCurve,
	keynum :BigInt,
	pubkey :PointJacobi,
}

#[allow(non_snake_case)]
impl PrivateKey {
	pub fn generate(curve :&ECCCurve,fname :Option<String>) -> Result<Self,Box<dyn Error>> {
		let mut bname :Option<String> = None;
		if fname.is_some() {
			bname = Some(format!("{}",fname.as_ref().unwrap()));
		}
		let mut rdops = RandOps::new(bname)?;
		let bitlen :usize = bit_length(&curve.order);
		let bs :usize = (bitlen + 7) / 8;
		let vecs = rdops.get_bytes(bs)?;
		let knum :BigInt = BigInt::from_bytes_be(Sign::Plus,&vecs);
		let mut bptr :PointJacobi = curve.generator.clone();
		let pubkey :PointJacobi = bptr.mul_int(&knum);
		Ok(PrivateKey {
			curve : curve.clone(),
			keynum : knum.clone(),
			pubkey : pubkey,
		})
	}

	pub fn new(curve :&ECCCurve, secnum :&BigInt) -> Result<Self,Box<dyn Error >> {
		let bitlen :usize = bit_length(&curve.order);
		let (_ ,vecs) = secnum.to_bytes_be();
		let mut vlen :usize = 0;
		if vecs.len() > 0 {
			vlen = (vecs.len() - 1) * 8;
			let mut uv :u8 = vecs[0];
			while uv > 0 {
				vlen += 1;
				uv >>= 1;
			}
		}

		if vlen > bitlen {
			ecsimple_new_error!{EccKeyError,"secnum [{}] < order [{}]", vlen,bitlen}
		}
		let mut gen :PointJacobi = curve.generator.clone();
		let pubkey :PointJacobi = gen.mul_int(&secnum);
		Ok (PrivateKey {
			curve : curve.clone(),
			keynum : secnum.clone(),
			pubkey : pubkey,
		})
	}

	pub fn sign(&self, hashcode :&[u8], randkey :&BigInt) -> Result<ECCSignature,Box<dyn Error>> {
		let n :BigInt;
		let mut G :PointJacobi = self.curve.generator.clone();
		n = G.order();
		let k :BigInt = randkey % (&n);
		let ks :BigInt = &k + &n;
		let kt :BigInt = &ks + &n;
		let p1 :PointJacobi;
		let r :BigInt;
		let s :BigInt;
		let hash :BigInt = BigInt::from_bytes_be(Sign::Plus,hashcode);

		if bit_length(&ks) == bit_length(&n) {
			p1 = G.mul_int(&kt);
		} else {
			p1 = G.mul_int(&ks);
		}

		r = p1.x() % (&n);
		if r == zero() {
			ecsimple_new_error!{EccKeyError,"randkey [{}] r zeroized", randkey}
		}
		s = (inverse_mod(&k,&n) * (((&hash) + &(self.keynum) * &r) % (&n) ) ) % (&n);
		if s == zero() {
			ecsimple_new_error!{EccKeyError,"randkey [{}] s zeroized", randkey}
		}
		ecsimple_log_trace!("r 0x{:x} s 0x{:x}",r, s);
		Ok (ECCSignature::new(&r,&s))
	}

	pub fn get_public_key(&self) -> PublicKey {
		PublicKey {
			curve : self.curve.clone(),
			pubkey : self.pubkey.clone(),
		}
	}
}