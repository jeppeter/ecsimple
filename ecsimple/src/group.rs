


use crate::bngf2m::*;
use num_bigint::{BigInt,Sign};
use num_traits::{zero};
use crate::*;
use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::consts::*;
use std::error::Error;
use hex::FromHex;

ecsimple_error_class!{ECGroupError}


pub trait ECGroup : Send + Sync {
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
	pub p :BigInt,
	pub order :BigInt,
	pub cofactor :BigInt,
	pub curvename :String,
	pub a :BnGf2m,
	pub b :BnGf2m,
}

impl std::fmt::Display for ECGroupBnGf2m {
	fn fmt(&self, f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f,"curve {} generator {} p 0x{:x} order 0x{:x} cofactor 0x{:x} a 0x{:x} b 0x{:x}", 
			self.curvename, self.generator,self.p,self.order,self.cofactor,self.a, self.b)
	}
}

impl std::default::Default for ECGroupBnGf2m {
	fn default() -> Self {
		ECGroupBnGf2m {
			generator : ECBnGf2mGenerator::default(),
			p : zero(),
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

unsafe impl Send for ECGroupBnGf2m {}
unsafe impl Sync for ECGroupBnGf2m {}


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

unsafe impl Send for ECGroupPrime {}
unsafe impl Sync for ECGroupPrime {}


fn create_group_curves() -> HashMap<String,Box<dyn ECGroup>> {
	let mut retv :HashMap<String,Box<dyn ECGroup>> = HashMap::new();
	let mut bngrp :ECGroupBnGf2m = ECGroupBnGf2m::default();
	let mut v8 :Vec<u8>;
	let mut p :BigInt;

	v8 = Vec::from_hex("0800000000000000000000000000000000000000C9").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	v8 = Vec::from_hex("000000000000000000000000000000000000000001").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.a = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("000000000000000000000000000000000000000001").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("02FE13C0537BBC11ACAA07D793DE4E6D5E5C94EEE8").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("0289070FB05D38FF58321F2E800536D538CCDAA3D9").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = BnGf2m::new_from_bigint(&p);
	bngrp.generator.z = BnGf2m::one();

	v8 = Vec::from_hex("04000000000000000000020108A2E0CC0D99F8A5EF").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();

	retv.insert(SECT163k1_NAME.to_string(),Box::new(bngrp.clone()));

	retv
}


lazy_static ! {
	static ref ECC_CURVES :HashMap<String,Box<dyn ECGroup>> = {
		create_group_curves()	
	};
}


pub fn get_group_curve(name :&str) -> Result<Box<dyn ECGroup>,Box<dyn Error>> {
	let retv :Box<dyn ECGroup>;

	match ECC_CURVES.get(name) {
		Some(pv) => {
			return Ok(*(pv.clone()));
		},
		_ => {
			ecsimple_new_error!{ECGroupError,"can not find [{}]",name}
		}
	}
}