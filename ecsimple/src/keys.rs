

use crate::point::*;
use crate::group::*;
use crate::bngf2m::*;
use crate::signature::*;
use crate::utils::*;
use crate::randop::*;
use crate::logger::*;
use num_bigint::{BigInt,Sign};
use num_traits::{zero};

use std::error::Error;


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

	pub fn verify_base(&self,sig :&ECSignature, hashnum :&BigInt) -> Result<bool,Box<dyn Error>> {
		Ok(true)
	}
}

#[derive(Clone)]
pub struct ECGf2mPrivateKey {
	base : ECGf2mPoint,
	privnum :BigInt,
	pubk :ECGf2mPoint,
}

impl ECGf2mPrivateKey {
	pub fn new(grp :&ECGroupBnGf2m , privnum :&BigInt) -> ECGf2mPrivateKey {
		let b :ECGf2mPoint = ECGf2mPoint::new(grp);
		let pubk :ECGf2mPoint = b.mul_op(privnum);
		ECGf2mPrivateKey {
			base : b,
			privnum : privnum.clone(),
			pubk : pubk,
		}
	}

	pub fn export_pubkey(&self) -> ECGf2mPubKey {
		let retv :ECGf2mPubKey = ECGf2mPubKey {
			base : self.base.clone(),
			pubk : self.pubk.clone(),
		};
		retv
	}

	fn setup_sign(&self,realhash :&BigInt, hashlen :i64) -> Result<(BigInt,BigInt),Box<dyn Error>> {
		let r :BigInt = zero();
		let kinv :BigInt = zero();
		let k = ecsimple_rand_range(hashlen,&self.base.group.order);
		ecsimple_log_trace!("k 0x{:X} dlen 0x{:x}", k, hashlen);
		Ok((kinv,r))
	}

	pub fn sign_base(&self,hashnum :&[u8]) -> Result<ECSignature,Box<dyn Error>> {
		let zv :BigInt = zero();
		let mut bs = hashnum.to_vec();
		let orderbits = get_max_bits(&self.base.group.order);
		if (bs.len() * 8 ) > (orderbits as usize) {
			bs = bs[0..(((orderbits as usize) +7) >> 3)].to_vec();
		}
		let mut realhash :BigInt = BigInt::from_bytes_be(Sign::Plus,&bs);

		if (bs.len() * 8) > (orderbits as usize) {
			realhash = realhash >> (8 - orderbits & 0x7);
		}

		(_, bs) = realhash.to_bytes_be();

		assert!(realhash <= self.base.group.order);
		let (kinv,r) = self.setup_sign(&realhash,hashnum.len() as i64)?;


		let retv :ECSignature = ECSignature::new(&zv,&zv);
		Ok(retv)
	}
}




