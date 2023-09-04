

use crate::bngf2m::*;
#[allow(unused_imports)]
use crate::logger::*;
use crate::*;
use num_bigint::{BigInt,Sign};
use num_traits::{zero,one};
//use crate::*;
use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::consts::*;
use crate::mont::*;
use crate::randop::*;
use std::error::Error;
use hex::FromHex;
use std::cmp::PartialEq;

ecsimple_error_class!{ECGroupError}


pub trait ECGroup  {
	fn x(&self) -> BigInt ;
	fn y(&self) -> BigInt ;
	fn z(&self) -> BigInt ;
	fn degree(&self) -> i64;
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

	pub fn eq_op(&self,other :&ECBnGf2mGenerator) -> bool {
		let mut retv :bool = true;
		if !self.x.eq_op(&other.x) {
			retv = false;
		}

		if !self.y.eq_op(&other.y) {
			retv = false;
		}

		if !self.z.eq_op(&other.z) {
			retv = false;
		}
		return retv;
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

impl ECGroupBnGf2m {
	pub fn eq_op(&self,other :&ECGroupBnGf2m) -> bool {
		let mut retv :bool = true;
		if !self.generator.eq_op(&other.generator) {
			retv = false;
		}
		if self.p != other.p {
			retv= false;
		}
		if self.order != other.order {
			retv = false;
		}
		if self.cofactor != other.cofactor {
			retv = false;
		}

		if self.curvename != other.curvename {
			retv = false;
		}

		if !self.a.eq_op(&other.a) {
			retv = false;
		}

		if ! self.b.eq_op(&other.b) {
			retv = false;
		}

		return retv;
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

	fn degree(&self) -> i64 {
		return get_max_bits(&self.p) - 1;
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

	pub fn eq_op(&self,other :&ECPrimeGenerator) -> bool {
		let mut retv :bool = true;
		if self.x != other.x {
			retv = false;
		}

		if self.y != other.y {
			retv = false;
		}

		if self.z != other.z {
			retv = false;
		}
		return retv;
	}
}

impl PartialEq for ECPrimeGenerator {
	fn eq(&self, other:&Self) -> bool {
		return self.eq_op(other);
	}

	fn ne(&self, other:&Self) -> bool {
		return ! self.eq_op(other);
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
	pub p :BigInt,
	pub order :BigInt,
	pub cofactor :BigInt,
	pub curvename :String,
	pub a :BigInt,
	pub b :BigInt,
	pub is_minus3 :bool,
	pub specflags : u32,
}

impl std::fmt::Display for ECGroupPrime {
	fn fmt(&self, f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f,"curve {} generator {} p 0x{:X} order 0x{:x} cofactor 0x{:x} a 0x{:x} b 0x{:x} specflags 0x{:x}", 
			self.curvename, self.generator, self.p,self.order,self.cofactor,self.a, self.b,self.specflags)
	}
}

impl std::default::Default for ECGroupPrime {
	fn default() -> Self {
		ECGroupPrime {
			generator : ECPrimeGenerator::default(),
			p : zero(),
			order :zero(),
			cofactor :zero(),
			curvename : "".to_string(),
			a : zero(),
			b : zero(),
			is_minus3 : false,
			specflags : 0,
		}
	}
}

impl ECGroupPrime {
	pub fn is_nist224(&self) -> bool {
		if (self.specflags & NIST224_SPEC_FLAGS) != 0 {
			return true;
		}
		return false;
	}

	pub fn eq_op(&self, other :&ECGroupPrime) -> bool {
		let mut retv : bool = true;
		if !self.generator.eq_op(&other.generator) {
			retv = false;
		}
		if self.p != other.p {
			retv = false;
		}

		if self.order != other.order {
			retv = false;
		}

		if self.cofactor != other.cofactor {
			retv = false;
		}

		if self.curvename != other.curvename {
			retv = false;
		}

		if self.a != other.a {
			retv = false;
		}

		if self.b != other.b {
			retv = false;
		}

		if self.specflags != other.specflags {
			retv = false;
		}

		return retv;
	}
}

impl PartialEq for ECGroupPrime {
	fn eq(&self, other:&Self) -> bool {
		return self.eq_op(other);
	}

	fn ne(&self, other:&Self) -> bool {
		return ! self.eq_op(other);
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

	fn degree(&self) -> i64 {
		return get_max_bits(&self.p) - 1;
	}
}



fn create_group_bn_curves() -> HashMap<String,ECGroupBnGf2m> {
	let mut retv :HashMap<String,ECGroupBnGf2m> = HashMap::new();
	let mut bngrp :ECGroupBnGf2m = ECGroupBnGf2m::default();
	let mut v8 :Vec<u8>;
	let mut p :BigInt;
	let ov :BigInt = one();

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
	bngrp.cofactor = &ov + &ov;
	bngrp.curvename = SECT163k1_NAME.to_string();

	retv.insert(SECT163k1_NAME.to_string(),bngrp.clone());

	retv
}


fn create_group_prime_curves() -> HashMap<String,ECGroupPrime> {
	let mut retv :HashMap<String,ECGroupPrime> = HashMap::new();
	let mut bngrp :ECGroupPrime = ECGroupPrime::default();
	let mut v8 :Vec<u8>;
	let mut p :BigInt;
	let mut tmpp :BigInt;
	let mut tmpa :BigInt;
	let ov :BigInt = one();
	//let mut montv :MontNum;
	let mut montv :MontNum;

	v8 = Vec::from_hex("DB7C2ABF62E35E668076BEAD208B").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	montv = MontNum::new(&bngrp.p).unwrap();
	tmpp = p.clone();
	v8 = Vec::from_hex("DB7C2ABF62E35E668076BEAD2088").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	tmpa = p.clone();
	bngrp.a = montv.mont_to(&p);
	v8 = Vec::from_hex("659EF8BA043916EEDE8911702B22").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = montv.mont_to(&p);
	v8 = Vec::from_hex("09487239995A5EE76B55F9C2F098").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = montv.mont_to(&p);
	v8 = Vec::from_hex("A89CE5AF8724C0A23E0E0FF77500").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = montv.mont_to(&p);
	bngrp.generator.z = montv.mont_to(&ov);

	v8 = Vec::from_hex("DB7C2ABF62E35E7628DFAC6561C5").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = ov.clone();
	bngrp.curvename = SECP112r1_NAME.to_string();

	//ecsimple_log_trace!("tmpp 0x{:X} tmpa 0x{:X}",tmpp,tmpa);
	if tmpp == (tmpa.clone() + ov.clone() + ov.clone() + ov.clone()) {
		bngrp.is_minus3 = true;
		//ecsimple_log_trace!("{} is_minus3 true",SECP112r1_NAME);
	} else {
		bngrp.is_minus3 = false;
		//ecsimple_log_trace!("{} is_minus3 false",SECP112r1_NAME);
	}
	bngrp.specflags = 0;
	retv.insert(SECP112r1_NAME.to_string(),bngrp.clone());


	v8 = Vec::from_hex("fffffffffffffffffffffffffffffffeffffffffffffffff").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	montv = MontNum::new(&bngrp.p).unwrap();
	tmpp = p.clone();
	v8 = Vec::from_hex("fffffffffffffffffffffffffffffffefffffffffffffffc").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	tmpa = p.clone();
	bngrp.a = montv.mont_to(&p);
	v8 = Vec::from_hex("64210519e59c80e70fa7e9ab72243049feb8deecc146b9b1").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = montv.mont_to(&p);
	v8 = Vec::from_hex("188da80eb03090f67cbf20eb43a18800f4ff0afd82ff1012").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = montv.mont_to(&p);
	v8 = Vec::from_hex("07192b95ffc8da78631011ed6b24cdd573f977a11e794811").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = montv.mont_to(&p);
	bngrp.generator.z = montv.mont_to(&ov);

	v8 = Vec::from_hex("ffffffffffffffffffffffff99def836146bc9b1b4d22831").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = ov.clone();
	bngrp.curvename = PRIME192v1_NAME.to_string();

	//ecsimple_log_trace!("tmpp 0x{:X} tmpa 0x{:X}",tmpp,tmpa);
	if tmpp == (tmpa.clone() + ov.clone() + ov.clone() + ov.clone()) {
		bngrp.is_minus3 = true;
		//ecsimple_log_trace!("{} is_minus3 true",PRIME192v1_NAME);
	} else {
		bngrp.is_minus3 = false;
		//ecsimple_log_trace!("{} is_minus3 false",PRIME192v1_NAME);
	}
	bngrp.specflags = 0;
	retv.insert(PRIME192v1_NAME.to_string(),bngrp.clone());



	v8 = Vec::from_hex("ffffffffffffffffffffffffffffffff000000000000000000000001").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	montv = MontNum::new(&bngrp.p).unwrap();
	tmpp = p.clone();
	v8 = Vec::from_hex("fffffffffffffffffffffffffffffffefffffffffffffffffffffffe").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	tmpa = p.clone();
	bngrp.a = montv.mont_to(&p);
	v8 = Vec::from_hex("b4050a850c04b3abf54132565044b0b7d7bfd8ba270b39432355ffb4").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = montv.mont_to(&p);
	v8 = Vec::from_hex("b70e0cbd6bb4bf7f321390b94a03c1d356c21122343280d6115c1d21").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = montv.mont_to(&p);
	v8 = Vec::from_hex("bd376388b5f723fb4c22dfe6cd4375a05a07476444d5819985007e34").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = montv.mont_to(&p);
	bngrp.generator.z = montv.mont_to(&ov);

	v8 = Vec::from_hex("ffffffffffffffffffffffffffff16a2e0b8f03e13dd29455c5c2a3d").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = ov.clone();
	bngrp.curvename = SECP224r1_NAME.to_string();

	//ecsimple_log_trace!("tmpp 0x{:X} tmpa 0x{:X}",tmpp,tmpa);
	if tmpp == (tmpa.clone() + ov.clone() + ov.clone() + ov.clone()) {
		bngrp.is_minus3 = true;
		//ecsimple_log_trace!("{} is_minus3 true",SECP224r1_NAME);
	} else {
		bngrp.is_minus3 = false;
		//ecsimple_log_trace!("{} is_minus3 false",SECP224r1_NAME);
	}
	bngrp.specflags = NIST224_SPEC_FLAGS;
	retv.insert(SECP224r1_NAME.to_string(),bngrp.clone());


	/*SECP384r1*/
	v8 = Vec::from_hex("fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffeffffffff0000000000000000ffffffff").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	montv = MontNum::new(&bngrp.p).unwrap();
	tmpp = p.clone();
	v8 = Vec::from_hex("fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffeffffffff0000000000000000fffffffc").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	tmpa = p.clone();
	bngrp.a = montv.mont_to(&p);
	v8 = Vec::from_hex("b3312fa7e23ee7e4988e056be3f82d19181d9c6efe8141120314088f5013875ac656398d8a2ed19d2a85c8edd3ec2aef").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = montv.mont_to(&p);
	v8 = Vec::from_hex("aa87ca22be8b05378eb1c71ef320ad746e1d3b628ba79b9859f741e082542a385502f25dbf55296c3a545e3872760ab7").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = montv.mont_to(&p);
	v8 = Vec::from_hex("3617de4a96262c6f5d9e98bf9292dc29f8f41dbd289a147ce9da3113b5f0b8c00a60b1ce1d7e819d7a431d7c90ea0e5f").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = montv.mont_to(&p);
	bngrp.generator.z = montv.mont_to(&ov);

	v8 = Vec::from_hex("ffffffffffffffffffffffffffffffffffffffffffffffffc7634d81f4372ddf581a0db248b0a77aecec196accc52973").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = ov.clone();
	bngrp.curvename = SECP384r1_NAME.to_string();

	//ecsimple_log_trace!("tmpp 0x{:X} tmpa 0x{:X}",tmpp,tmpa);
	if tmpp == (tmpa.clone() + ov.clone() + ov.clone() + ov.clone()) {
		bngrp.is_minus3 = true;
		//ecsimple_log_trace!("{} is_minus3 true",SECP384r1_NAME);
	} else {
		bngrp.is_minus3 = false;
		//ecsimple_log_trace!("{} is_minus3 false",SECP384r1_NAME);
	}
	bngrp.specflags = 0;
	retv.insert(SECP384r1_NAME.to_string(),bngrp.clone());


	/*SECP521r1*/
	v8 = Vec::from_hex("01ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	montv = MontNum::new(&bngrp.p).unwrap();
	tmpp = p.clone();
	v8 = Vec::from_hex("01fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	tmpa = p.clone();
	bngrp.a = montv.mont_to(&p);
	v8 = Vec::from_hex("0051953eb9618e1c9a1f929a21a0b68540eea2da725b99b315f3b8b489918ef109e156193951ec7e937b1652c0bd3bb1bf073573df883d2c34f1ef451fd46b503f00").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = montv.mont_to(&p);
	v8 = Vec::from_hex("00c6858e06b70404e9cd9e3ecb662395b4429c648139053fb521f828af606b4d3dbaa14b5e77efe75928fe1dc127a2ffa8de3348b3c1856a429bf97e7e31c2e5bd66").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = montv.mont_to(&p);
	v8 = Vec::from_hex("011839296a789a3bc0045c8a5fb42c7d1bd998f54449579b446817afbd17273e662c97ee72995ef42640c550b9013fad0761353c7086a272c24088be94769fd16650").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = montv.mont_to(&p);
	bngrp.generator.z = montv.mont_to(&ov);

	v8 = Vec::from_hex("01fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa51868783bf2f966b7fcc0148f709a5d03bb5c9b8899c47aebb6fb71e91386409").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = ov.clone();
	bngrp.curvename = SECP521r1_NAME.to_string();

	//ecsimple_log_trace!("tmpp 0x{:X} tmpa 0x{:X}",tmpp,tmpa);
	if tmpp == (tmpa.clone() + ov.clone() + ov.clone() + ov.clone()) {
		bngrp.is_minus3 = true;
		//ecsimple_log_trace!("{} is_minus3 true",SECP521r1_NAME);
	} else {
		bngrp.is_minus3 = false;
		//ecsimple_log_trace!("{} is_minus3 false",SECP521r1_NAME);
	}
	bngrp.specflags = 0;
	retv.insert(SECP521r1_NAME.to_string(),bngrp.clone());


	retv
}


lazy_static ! {
	static ref ECC_BN_CURVES :HashMap<String,ECGroupBnGf2m> = {
		create_group_bn_curves()	
	};

	static ref ECC_PRIME_CURVES :HashMap<String,ECGroupPrime> = {
		create_group_prime_curves()
	};


}


pub fn get_bn_group_curve(name :&str) -> Result<ECGroupBnGf2m,Box<dyn Error>> {
	match ECC_BN_CURVES.get(name) {
		Some(pv) => {
			return Ok(pv.clone());
		},
		_ => {
			ecsimple_new_error!{ECGroupError,"can not find [{}]",name}
		}
	}
}

pub fn get_prime_group_curve(name :&str) -> Result<ECGroupPrime,Box<dyn Error>> {
	match ECC_PRIME_CURVES.get(name) {
		Some(pv) => {
			ecsimple_log_trace!("load [{}]",name);
			return Ok(pv.clone());
		},
		_ => {
			ecsimple_new_error!{ECGroupError,"can not find [{}]",name}
		}
	}
}