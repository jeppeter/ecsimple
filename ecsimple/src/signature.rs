
use num_bigint::{BigInt};


pub struct ECSignature {
	r : BigInt,
	s : BigInt,
}

impl ECSignature {
	pub fn new(r :&BigInt, s :&BigInt) -> Self {
		ECSignature {
			r : r.clone(),
			s : s.clone(),
		}
	}
}