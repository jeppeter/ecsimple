
use crate::bngf2m::*;
use num_bigint::{BigInt};
use num_traits::{zero};


pub trait ECGroup {
	fn x(&self) -> BigInt ;
	fn y(&self) -> BigInt ;
	fn z(&self) -> BigInt ;
}

#[derive(Clone)]
pub struct ECBnGf2mGenerator {
	pub x :BnGf2m,
	pub y :BnGf2m,
	pub z :BnGf2m,
}

impl std::fmt::Display for ECBnGf2mGenerator {
	fn fmt(&self, f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f,"x 0x{:x} y 0x{:x} z 0x{:x}", self.x,self.y,self.z)
	}
}

impl ECBnGf2mGenerator {
	pub fn new(x :&BnGf2m, y :&BnGf2m,z :&BnGf2m) -> Self {
		ECBnGf2mGenerator {
			x :x.clone(),
			y :y.clone(),
			z :z.clone(),
		}
	}
}

impl std::default::Default for ECBnGf2mGenerator {
	fn default() -> Self {
		ECBnGf2mGenerator {
			x : BnGf2m::default(),
			y :BnGf2m::default(),
			z :BnGf2m::default(),
		}
	}
}


#[derive(Clone)]
pub struct ECGroupBnGf2m {
	pub generator :ECBnGf2mGenerator,
	pub order :BigInt,
	pub cofactor :BigInt,
	pub curvename :String,
	pub a :BnGf2m,
	pub b :BnGf2m,
}

impl std::fmt::Display for ECGroupBnGf2m {
	fn fmt(&self, f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f,"curve {} generator {} order 0x{:x} cofactor 0x{:x} a 0x{:x} b 0x{:x}", 
			self.curvename, self.generator,self.order,self.cofactor,self.a, self.b)
	}
}

impl std::default::Default for ECGroupBnGf2m {
	fn default() -> Self {
		ECGroupBnGf2m {
			generator : ECBnGf2mGenerator::default(),
			order :zero(),
			cofactor :zero(),
			curvename : "".to_string(),
			a : BnGf2m::default(),
			b : BnGf2m::default(),
		}
	}
}

impl ECGroup for ECGroupBnGf2m {
	fn x(&self) -> BigInt {
		return self.generator.x.to_bigint();
	}

	fn y(&self) -> BigInt {
		return self.generator.y.to_bigint();
	}

	fn z(&self) -> BigInt {
		return self.generator.z.to_bigint();
	}
}


#[derive(Clone)]
pub struct ECPrimeGenerator {
	pub x :BigInt,
	pub y :BigInt,
	pub z :BigInt,
}

impl std::fmt::Display for ECPrimeGenerator {
	fn fmt(&self, f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f,"x 0x{:x} y 0x{:x} z 0x{:x}", self.x,self.y,self.z)
	}
}

impl ECPrimeGenerator {
	pub fn new(x :&BigInt, y :&BigInt,z :&BigInt) -> Self {
		ECPrimeGenerator {
			x :x.clone(),
			y :y.clone(),
			z :z.clone(),
		}
	}
}

impl std::default::Default for ECPrimeGenerator {
	fn default() -> Self {
		ECPrimeGenerator {
			x : zero(),
			y : zero(),
			z : zero(),
		}
	}
}


#[derive(Clone)]
pub struct ECGroupPrime {
	pub generator :ECPrimeGenerator,
	pub order :BigInt,
	pub cofactor :BigInt,
	pub curvename :String,
	pub a :BigInt,
	pub b :BigInt,
}

impl std::fmt::Display for ECGroupPrime {
	fn fmt(&self, f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f,"curve {} generator {} order 0x{:x} cofactor 0x{:x} a 0x{:x} b 0x{:x}", 
			self.curvename, self.generator,self.order,self.cofactor,self.a, self.b)
	}
}

impl std::default::Default for ECGroupPrime {
	fn default() -> Self {
		ECGroupPrime {
			generator : ECPrimeGenerator::default(),
			order :zero(),
			cofactor :zero(),
			curvename : "".to_string(),
			a : zero(),
			b : zero(),
		}
	}
}


impl ECGroup for ECGroupPrime {
	fn x(&self) -> BigInt {
		return self.generator.x.clone();
	}

	fn y(&self) -> BigInt {
		return self.generator.y.clone();
	}

	fn z(&self) -> BigInt {
		return self.generator.z.clone();
	}
}

