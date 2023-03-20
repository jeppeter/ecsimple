extern crate num_bigint_dig as num_bigint2;

use crate::*;
use num_bigint2::{BigInt,Sign};
use crate::arithmetics::*;
use crate::utils::*;
use crate::jacobi::{PointJacobi,ECCPoint};
use crate::curves::*;
use crate::signature::*;
use std::error::Error;
use num_traits::{zero,one};
//use rand::RngCore;

ecsimple_error_class!{EccKeyError}


#[derive(Clone)]
pub struct PublicKey {
	curve :ECCCurve,
	pubkey :PointJacobi,
}

#[allow(non_snake_case)]
impl PublicKey {
	pub fn new(curve :&ECCCurve, pt :&ECCPoint) -> Result<Self,Box<dyn Error>> {
		Ok(PublicKey {
			curve :curve.clone(),
			pubkey : PointJacobi::from_affine(pt,false),
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
		s = inverse_mod(&k,&n) * (((&hash) + &(self.keynum) * &r) % (&n) ) ;
		if s == zero() {
			ecsimple_new_error!{EccKeyError,"randkey [{}] s zeroized", randkey}
		}
		Ok (ECCSignature::new(&r,&s))
	}

	pub fn get_public_key(&self) -> PublicKey {
		PublicKey {
			curve : self.curve.clone(),
			pubkey : self.pubkey.clone(),
		}
	}
}