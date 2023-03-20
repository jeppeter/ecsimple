extern crate num_bigint_dig as bigint2;

use num_bigint::{BigInt,Sign};
use num_bigint::BigUint as BaseBigUint;

#[allow(unused_imports)]
use asn1obj_codegen::{asn1_choice,asn1_obj_selector,asn1_sequence,asn1_int_choice};
use asn1obj::complex::*;
use asn1obj::strop::*;
use asn1obj::base::*;
use asn1obj::*;
use asn1obj::asn1impl::*;
use std::error::Error;
use std::io::Write;
//use rand::RngCore;

ecsimple_error_class!{EccSignatureError}


#[derive(Clone)]
pub struct ECCSignature {
	pub r :BigInt,
	pub s :BigInt,
}

#[asn1_sequence()]
#[derive(Clone)]
struct Asn1ECCSignatureElem {
	pub r :Asn1BigNum,
	pub s :Asn1BigNum,
}

#[asn1_sequence()]
#[derive(Clone)]
struct Asn1ECCSignature {
	pub elem :Asn1Seq<Asn1ECCSignatureElem>,
}

impl ECCSignature {
	pub fn new(r :&BigInt, s :&BigInt) -> Self {
		ECCSignature {
			r : r.clone(),
			s : s.clone(),
		}
	}

	pub fn to_der(&self) -> Result<Vec<u8>,Box<dyn Error>> {
		let mut sigasn1 :Asn1ECCSignature = Asn1ECCSignature::init_asn1();
		let mut sigelemasn1 :Asn1ECCSignatureElem = Asn1ECCSignatureElem::init_asn1();
		let (_ , vecs) = self.r.to_bytes_be();
		sigelemasn1.r.val = BaseBigUint::from_bytes_be(&vecs);
		let (_ , vecs) = self.s.to_bytes_be();
		sigelemasn1.s.val = BaseBigUint::from_bytes_be(&vecs);
		sigasn1.elem.val.push(sigelemasn1);
		return sigasn1.encode_asn1();
	}

	pub fn from_der(sigcode :&[u8]) -> Result<Self,Box<dyn Error>> {
		let mut sigasn1 :Asn1ECCSignature = Asn1ECCSignature::init_asn1();
		let _ = sigasn1.decode_asn1(sigcode)?;
		if sigasn1.elem.val.len() != 1 {
			ecsimple_new_error!{EccSignatureError,"not valid asn1 code"}
		}
		Ok(ECCSignature {
			r : BigInt::from_bytes_be(Sign::Plus,&(sigasn1.elem.val[0].r.val.to_bytes_be())),
			s : BigInt::from_bytes_be(Sign::Plus,&(sigasn1.elem.val[0].s.val.to_bytes_be())),
		})
	}

}