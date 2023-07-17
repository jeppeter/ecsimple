

use crate::point::*;
use crate::group::*;
use crate::bngf2m::*;
use crate::signature::*;
use crate::utils::*;
use crate::randop::*;
use crate::logger::*;
use num_bigint::{BigInt,Sign};
use num_traits::{zero,one};

use std::error::Error;


#[allow(dead_code)]
#[derive(Clone)]
pub struct ECGf2mPubKey {
	base :ECGf2mPoint,
	pubk :ECGf2mPoint,
}

impl ECGf2mPubKey {
	pub fn new(grp :&ECGroupBnGf2m,x :&BigInt,y :&BigInt) -> ECGf2mPubKey {
		let b = ECGf2mPoint::new(grp);
		let xn :BnGf2m = BnGf2m::new_from_bigint(x);
		let yn :BnGf2m = BnGf2m::new_from_bigint(y);
		let zn :BnGf2m = BnGf2m::one();
		ECGf2mPubKey {
			base : b,
			pubk : ECGf2mPoint::new_point(&xn,&yn,&zn,grp),
		}
	}

	#[allow(unused_variables)]
	pub fn verify_base(&self,sig :&ECSignature, hashnum :&BigInt) -> Result<bool,Box<dyn Error>> {
		Ok(true)
	}
}

#[derive(Clone)]
pub struct ECGf2mPrivateKey {
	base : ECGf2mPoint,
	privnum :BigInt,
}

impl ECGf2mPrivateKey {
	pub fn new(grp :&ECGroupBnGf2m , privnum :&BigInt) -> ECGf2mPrivateKey {
		let b :ECGf2mPoint = ECGf2mPoint::new(grp);
		ECGf2mPrivateKey {
			base : b,
			privnum : privnum.clone(),
		}
	}

	pub fn export_pubkey(&self) -> ECGf2mPubKey {
		let ck : ECGf2mPoint;
		ck = self.base.mul_op(&self.privnum);
		let retv :ECGf2mPubKey = ECGf2mPubKey {
			base : self.base.clone(),
			pubk : ck.clone(),
		};
		retv
	}

	#[allow(unused_variables)]
	#[allow(non_snake_case)]
	fn setup_sign(&self,realhash :&BigInt, hashlen :i64) -> Result<(BigInt,BigInt),Box<dyn Error>> {
		let mut r :BigInt;
		let mut tmppnt :ECGf2mPoint = self.base.clone();
		let zv :BnGf2m = BnGf2m::zero();
		let ov :BigInt = one();
		let order :BnGf2m;
		tmppnt.set_x(&zv);
		tmppnt.set_y(&zv);
		tmppnt.set_z(&zv);
		let mut k  :BigInt ;
		let mut X :BnGf2m;
		let blen = get_max_bits(&self.base.group.order);
		ecsimple_log_trace!("tmp.x 0x{:X} tmp.y 0x{:X}, tmp.z 0x{:X}", tmppnt.x(),tmppnt.y(),tmppnt.z());
		ecsimple_log_trace!("order 0x{:X}",self.base.group.order);
		k = ov << blen;
		order = BnGf2m::new_from_bigint(&self.base.group.order);
		loop {
			ecsimple_log_trace!("k 0x{:X}",k);
			k = ecsimple_rand_range(&self.base.group.order);
			ecsimple_log_trace!("k 0x{:X} order 0x{:X} dlen 0x{:x}", k, self.base.group.order,((blen + 7 ) >> 3) as i64);

			ecsimple_log_trace!("group.x 0x{:X} group.y 0x{:X} group.z 0x{:X}", self.base.group.generator.x,self.base.group.generator.y,self.base.group.generator.z);
			tmppnt = self.base.mul_op(&k);
			ecsimple_log_trace!("tmp.x 0x{:X} tmp.y 0x{:X} tmp.z 0x{:X}", tmppnt.x(),tmppnt.y(),tmppnt.z());

			(X,_) = tmppnt.get_affine_points()?;

			ecsimple_log_trace!("tmp.x 0x{:X} tmp.y 0x{:X} tmp.z 0x{:X}", tmppnt.x(),tmppnt.y(),tmppnt.z());
			ecsimple_log_trace!("X 0x{:X} order 0x{:X}",X,order);


			let xb :BigInt = X.to_bigint();

			r = xb % &self.base.group.order;


			if r != zero() {
				break;
			}
		}

		ecsimple_log_trace!("X 0x{:X} r 0x{:X}", X,r);

		let e :BigInt = &self.base.group.order - 2;
		let kn = k.clone();

		k = k.modpow(&e,&self.base.group.order);
		ecsimple_log_trace!("(x 0x{:X} ^ e 0x{:X}) = (r 0x{:X}) = 1 % order 0x{:X}",kn,e,k,self.base.group.order);
		ecsimple_log_trace!("k 0x{:X} r 0x{:X}",k,r);
		Ok((k,r))
	}

	pub fn sign_base(&self,hashnum :&[u8]) -> Result<ECSignature,Box<dyn Error>> {
		let zv :BigInt = zero();
		let mut bs = hashnum.to_vec();
		let orderbits = get_max_bits(&self.base.group.order);
		ecsimple_log_trace!("begin sign");
		if (bs.len() * 8 ) > (orderbits as usize) {
			bs = bs[0..(((orderbits as usize) +7) >> 3)].to_vec();
		}
		let mut realhash :BigInt = BigInt::from_bytes_be(Sign::Plus,&bs);
		let r :BigInt = zero();
		let s :BigInt = zero();
		ecsimple_log_trace!("r 0x{:X} s 0x{:X}",r,s);
		ecsimple_log_trace!("order 0x{:X}", self.base.group.order);
		ecsimple_log_trace!("dgst 0x{:X}", realhash);


		if (bs.len() * 8) > (orderbits as usize) {
			realhash = realhash >> (8 - orderbits & 0x7);
		}

		ecsimple_log_trace!("dgst rshift 0x{:X}", realhash);

		(_, bs) = realhash.to_bytes_be();


		assert!(realhash <= self.base.group.order);
		let (kinv,r) = self.setup_sign(&realhash,hashnum.len() as i64)?;
		ecsimple_log_trace!("ckinv 0x{:X} r 0x{:X}",kinv,r);


		let retv :ECSignature = ECSignature::new(&zv,&zv);
		Ok(retv)
	}
}




