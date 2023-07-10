
use crate::bngf2m::*;
use crate::group::*;
use num_bigint::{BigInt};
use num_traits::{zero,one};

use crate::logger::*;

fn get_max_bits(bn :&BigInt) -> i64 {
	let mut retv : i64 = -1;
	let mut idx : i64 = 0 ;
	let zv :BigInt = zero();
	let mut cv : BigInt = one();

	while bn >= &cv {
		if (&cv & bn) != zv {
			/*for expand*/
			retv = (idx + 1);
		}
		idx += 1;
		cv <<= 1;
	}
	return retv;
}

#[derive(Clone)]
pub struct ECGf2mPoint {
	x :BnGf2m,
	y :BnGf2m,
	z :BnGf2m,
	group :ECGroupBnGf2m,
	infinity : bool,
}

impl std::fmt::Display for ECGf2mPoint {
	fn fmt(&self, f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f,"curve[{}] isinfinity {} x 0x{:x} y 0x{:x} z 0x{:x}", self.group,self.infinity,self.x,self.y,self.z)
	}
}

impl std::default::Default for ECGf2mPoint {
	fn default() -> Self {
		ECGf2mPoint {
			x : BnGf2m::default(),
			y :BnGf2m::default(),
			z :BnGf2m::default(),
			group : ECGroupBnGf2m::default(),
			infinity : true,
		}
	}
}


impl ECGf2mPoint {
	pub fn is_infinity(&self) -> bool {
		return self.infinity;
	}

	pub fn new(grp :&ECGroupBnGf2m) -> ECGf2mPoint {
		ECGf2mPoint {
			x : grp.generator.x.clone(),
			y : grp.generator.y.clone(),
			z : grp.generator.z.clone(),
			group : grp.clone(),
			infinity : false,
		}
	}


	pub fn new_point(x :&BnGf2m, y :&BnGf2m,z :&BnGf2m, grp :&ECGroupBnGf2m) -> Self {
		Self {
			x :x.clone(),
			y :y.clone(),
			z :z.clone(),
			group :grp.clone(),
			infinity : false,
		}
	}


	pub fn set_group(&mut self, grp :&ECGroupBnGf2m) {
		self.group = grp.clone();
	}

	pub fn set_x(&mut self, x :&BnGf2m) {
		self.x = x.clone();
	}

	pub fn set_y(&mut self, y :&BnGf2m) {
		self.y = y.clone();
	}

	pub fn set_z(&mut self, z :&BnGf2m) {
		self.z = z.clone();
	}

	pub fn mul_op(&self, bn :&BigInt) -> ECGf2mPoint {
		let zv :BigInt = zero();
		let mut retv :ECGf2mPoint;
		let mut p :ECGf2mPoint = ECGf2mPoint::new(&self.group);
		let mut s :ECGf2mPoint = ECGf2mPoint::new(&self.group);
		let mut cardinal :BigInt = zero();
		let mut lamda :BigInt = zero();
		let mut k :BigInt = zero();
		if bn <= &zv {
			let mut retv :ECGf2mPoint = self.clone();
			retv.infinity = true;
			return retv;
		}

		if self.infinity {
			return self.clone();
		}

		if self.group.order == zv || self.group.cofactor == zv {
			panic!("group order 0x{:x} or group cofactor 0x{:x}", self.group.order, self.group.cofactor);
		}

		cardinal = &self.group.order * &self.group.cofactor;

		ecsimple_log_trace!("field 0x{:X} p 0x{:X}", self.group.p,self.group.p);
		ecsimple_log_trace!("group->a 0x{:X} a 0x{:X}", self.group.a,self.group.a);
		ecsimple_log_trace!("group->b 0x{:X} b 0x{:X}", self.group.b,self.group.b);
		ecsimple_log_trace!("cardinality 0x{:X} order 0x{:X} cofactor 0x{:X}",cardinal,self.group.order,self.group.cofactor);
		ecsimple_log_trace!("k 0x{:X} lamda 0x{:X}", k, lamda);

		k = bn.clone();
		lamda = &k + &cardinal;
		ecsimple_log_trace!("scalar 0x{:X} k 0x{:X}",k,k);
		ecsimple_log_trace!("lamda 0x{:X}",lamda);

		k = &lamda + &cardinal;

		let cardbits = get_max_bits(&cardinal);
		ecsimple_log_trace!("k 0x{:X} cardinality 0x{:X} cardinality_bits 0x{:x}",k,cardinal,cardbits);

		

		return self.clone();
	}

}