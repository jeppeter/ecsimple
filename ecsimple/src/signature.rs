use num_bigint::{BigInt,Sign};
use std::error::Error;
//use rand::RngCore;

ecsimple_error_class!{EccSignatureError}


#[derive(Clone)]
pub struct ECCSignature {
	pub r :BigInt,
	pub s :BigInt,
}

impl ECCSignature {
	pub fn new(r :&BigInt, s :&BigInt) -> Self {
		ECCSignature {
			r : r.clone(),
			s : s.clone(),
		}
	}
}