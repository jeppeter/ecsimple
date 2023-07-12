

use crate::point::*;
use crate::group::*;
use crate::bngf2m::*;
use crate::signature::*;
use num_bigint::{BigInt};
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

	pub fn sign_base(&self,hashnum :&BigInt) -> Result<ECSignature,Box<dyn Error>> {
		let zv :BigInt = zero();
		let retv :ECSignature = ECSignature::new(&zv,&zv);
		Ok(retv)
	}
}




