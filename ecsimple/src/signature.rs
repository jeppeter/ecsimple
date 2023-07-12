
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

impl std::cmp::PartialEq for ECSignature {
	fn eq(&self,other :&Self) -> bool {
		let mut retv : bool = true;
		if self.r != other.r {
			retv = false;
		}

		if self.s != other.s {
			retv = false;
		}
		retv
	}

	fn ne(&self,other :&Self) -> bool {
		return ! self.eq(other);
	}
}