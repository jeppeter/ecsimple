
use crate::*;
use num_bigint::{BigInt,Sign};
use crate::arithmetics::*;
use crate::utils::*;
use crate::jacobi::PointJacobi;
use crate::curves::*;
use std::error::Error;
//use rand::RngCore;

ecsimple_error_class!{EccKeyError}


#[derive(Clone)]
pub struct PublicKey {
	pubkey :PointJacobi,
}


#[derive(Clone)]
pub struct PrivateKey {
	keynum :BigInt,
	pubkey :PointJacobi,
}

impl PrivateKey {
	pub fn generate(curve :&ECCCurve,fname :Option<String>) -> Result<Self,Box<dyn Error>> {
		let mut rdops = RandOps::new(fname)?;
		let bitlen :usize = bit_length(&curve.order);
		let bs :usize = (bitlen + 7) / 8;
		let vecs = rdops.get_bytes(bs)?;
		let knum :BigInt = BigInt::from_bytes_be(Sign::Plus,&vecs);
		let mut bptr :PointJacobi = curve.generator.clone();
		let pubkey :PointJacobi = bptr.mul_int(&knum);
		Ok(PrivateKey {
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
				keynum : secnum.clone(),
				pubkey : pubkey,
			})
	}

	pub fn get_public_key(&self) -> PublicKey {
		PublicKey {
			pubkey : self.pubkey.clone(),
		}
	}
}