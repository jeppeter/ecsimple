

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


pub (crate) trait ECGroupInterface  {
	fn x(&self) -> BigInt ;
	fn y(&self) -> BigInt ;
	fn z(&self) -> BigInt ;
	fn degree(&self) -> i64;
}

#[derive(Clone)]
pub (crate) struct ECBnGf2mGenerator {
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
pub (crate) struct ECGroupBnGf2m {
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

impl ECGroupInterface for ECGroupBnGf2m {
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
pub (crate) struct ECPrimeGenerator {
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
pub (crate) struct ECGroupPrime {
	pub (crate) generator :ECPrimeGenerator,
	pub (crate) p :BigInt,
	pub (crate) order :BigInt,
	pub (crate) cofactor :BigInt,
	pub (crate) curvename :String,
	pub (crate) a :BigInt,
	pub (crate) b :BigInt,
	pub (crate) is_minus3 :bool,
}

impl std::fmt::Display for ECGroupPrime {
	fn fmt(&self, f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f,"curve {} generator {} p 0x{:X} order 0x{:x} cofactor 0x{:x} a 0x{:x} b 0x{:x}", 
			self.curvename, self.generator, self.p,self.order,self.cofactor,self.a, self.b)
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
		}
	}
}

impl ECGroupPrime {
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


impl ECGroupInterface for ECGroupPrime {
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
		return get_max_bits(&self.p);
	}
}

#[derive(Clone)]
pub struct ECGroup {
	pub (crate) bngrp :Option<ECGroupBnGf2m>,
	pub (crate) primegrp :Option<ECGroupPrime>,
}

impl Default for ECGroup {
	fn default() -> Self {
		Self {
			bngrp : None,
			primegrp : None,
		}
	}
}

impl ECGroup {
	pub (crate) fn new_bn_group( grp:&ECGroupBnGf2m) -> Self {
		Self {
			bngrp : Some(grp.clone()),
			primegrp : None,
		}
	}

	pub (crate) fn new_prime_group(grp :&ECGroupPrime) -> Self {
		Self {
			bngrp : None,
			primegrp : Some(grp.clone()),
		}
	}

	pub fn new_bn_group_base(p :&BigInt,a :&BigInt,b :&BigInt,x :&BigInt,y :&BigInt, z :&BigInt, order :&BigInt,cofactor :&BigInt,name :&str) -> Result<Self,Box<dyn Error>> {
		let mut bngrp :ECGroupBnGf2m = ECGroupBnGf2m::default();
		bngrp.p = p.clone();
		bngrp.a = BnGf2m::new_from_bigint(a);
		bngrp.b = BnGf2m::new_from_bigint(b);
		bngrp.generator.x = BnGf2m::new_from_bigint(x);
		bngrp.generator.y = BnGf2m::new_from_bigint(y);
		bngrp.generator.z = BnGf2m::new_from_bigint(z);
		bngrp.order = order.clone();
		bngrp.cofactor = cofactor.clone();
		bngrp.curvename = format!("{}",name);
		Ok(Self {
			bngrp : Some(bngrp.clone()),
			primegrp : None,
		})
	}

	pub fn new_prime_group_base(p :&BigInt,a :&BigInt, b:&BigInt, x :&BigInt,y :&BigInt, z :&BigInt, order :&BigInt, cofactor :&BigInt,name :&str) -> Result<Self,Box<dyn Error>> {
		let mut primegrp :ECGroupPrime = ECGroupPrime::default();
		let montv :MontNum;
		let ov :BigInt = one();
		let tmpp :BigInt;
		let tmpa :BigInt;
		primegrp.p = p.clone();
		montv = MontNum::new(&primegrp.p).unwrap();
		primegrp.a = montv.mont_to(a);
		primegrp.b = montv.mont_to(b);
		primegrp.generator.x = montv.mont_to(x);
		primegrp.generator.y = montv.mont_to(y);
		primegrp.generator.z = montv.mont_to(z);
		primegrp.order = order.clone();
		primegrp.cofactor = cofactor.clone();
		primegrp.curvename = format!("{}",name);
		tmpp = p.clone();
		tmpa = a.clone();

		if tmpp == (tmpa + ov.clone() + ov.clone() + ov.clone()) {
			primegrp.is_minus3 = true;
		} else {
			primegrp.is_minus3 = false;
		}
		Ok(Self {
			bngrp : None,
			primegrp : Some(primegrp.clone()),
		})
	}

	pub (crate) fn is_bn_group(&self) -> bool {
		if self.bngrp.is_some() {
			return true;
		}
		return false;
	}

	pub (crate) fn is_prime_group(&self) -> bool {
		if self.primegrp.is_some() {
			return true;
		}
		return false;
	}

	pub (crate) fn get_bn_group(&self) -> ECGroupBnGf2m {
		let mut retv :ECGroupBnGf2m = ECGroupBnGf2m::default();
		if self.is_bn_group() {
			retv = self.bngrp.as_ref().unwrap().clone();
		}
		return retv;
	}

	pub (crate) fn get_prime_group(&self) -> ECGroupPrime {
		let mut retv :ECGroupPrime = ECGroupPrime::default();
		if self.is_prime_group() {
		 	retv = self.primegrp.as_ref().unwrap().clone();
		}
		return retv;
	}
}



fn create_group_bn_curves() -> HashMap<String,ECGroupBnGf2m> {
	let mut retv :HashMap<String,ECGroupBnGf2m> = HashMap::new();
	let mut bngrp :ECGroupBnGf2m = ECGroupBnGf2m::default();
	let mut v8 :Vec<u8>;
	let mut p :BigInt;
	let ov :BigInt = one();

	/*sect113r1*/
	v8 = Vec::from_hex("020000000000000000000000000201").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	v8 = Vec::from_hex("003088250ca6e7c7fe649ce85820f7").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.a = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("00e8bee4d3e2260744188be0e9c723").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("009d73616f35f4ab1407d73562c10f").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("00a52830277958ee84d1315ed31886").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = BnGf2m::new_from_bigint(&p);
	bngrp.generator.z = BnGf2m::one();

	v8 = Vec::from_hex("0100000000000000d9ccec8a39e56f").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = &ov + &ov;
	bngrp.curvename = SECT113r1_NAME.to_string();

	retv.insert(SECT113r1_NAME.to_string(),bngrp.clone());

	/*sect113r2*/
	v8 = Vec::from_hex("020000000000000000000000000201").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	v8 = Vec::from_hex("00689918dbec7e5a0dd6dfc0aa55c7").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.a = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("0095e9a9ec9b297bd4bf36e059184f").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("01a57a6a7b26ca5ef52fcdb8164797").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("00b3adc94ed1fe674c06e695baba1d").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = BnGf2m::new_from_bigint(&p);
	bngrp.generator.z = BnGf2m::one();

	v8 = Vec::from_hex("010000000000000108789b2496af93").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = &ov + &ov;
	bngrp.curvename = SECT113r2_NAME.to_string();

	retv.insert(SECT113r2_NAME.to_string(),bngrp.clone());

	/*sect131r1*/
	v8 = Vec::from_hex("080000000000000000000000000000010d").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	v8 = Vec::from_hex("07a11b09a76b562144418ff3ff8c2570b8").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.a = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("0217c05610884b63b9c6c7291678f9d341").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("0081baf91fdf9833c40f9c181343638399").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("078c6e7ea38c001f73c8134b1b4ef9e150").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = BnGf2m::new_from_bigint(&p);
	bngrp.generator.z = BnGf2m::one();

	v8 = Vec::from_hex("0400000000000000023123953a9464b54d").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = &ov + &ov;
	bngrp.curvename = SECT131r1_NAME.to_string();

	retv.insert(SECT131r1_NAME.to_string(),bngrp.clone());

	/*sect131r2*/
	v8 = Vec::from_hex("080000000000000000000000000000010d").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	v8 = Vec::from_hex("03e5a88919d7cafcbf415f07c2176573b2").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.a = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("04b8266a46c55657ac734ce38f018f2192").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("0356dcd8f2f95031ad652d23951bb366a8").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("0648f06d867940a5366d9e265de9eb240f").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = BnGf2m::new_from_bigint(&p);
	bngrp.generator.z = BnGf2m::one();

	v8 = Vec::from_hex("0400000000000000016954a233049ba98f").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = &ov + &ov;
	bngrp.curvename = SECT131r2_NAME.to_string();

	retv.insert(SECT131r2_NAME.to_string(),bngrp.clone());

	/*sect163k1*/
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

	/*sect163r1*/
	v8 = Vec::from_hex("0800000000000000000000000000000000000000C9").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	v8 = Vec::from_hex("07b6882caaefa84f9554ff8428bd88e246d2782ae2").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.a = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("0713612dcddcb40aab946bda29ca91f73af958afd9").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("0369979697ab43897789566789567f787a7876a654").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("00435edb42efafb2989d51fefce3c80988f41ff883").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = BnGf2m::new_from_bigint(&p);
	bngrp.generator.z = BnGf2m::one();

	v8 = Vec::from_hex("03ffffffffffffffffffff48aab689c29ca710279b").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = &ov + &ov;
	bngrp.curvename = SECT163r1_NAME.to_string();

	retv.insert(SECT163r1_NAME.to_string(),bngrp.clone());

	/*sect163r2*/
	v8 = Vec::from_hex("0800000000000000000000000000000000000000C9").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	v8 = Vec::from_hex("000000000000000000000000000000000000000001").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.a = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("020a601907b8c953ca1481eb10512f78744a3205fd").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("03f0eba16286a2d57ea0991168d4994637e8343e36").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("00d51fbc6c71a0094fa2cdd545b11c5c0c797324f1").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = BnGf2m::new_from_bigint(&p);
	bngrp.generator.z = BnGf2m::one();

	v8 = Vec::from_hex("040000000000000000000292fe77e70c12a4234c33").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = &ov + &ov;
	bngrp.curvename = SECT163r2_NAME.to_string();

	retv.insert(SECT163r2_NAME.to_string(),bngrp.clone());

	/*sect193r1*/
	v8 = Vec::from_hex("02000000000000000000000000000000000000000000008001").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	v8 = Vec::from_hex("0017858feb7a98975169e171f77b4087de098ac8a911df7b01").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.a = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("00fdfb49bfe6c3a89facadaa7a1e5bbc7cc1c2e5d831478814").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("01f481bc5f0ff84a74ad6cdf6fdef4bf6179625372d8c0c5e1").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("0025e399f2903712ccf3ea9e3a1ad17fb0b3201b6af7ce1b05").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = BnGf2m::new_from_bigint(&p);
	bngrp.generator.z = BnGf2m::one();

	v8 = Vec::from_hex("01000000000000000000000000c7f34a778f443acc920eba49").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = &ov + &ov;
	bngrp.curvename = SECT193r1_NAME.to_string();

	retv.insert(SECT193r1_NAME.to_string(),bngrp.clone());

	/*sect193r2*/
	v8 = Vec::from_hex("02000000000000000000000000000000000000000000008001").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	v8 = Vec::from_hex("0163f35a5137c2ce3ea6ed8667190b0bc43ecd69977702709b").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.a = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("00c9bb9e8927d4d64c377e2ab2856a5b16e3efb7f61d4316ae").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("00d9b67d192e0367c803f39e1a7e82ca14a651350aae617e8f").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("01ce94335607c304ac29e7defbd9ca01f596f927224cdecf6c").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = BnGf2m::new_from_bigint(&p);
	bngrp.generator.z = BnGf2m::one();

	v8 = Vec::from_hex("010000000000000000000000015aab561b005413ccd4ee99d5").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = &ov + &ov;
	bngrp.curvename = SECT193r2_NAME.to_string();

	retv.insert(SECT193r2_NAME.to_string(),bngrp.clone());

	/*sect233k1*/
	v8 = Vec::from_hex("020000000000000000000000000000000000000004000000000000000001").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	v8 = Vec::from_hex("000000000000000000000000000000000000000000000000000000000000").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.a = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("000000000000000000000000000000000000000000000000000000000001").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("017232ba853a7e731af129f22ff4149563a419c26bf50a4c9d6eefad6126").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("01db537dece819b7f70f555a67c427a8cd9bf18aeb9b56e0c11056fae6a3").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = BnGf2m::new_from_bigint(&p);
	bngrp.generator.z = BnGf2m::one();

	v8 = Vec::from_hex("8000000000000000000000000000069d5bb915bcd46efb1ad5f173abdf").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = &ov + &ov + &ov + &ov;
	bngrp.curvename = SECT233k1_NAME.to_string();

	retv.insert(SECT233k1_NAME.to_string(),bngrp.clone());

	/*sect233r1*/
	v8 = Vec::from_hex("020000000000000000000000000000000000000004000000000000000001").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	v8 = Vec::from_hex("000000000000000000000000000000000000000000000000000000000001").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.a = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("0066647ede6c332c7f8c0923bb58213b333b20e9ce4281fe115f7d8f90ad").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("00fac9dfcbac8313bb2139f1bb755fef65bc391f8b36f8f8eb7371fd558b").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("01006a08a41903350678e58528bebf8a0beff867a7ca36716f7e01f81052").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = BnGf2m::new_from_bigint(&p);
	bngrp.generator.z = BnGf2m::one();

	v8 = Vec::from_hex("01000000000000000000000000000013e974e72f8a6922031d2603cfe0d7").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = &ov + &ov;
	bngrp.curvename = SECT233r1_NAME.to_string();

	retv.insert(SECT233r1_NAME.to_string(),bngrp.clone());

	/*sect239k1*/
	v8 = Vec::from_hex("800000000000000000004000000000000000000000000000000000000001").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	v8 = Vec::from_hex("000000000000000000000000000000000000000000000000000000000000").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.a = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("000000000000000000000000000000000000000000000000000000000001").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("29a0b6a887a983e9730988a68727a8b2d126c44cc2cc7b2a6555193035dc").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("76310804f12e549bdb011c103089e73510acb275fc312a5dc6b76553f0ca").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = BnGf2m::new_from_bigint(&p);
	bngrp.generator.z = BnGf2m::one();

	v8 = Vec::from_hex("2000000000000000000000000000005a79fec67cb6e91f1c1da800e478a5").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = &ov + &ov + &ov + &ov;
	bngrp.curvename = SECT239k1_NAME.to_string();

	retv.insert(SECT239k1_NAME.to_string(),bngrp.clone());

	/*sect283k1*/
	v8 = Vec::from_hex("0800000000000000000000000000000000000000000000000000000000000000000010a1").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	v8 = Vec::from_hex("000000000000000000000000000000000000000000000000000000000000000000000000").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.a = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("000000000000000000000000000000000000000000000000000000000000000000000001").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("0503213f78ca44883f1a3b8162f188e553cd265f23c1567a16876913b0c2ac2458492836").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("01ccda380f1c9e318d90f95d07e5426fe87e45c0e8184698e45962364e34116177dd2259").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = BnGf2m::new_from_bigint(&p);
	bngrp.generator.z = BnGf2m::one();

	v8 = Vec::from_hex("01ffffffffffffffffffffffffffffffffffe9ae2ed07577265dff7f94451e061e163c61").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = &ov + &ov + &ov + &ov;
	bngrp.curvename = SECT283k1_NAME.to_string();

	retv.insert(SECT283k1_NAME.to_string(),bngrp.clone());

	/*sect283r1*/
	v8 = Vec::from_hex("0800000000000000000000000000000000000000000000000000000000000000000010a1").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	v8 = Vec::from_hex("000000000000000000000000000000000000000000000000000000000000000000000001").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.a = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("027b680ac8b8596da5a4af8a19a0303fca97fd7645309fa2a581485af6263e313b79a2f5").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("05f939258db7dd90e1934f8c70b0dfec2eed25b8557eac9c80e2e198f8cdbecd86b12053").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("03676854fe24141cb98fe6d4b20d02b4516ff702350eddb0826779c813f0df45be8112f4").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = BnGf2m::new_from_bigint(&p);
	bngrp.generator.z = BnGf2m::one();

	v8 = Vec::from_hex("03ffffffffffffffffffffffffffffffffffef90399660fc938a90165b042a7cefadb307").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = &ov + &ov;
	bngrp.curvename = SECT283r1_NAME.to_string();

	retv.insert(SECT283r1_NAME.to_string(),bngrp.clone());

	/*sect409k1*/
	v8 = Vec::from_hex("02000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000001").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	v8 = Vec::from_hex("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.a = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("0060f05f658f49c1ad3ab1890f7184210efd0987e307c84c27accfb8f9f67cc2c460189eb5aaaa62ee222eb1b35540cfe9023746").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("01e369050b7c4e42acba1dacbf04299c3460782f918ea427e6325165e9ea10e3da5f6c42e9c55215aa9ca27a5863ec48d8e0286b").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = BnGf2m::new_from_bigint(&p);
	bngrp.generator.z = BnGf2m::one();

	v8 = Vec::from_hex("007ffffffffffffffffffffffffffffffffffffffffffffffffffe5f83b2d4ea20400ec4557d5ed3e3e7ca5b4b5c83b8e01e5fcf").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = &ov + &ov + &ov + &ov;
	bngrp.curvename = SECT409k1_NAME.to_string();

	retv.insert(SECT409k1_NAME.to_string(),bngrp.clone());


	/*sect409r1*/
	v8 = Vec::from_hex("02000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000001").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	v8 = Vec::from_hex("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.a = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("0021a5c2c8ee9feb5c4b9a753b7b476b7fd6422ef1f3dd674761fa99d6ac27c8a9a197b272822f6cd57a55aa4f50ae317b13545f").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("015d4860d088ddb3496b0c6064756260441cde4af1771d4db01ffe5b34e59703dc255a868a1180515603aeab60794e54bb7996a7").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("0061b1cfab6be5f32bbfa78324ed106a7636b9c5a7bd198d0158aa4f5488d08f38514f1fdf4b4f40d2181b3681c364ba0273c706").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = BnGf2m::new_from_bigint(&p);
	bngrp.generator.z = BnGf2m::one();

	v8 = Vec::from_hex("010000000000000000000000000000000000000000000000000001e2aad6a612f33307be5fa47c3c9e052f838164cd37d9a21173").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = &ov + &ov;
	bngrp.curvename = SECT409r1_NAME.to_string();

	retv.insert(SECT409r1_NAME.to_string(),bngrp.clone());

	/*sect571k1*/
	v8 = Vec::from_hex("080000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000425").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	v8 = Vec::from_hex("000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.a = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("026eb7a859923fbc82189631f8103fe4ac9ca2970012d5d46024804801841ca44370958493b205e647da304db4ceb08cbbd1ba39494776fb988b47174dca88c7e2945283a01c8972").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("0349dc807f4fbf374f4aeade3bca95314dd58cec9f307a54ffc61efc006d8a2c9d4979c0ac44aea74fbebbb9f772aedcb620b01a7ba7af1b320430c8591984f601cd4c143ef1c7a3").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = BnGf2m::new_from_bigint(&p);
	bngrp.generator.z = BnGf2m::one();

	v8 = Vec::from_hex("020000000000000000000000000000000000000000000000000000000000000000000000131850e1f19a63e4b391a8db917f4138b630d84be5d639381e91deb45cfe778f637c1001").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = &ov + &ov + &ov + &ov;
	bngrp.curvename = SECT571k1_NAME.to_string();

	retv.insert(SECT571k1_NAME.to_string(),bngrp.clone());

	/*sect571r1*/
	v8 = Vec::from_hex("080000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000425").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	v8 = Vec::from_hex("000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.a = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("02f40e7e2221f295de297117b7f3d62f5c6a97ffcb8ceff1cd6ba8ce4a9a18ad84ffabbd8efa59332be7ad6756a66e294afd185a78ff12aa520e4de739baca0c7ffeff7f2955727a").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("0303001d34b856296c16c0d40d3cd7750a93d1d2955fa80aa5f40fc8db7b2abdbde53950f4c0d293cdd711a35b67fb1499ae60038614f1394abfa3b4c850d927e1e7769c8eec2d19").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("037bf27342da639b6dccfffeb73d69d78c6c27a6009cbbca1980f8533921e8a684423e43bab08a576291af8f461bb2a8b3531d2f0485c19b16e2f1516e23dd3c1a4827af1b8ac15b").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = BnGf2m::new_from_bigint(&p);
	bngrp.generator.z = BnGf2m::one();

	v8 = Vec::from_hex("03ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe661ce18ff55987308059b186823851ec7dd9ca1161de93d5174d66e8382e9bb2fe84e47").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = &ov + &ov;
	bngrp.curvename = SECT571r1_NAME.to_string();

	retv.insert(SECT571r1_NAME.to_string(),bngrp.clone());


	/*c2pnb163v1*/
	v8 = Vec::from_hex("080000000000000000000000000000000000000107").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	v8 = Vec::from_hex("072546b5435234a422e0789675f432c89435de5242").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.a = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("00c9517d06d5240d3cff38c74b20b6cd4d6f9dd4d9").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("07af69989546103d79329fcc3d74880f33bbe803cb").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("01ec23211b5966adea1d3f87f7ea5848aef0b7ca9f").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = BnGf2m::new_from_bigint(&p);
	bngrp.generator.z = BnGf2m::one();

	v8 = Vec::from_hex("0400000000000000000001e60fc8821cc74daeafc1").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = &ov + &ov;
	bngrp.curvename = C2PNB163v1_NAME.to_string();

	retv.insert(C2PNB163v1_NAME.to_string(),bngrp.clone());

	/*c2pnb163v2*/
	v8 = Vec::from_hex("080000000000000000000000000000000000000107").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	v8 = Vec::from_hex("0108b39e77c4b108bed981ed0e890e117c511cf072").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.a = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("0667aceb38af4e488c407433ffae4f1c811638df20").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("0024266e4eb5106d0a964d92c4860e2671db9b6cc5").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("079f684ddf6684c5cd258b3890021b2386dfd19fc5").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = BnGf2m::new_from_bigint(&p);
	bngrp.generator.z = BnGf2m::one();

	v8 = Vec::from_hex("03fffffffffffffffffffdf64de1151adbb78f10a7").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = &ov + &ov;
	bngrp.curvename = C2PNB163v2_NAME.to_string();

	retv.insert(C2PNB163v2_NAME.to_string(),bngrp.clone());

	/*c2pnb163v3*/
	v8 = Vec::from_hex("080000000000000000000000000000000000000107").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	v8 = Vec::from_hex("07a526c63d3e25a256a007699f5447e32ae456b50e").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.a = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("03f7061798eb99e238fd6f1bf95b48feeb4854252b").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("02f9f87b7c574d0bdecf8a22e6524775f98cdebdcb").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("05b935590c155e17ea48eb3ff3718b893df59a05d0").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = BnGf2m::new_from_bigint(&p);
	bngrp.generator.z = BnGf2m::one();

	v8 = Vec::from_hex("03fffffffffffffffffffe1aee140f110aff961309").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = &ov + &ov;
	bngrp.curvename = C2PNB163v3_NAME.to_string();

	retv.insert(C2PNB163v3_NAME.to_string(),bngrp.clone());

	/*c2pnb176v1*/
	v8 = Vec::from_hex("0100000000000000000000000000000000080000000007").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	v8 = Vec::from_hex("00E4E6DB2995065C407D9D39B8D0967B96704BA8E9C90B").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.a = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("005DDA470ABE6414DE8EC133AE28E9BBD7FCEC0AE0FFF2").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("008D16C2866798B600F9F08BB4A8E860F3298CE04A5798").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("006FA4539C2DADDDD6BAB5167D61B436E1D92BB16A562C").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = BnGf2m::new_from_bigint(&p);
	bngrp.generator.z = BnGf2m::one();

	v8 = Vec::from_hex("0000010092537397ECA4F6145799D62B0A19CE06FE26AD").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	v8 = Vec::from_hex("ff6e").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.cofactor = p.clone();
	bngrp.curvename = C2PNB176v1_NAME.to_string();

	retv.insert(C2PNB176v1_NAME.to_string(),bngrp.clone());

	/*c2tnb191v1*/
	v8 = Vec::from_hex("800000000000000000000000000000000000000000000201").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	v8 = Vec::from_hex("2866537b676752636a68f56554e12640276b649ef7526267").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.a = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("2e45ef571f00786f67b0081b9495a3d95462f5de0aa185ec").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("36b3daf8a23206f9c4f299d7b21a9c369137f2c84ae1aa0d").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("765be73433b3f95e332932e70ea245ca2418ea0ef98018fb").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = BnGf2m::new_from_bigint(&p);
	bngrp.generator.z = BnGf2m::one();

	v8 = Vec::from_hex("40000000000000000000000004a20e90c39067c893bbb9a5").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	v8 = Vec::from_hex("02").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.cofactor = p.clone();
	bngrp.curvename = C2TNB191v1_NAME.to_string();

	retv.insert(C2TNB191v1_NAME.to_string(),bngrp.clone());

	/*c2tnb191v2*/
	v8 = Vec::from_hex("800000000000000000000000000000000000000000000201").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	v8 = Vec::from_hex("401028774d7777c7b7666d1366ea432071274f89ff01e718").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.a = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("0620048d28bcbd03b6249c99182b7c8cd19700c362c46a01").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("3809b2b7cc1b28cc5a87926aad83fd28789e81e2c9e3bf10").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("17434386626d14f3dbf01760d9213a3e1cf37aec437d668a").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = BnGf2m::new_from_bigint(&p);
	bngrp.generator.z = BnGf2m::one();

	v8 = Vec::from_hex("20000000000000000000000050508cb89f652824e06b8173").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	v8 = Vec::from_hex("04").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.cofactor = p.clone();
	bngrp.curvename = C2TNB191v2_NAME.to_string();

	retv.insert(C2TNB191v2_NAME.to_string(),bngrp.clone());

	/*c2tnb191v3*/
	v8 = Vec::from_hex("800000000000000000000000000000000000000000000201").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	v8 = Vec::from_hex("6c01074756099122221056911c77d77e77a777e7e7e77fcb").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.a = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("71fe1af926cf847989efef8db459f66394d90f32ad3f15e8").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("375d4ce24fde434489de8746e71786015009e66e38a926dd").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("545a39176196575d985999366e6ad34ce0a77cd7127b06be").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = BnGf2m::new_from_bigint(&p);
	bngrp.generator.z = BnGf2m::one();

	v8 = Vec::from_hex("155555555555555555555555610c0b196812bfb6288a3ea3").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	v8 = Vec::from_hex("06").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.cofactor = p.clone();
	bngrp.curvename = C2TNB191v3_NAME.to_string();

	retv.insert(C2TNB191v3_NAME.to_string(),bngrp.clone());

	/*c2pnb208w1*/
	v8 = Vec::from_hex("010000000000000000000000000000000800000000000000000007").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	v8 = Vec::from_hex("000000000000000000000000000000000000000000000000000000").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.a = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("00C8619ED45A62E6212E1160349E2BFA844439FAFC2A3FD1638F9E").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("0089FDFBE4ABE193DF9559ECF07AC0CE78554E2784EB8C1ED1A57A").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("000F55B51A06E78E9AC38A035FF520D8B01781BEB1A6BB08617DE3").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = BnGf2m::new_from_bigint(&p);
	bngrp.generator.z = BnGf2m::one();

	v8 = Vec::from_hex("00000101BAF95C9723C57B6C21DA2EFF2D5ED588BDD5717E212F9D").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	v8 = Vec::from_hex("FE48").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.cofactor = p.clone();
	bngrp.curvename = C2PNB208w1_NAME.to_string();

	retv.insert(C2PNB208w1_NAME.to_string(),bngrp.clone());


	/*c2pnb208w1*/
	v8 = Vec::from_hex("010000000000000000000000000000000800000000000000000007").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	v8 = Vec::from_hex("000000000000000000000000000000000000000000000000000000").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.a = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("00C8619ED45A62E6212E1160349E2BFA844439FAFC2A3FD1638F9E").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("0089FDFBE4ABE193DF9559ECF07AC0CE78554E2784EB8C1ED1A57A").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("000F55B51A06E78E9AC38A035FF520D8B01781BEB1A6BB08617DE3").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = BnGf2m::new_from_bigint(&p);
	bngrp.generator.z = BnGf2m::one();

	v8 = Vec::from_hex("00000101BAF95C9723C57B6C21DA2EFF2D5ED588BDD5717E212F9D").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	v8 = Vec::from_hex("FE48").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.cofactor = p.clone();
	bngrp.curvename = C2PNB208w1_NAME.to_string();

	retv.insert(C2PNB208w1_NAME.to_string(),bngrp.clone());

	/*c2tnb239v1*/
	v8 = Vec::from_hex("800000000000000000000000000000000000000000000000001000000001").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	v8 = Vec::from_hex("32010857077c5431123a46b808906756f543423e8d27877578125778ac76").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.a = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("790408f2eedaf392b012edefb3392f30f4327c0ca3f31fc383c422aa8c16").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("57927098fa932e7c0a96d3fd5b706ef7e5f5c156e16b7e7c86038552e91d").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("61d8ee5077c33fecf6f1a16b268de469c3c7744ea9a971649fc7a9616305").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = BnGf2m::new_from_bigint(&p);
	bngrp.generator.z = BnGf2m::one();

	v8 = Vec::from_hex("2000000000000000000000000000000f4d42ffe1492a4993f1cad666e447").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	v8 = Vec::from_hex("04").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.cofactor = p.clone();
	bngrp.curvename = C2TNB239v1_NAME.to_string();

	retv.insert(C2TNB239v1_NAME.to_string(),bngrp.clone());

	/*c2tnb239v2*/
	v8 = Vec::from_hex("800000000000000000000000000000000000000000000000001000000001").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	v8 = Vec::from_hex("4230017757a767fae42398569b746325d45313af0766266479b75654e65f").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.a = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("5037ea654196cff0cd82b2c14a2fcf2e3ff8775285b545722f03eacdb74b").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("28f9d04e900069c8dc47a08534fe76d2b900b7d7ef31f5709f200c4ca205").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("5667334c45aff3b5a03bad9dd75e2c71a99362567d5453f7fa6e227ec833").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = BnGf2m::new_from_bigint(&p);
	bngrp.generator.z = BnGf2m::one();

	v8 = Vec::from_hex("1555555555555555555555555555553c6f2885259c31e3fcdf154624522d").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	v8 = Vec::from_hex("06").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.cofactor = p.clone();
	bngrp.curvename = C2TNB239v2_NAME.to_string();

	retv.insert(C2TNB239v2_NAME.to_string(),bngrp.clone());

	/*c2tnb239v3*/
	v8 = Vec::from_hex("800000000000000000000000000000000000000000000000001000000001").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	v8 = Vec::from_hex("01238774666a67766d6676f778e676b66999176666e687666d8766c66a9f").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.a = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("6a941977ba9f6a435199acfc51067ed587f519c5ecb541b8e44111de1d40").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("70f6e9d04d289c4e89913ce3530bfde903977d42b146d539bf1bde4e9c92").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("2e5a0eaf6e5e1305b9004dce5c0ed7fe59a35608f33837c816d80b79f461").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = BnGf2m::new_from_bigint(&p);
	bngrp.generator.z = BnGf2m::one();

	v8 = Vec::from_hex("0cccccccccccccccccccccccccccccac4912d2d9df903ef9888b8a0e4cff").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	v8 = Vec::from_hex("0a").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.cofactor = p.clone();
	bngrp.curvename = C2TNB239v3_NAME.to_string();

	retv.insert(C2TNB239v3_NAME.to_string(),bngrp.clone());

	/*c2pnb272w1*/
	v8 = Vec::from_hex("010000000000000000000000000000000000000000000000000000010000000000000b").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	v8 = Vec::from_hex("91a091f03b5fba4ab2ccf49c4edd220fb028712d42be752b2c40094dbacdb586fb20").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.a = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("7167efc92bb2e3ce7c8aaaff34e12a9c557003d7c73a6faf003f99f6cc8482e540f7").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("6108babb2ceebcf787058a056cbe0cfe622d7723a289e08a07ae13ef0d10d171dd8d").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("10c7695716851eef6ba7f6872e6142fbd241b830ff5efcaceccab05e02005dde9d23").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = BnGf2m::new_from_bigint(&p);
	bngrp.generator.z = BnGf2m::one();

	v8 = Vec::from_hex("0100faf51354e0e39e4892df6e319c72c8161603fa45aa7b998a167b8f1e629521").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	v8 = Vec::from_hex("ff06").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.cofactor = p.clone();
	bngrp.curvename = C2PNB272w1_NAME.to_string();

	retv.insert(C2PNB272w1_NAME.to_string(),bngrp.clone());


	/*c2pnb304w1*/
	v8 = Vec::from_hex("010000000000000000000000000000000000000000000000000000000000000000000000000807").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	v8 = Vec::from_hex("fd0d693149a118f651e6dce6802085377e5f882d1b510b44160074c1288078365a0396c8e681").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.a = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("bddb97e555a50a908e43b01c798ea5daa6788f1ea2794efcf57166b8c14039601e55827340be").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("197b07845e9be2d96adb0f5f3c7f2cffbd7a3eb8b6fec35c7fd67f26ddf6285a644f740a2614").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("e19fbeb76e0da171517ecf401b50289bf014103288527a9b416a105e80260b549fdc1b92c03b").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = BnGf2m::new_from_bigint(&p);
	bngrp.generator.z = BnGf2m::one();

	v8 = Vec::from_hex("0101d556572aabac800101d556572aabac8001022d5c91dd173f8fb561da6899164443051d").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	v8 = Vec::from_hex("fe2e").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.cofactor = p.clone();
	bngrp.curvename = C2PNB304w1_NAME.to_string();

	retv.insert(C2PNB304w1_NAME.to_string(),bngrp.clone());


	/*c2tnb359v1*/
	v8 = Vec::from_hex("800000000000000000000000000000000000000000000000000000000000000000000000100000000000000001").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	v8 = Vec::from_hex("5667676a654b20754f356ea92017d946567c46675556f19556a04616b567d223a5e05656fb549016a96656a557").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.a = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("2472e2d0197c49363f1fe7f5b6db075d52b6947d135d8ca445805d39bc345626089687742b6329e70680231988").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("3c258ef3047767e7ede0f1fdaa79daee3841366a132e163aced4ed2401df9c6bdcde98e8e707c07a2239b1b097").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("53d7e08529547048121e9c95f3791dd804963948f34fae7bf44ea82365dc7868fe57e4ae2de211305a407104bd").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = BnGf2m::new_from_bigint(&p);
	bngrp.generator.z = BnGf2m::one();

	v8 = Vec::from_hex("01af286bca1af286bca1af286bca1af286bca1af286bc9fb8f6b85c556892c20a7eb964fe7719e74f490758d3b").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	v8 = Vec::from_hex("4c").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.cofactor = p.clone();
	bngrp.curvename = C2TNB359v1_NAME.to_string();

	retv.insert(C2TNB359v1_NAME.to_string(),bngrp.clone());


	/*c2pnb368w1*/
	v8 = Vec::from_hex("0100000000000000000000000000000000000000000000000000000000000000000000002000000000000000000007").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	v8 = Vec::from_hex("e0d2ee25095206f5e2a4f9ed229f1f256e79a0e2b455970d8d0d865bd94778c576d62f0ab7519ccd2a1a906ae30d").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.a = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("fc1217d4320a90452c760a58edcd30c8dd069b3c34453837a34ed50cb54917e1c2112d84d164f444f8f74786046a").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("1085e2755381dccce3c1557afa10c2f0c0c2825646c5b34a394cbcfa8bc16b22e7e789e927be216f02e1fb136a5f").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("7b3eb1bddcba62d5d8b2059b525797fc73822c59059c623a45ff3843cee8f87cd1855adaa81e2a0750b80fda2310").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = BnGf2m::new_from_bigint(&p);
	bngrp.generator.z = BnGf2m::one();

	v8 = Vec::from_hex("010090512da9af72b08349d98a5dd4c7b0532eca51ce03e2d10f3b7ac579bd87e909ae40a6f131e9cfce5bd967").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	v8 = Vec::from_hex("ff70").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.cofactor = p.clone();
	bngrp.curvename = C2PNB368w1_NAME.to_string();

	retv.insert(C2PNB368w1_NAME.to_string(),bngrp.clone());

	/*c2tnb431r1*/
	v8 = Vec::from_hex("800000000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000001").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	v8 = Vec::from_hex("1a827ef00dd6fc0e234caf046c6a5d8a85395b236cc4ad2cf32a0cadbdc9ddf620b0eb9906d0957f6c6feacd615468df104de296cd8f").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.a = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("10d9b4a3d9047d8b154359abfb1b7f5485b04ceb868237ddc9deda982a679a5a919b626d4e50a8dd731b107a9962381fb5d807bf2618").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("120fc05d3c67a99de161d2f4092622feca701be4f50f4758714e8a87bbf2a658ef8c21e7c5efe965361f6c2999c0c247b0dbd70ce6b7").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("20d0af8903a96f8d5fa2c255745d3c451b302c9346d9b7e485e7bce41f6b591f3e8f6addcbb0bc4c2f947a7de1a89b625d6a598b3760").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = BnGf2m::new_from_bigint(&p);
	bngrp.generator.z = BnGf2m::one();

	v8 = Vec::from_hex("0340340340340340340340340340340340340340340340340340340323c313fab50589703b5ec68d3587fec60d161cc149c1ad4a91").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	v8 = Vec::from_hex("2760").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.cofactor = p.clone();
	bngrp.curvename = C2TNB431r1_NAME.to_string();

	retv.insert(C2TNB431r1_NAME.to_string(),bngrp.clone());


	/*wap-wsg-idm-ecid-wtls1*/
	v8 = Vec::from_hex("020000000000000000000000000201").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	v8 = Vec::from_hex("01").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.a = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("01").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("01667979a40ba497e5d5c270780617").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = BnGf2m::new_from_bigint(&p);
	v8 = Vec::from_hex("00f44b4af1ecc2630e08785cebcc15").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = BnGf2m::new_from_bigint(&p);
	bngrp.generator.z = BnGf2m::one();

	v8 = Vec::from_hex("00fffffffffffffffdbf91af6dea73").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	v8 = Vec::from_hex("02").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.cofactor = p.clone();
	bngrp.curvename = WTLS1_NAME.to_string();

	retv.insert(WTLS1_NAME.to_string(),bngrp.clone());


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
	retv.insert(SECP521r1_NAME.to_string(),bngrp.clone());


	/*prime192v2*/
	v8 = Vec::from_hex("fffffffffffffffffffffffffffffffeffffffffffffffff").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	montv = MontNum::new(&bngrp.p).unwrap();
	tmpp = p.clone();
	v8 = Vec::from_hex("fffffffffffffffffffffffffffffffefffffffffffffffc").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	tmpa = p.clone();
	bngrp.a = montv.mont_to(&p);
	v8 = Vec::from_hex("cc22d6dfb95c6b25e49c0d6364a4e5980c393aa21668d953").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = montv.mont_to(&p);
	v8 = Vec::from_hex("eea2bae7e1497842f2de7769cfe9c989c072ad696f48034a").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = montv.mont_to(&p);
	v8 = Vec::from_hex("6574d11d69b6ec7a672bb82a083df2f2b0847de970b2de15").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = montv.mont_to(&p);
	bngrp.generator.z = montv.mont_to(&ov);

	v8 = Vec::from_hex("fffffffffffffffffffffffe5fb1a724dc80418648d8dd31").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = ov.clone();
	bngrp.curvename = PRIME192v2_NAME.to_string();

	//ecsimple_log_trace!("tmpp 0x{:X} tmpa 0x{:X}",tmpp,tmpa);
	if tmpp == (tmpa.clone() + ov.clone() + ov.clone() + ov.clone()) {
		bngrp.is_minus3 = true;
		//ecsimple_log_trace!("{} is_minus3 true",PRIME192v2_NAME);
	} else {
		bngrp.is_minus3 = false;
		//ecsimple_log_trace!("{} is_minus3 false",PRIME192v2_NAME);
	}
	retv.insert(PRIME192v2_NAME.to_string(),bngrp.clone());


	/*prime192v3*/
	v8 = Vec::from_hex("fffffffffffffffffffffffffffffffeffffffffffffffff").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	montv = MontNum::new(&bngrp.p).unwrap();
	tmpp = p.clone();
	v8 = Vec::from_hex("fffffffffffffffffffffffffffffffefffffffffffffffc").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	tmpa = p.clone();
	bngrp.a = montv.mont_to(&p);
	v8 = Vec::from_hex("22123dc2395a05caa7423daeccc94760a7d462256bd56916").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = montv.mont_to(&p);
	v8 = Vec::from_hex("7d29778100c65a1da1783716588dce2b8b4aee8e228f1896").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = montv.mont_to(&p);
	v8 = Vec::from_hex("38a90f22637337334b49dcb66a6dc8f9978aca7648a943b0").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = montv.mont_to(&p);
	bngrp.generator.z = montv.mont_to(&ov);

	v8 = Vec::from_hex("ffffffffffffffffffffffff7a62d031c83f4294f640ec13").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = ov.clone();
	bngrp.curvename = PRIME192v3_NAME.to_string();

	//ecsimple_log_trace!("tmpp 0x{:X} tmpa 0x{:X}",tmpp,tmpa);
	if tmpp == (tmpa.clone() + ov.clone() + ov.clone() + ov.clone()) {
		bngrp.is_minus3 = true;
		//ecsimple_log_trace!("{} is_minus3 true",PRIME192v3_NAME);
	} else {
		bngrp.is_minus3 = false;
		//ecsimple_log_trace!("{} is_minus3 false",PRIME192v3_NAME);
	}
	retv.insert(PRIME192v3_NAME.to_string(),bngrp.clone());

	/*prime239v1*/
	v8 = Vec::from_hex("7fffffffffffffffffffffff7fffffffffff8000000000007fffffffffff").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	montv = MontNum::new(&bngrp.p).unwrap();
	tmpp = p.clone();
	v8 = Vec::from_hex("7fffffffffffffffffffffff7fffffffffff8000000000007ffffffffffc").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	tmpa = p.clone();
	bngrp.a = montv.mont_to(&p);
	v8 = Vec::from_hex("6b016c3bdcf18941d0d654921475ca71a9db2fb27d1d37796185c2942c0a").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = montv.mont_to(&p);
	v8 = Vec::from_hex("0ffa963cdca8816ccc33b8642bedf905c3d358573d3f27fbbd3b3cb9aaaf").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = montv.mont_to(&p);
	v8 = Vec::from_hex("7debe8e4e90a5dae6e4054ca530ba04654b36818ce226b39fccb7b02f1ae").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = montv.mont_to(&p);
	bngrp.generator.z = montv.mont_to(&ov);

	v8 = Vec::from_hex("7fffffffffffffffffffffff7fffff9e5e9a9f5d9071fbd1522688909d0b").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = ov.clone();
	bngrp.curvename = PRIME239v1_NAME.to_string();

	//ecsimple_log_trace!("tmpp 0x{:X} tmpa 0x{:X}",tmpp,tmpa);
	if tmpp == (tmpa.clone() + ov.clone() + ov.clone() + ov.clone()) {
		bngrp.is_minus3 = true;
		//ecsimple_log_trace!("{} is_minus3 true",PRIME239v1_NAME);
	} else {
		bngrp.is_minus3 = false;
		//ecsimple_log_trace!("{} is_minus3 false",PRIME239v1_NAME);
	}
	retv.insert(PRIME239v1_NAME.to_string(),bngrp.clone());

	/*prime239v2*/
	v8 = Vec::from_hex("7fffffffffffffffffffffff7fffffffffff8000000000007fffffffffff").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	montv = MontNum::new(&bngrp.p).unwrap();
	tmpp = p.clone();
	v8 = Vec::from_hex("7fffffffffffffffffffffff7fffffffffff8000000000007ffffffffffc").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	tmpa = p.clone();
	bngrp.a = montv.mont_to(&p);
	v8 = Vec::from_hex("617fab6832576cbbfed50d99f0249c3fee58b94ba0038c7ae84c8c832f2c").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = montv.mont_to(&p);
	v8 = Vec::from_hex("38af09d98727705120c921bb5e9e26296a3cdcf2f35757a0eafd87b830e7").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = montv.mont_to(&p);
	v8 = Vec::from_hex("5b0125e4dbea0ec7206da0fc01d9b081329fb555de6ef460237dff8be4ba").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = montv.mont_to(&p);
	bngrp.generator.z = montv.mont_to(&ov);

	v8 = Vec::from_hex("7fffffffffffffffffffffff800000cfa7e8594377d414c03821bc582063").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = ov.clone();
	bngrp.curvename = PRIME239v2_NAME.to_string();

	//ecsimple_log_trace!("tmpp 0x{:X} tmpa 0x{:X}",tmpp,tmpa);
	if tmpp == (tmpa.clone() + ov.clone() + ov.clone() + ov.clone()) {
		bngrp.is_minus3 = true;
		//ecsimple_log_trace!("{} is_minus3 true",PRIME239v2_NAME);
	} else {
		bngrp.is_minus3 = false;
		//ecsimple_log_trace!("{} is_minus3 false",PRIME239v2_NAME);
	}
	retv.insert(PRIME239v2_NAME.to_string(),bngrp.clone());

	/*prime239v3*/
	v8 = Vec::from_hex("7fffffffffffffffffffffff7fffffffffff8000000000007fffffffffff").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	montv = MontNum::new(&bngrp.p).unwrap();
	tmpp = p.clone();
	v8 = Vec::from_hex("7fffffffffffffffffffffff7fffffffffff8000000000007ffffffffffc").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	tmpa = p.clone();
	bngrp.a = montv.mont_to(&p);
	v8 = Vec::from_hex("255705fa2a306654b1f4cb03d6a750a30c250102d4988717d9ba15ab6d3e").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = montv.mont_to(&p);
	v8 = Vec::from_hex("6768ae8e18bb92cfcf005c949aa2c6d94853d0e660bbf854b1c9505fe95a").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = montv.mont_to(&p);
	v8 = Vec::from_hex("1607e6898f390c06bc1d552bad226f3b6fcfe48b6e818499af18e3ed6cf3").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = montv.mont_to(&p);
	bngrp.generator.z = montv.mont_to(&ov);

	v8 = Vec::from_hex("7fffffffffffffffffffffff7fffff975deb41b3a6057c3c432146526551").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = ov.clone();
	bngrp.curvename = PRIME239v3_NAME.to_string();

	//ecsimple_log_trace!("tmpp 0x{:X} tmpa 0x{:X}",tmpp,tmpa);
	if tmpp == (tmpa.clone() + ov.clone() + ov.clone() + ov.clone()) {
		bngrp.is_minus3 = true;
		//ecsimple_log_trace!("{} is_minus3 true",PRIME239v3_NAME);
	} else {
		bngrp.is_minus3 = false;
		//ecsimple_log_trace!("{} is_minus3 false",PRIME239v3_NAME);
	}
	retv.insert(PRIME239v3_NAME.to_string(),bngrp.clone());


	/*prime256v1*/
	v8 = Vec::from_hex("ffffffff00000001000000000000000000000000ffffffffffffffffffffffff").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	montv = MontNum::new(&bngrp.p).unwrap();
	tmpp = p.clone();
	v8 = Vec::from_hex("ffffffff00000001000000000000000000000000fffffffffffffffffffffffc").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	tmpa = p.clone();
	bngrp.a = montv.mont_to(&p);
	v8 = Vec::from_hex("5ac635d8aa3a93e7b3ebbd55769886bc651d06b0cc53b0f63bce3c3e27d2604b").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = montv.mont_to(&p);
	v8 = Vec::from_hex("6b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = montv.mont_to(&p);
	v8 = Vec::from_hex("4fe342e2fe1a7f9b8ee7eb4a7c0f9e162bce33576b315ececbb6406837bf51f5").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = montv.mont_to(&p);
	bngrp.generator.z = montv.mont_to(&ov);

	v8 = Vec::from_hex("ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = ov.clone();
	bngrp.curvename = PRIME256v1_NAME.to_string();

	//ecsimple_log_trace!("tmpp 0x{:X} tmpa 0x{:X}",tmpp,tmpa);
	if tmpp == (tmpa.clone() + ov.clone() + ov.clone() + ov.clone()) {
		bngrp.is_minus3 = true;
		//ecsimple_log_trace!("{} is_minus3 true",PRIME256v1_NAME);
	} else {
		bngrp.is_minus3 = false;
		//ecsimple_log_trace!("{} is_minus3 false",PRIME256v1_NAME);
	}
	retv.insert(PRIME256v1_NAME.to_string(),bngrp.clone());


	/*secp112r2*/
	v8 = Vec::from_hex("db7c2abf62e35e668076bead208b").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	montv = MontNum::new(&bngrp.p).unwrap();
	tmpp = p.clone();
	v8 = Vec::from_hex("6127c24c05f38a0aaaf65c0ef02c").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	tmpa = p.clone();
	bngrp.a = montv.mont_to(&p);
	v8 = Vec::from_hex("51def1815db5ed74fcc34c85d709").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = montv.mont_to(&p);
	v8 = Vec::from_hex("4ba30ab5e892b4e1649dd0928643").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = montv.mont_to(&p);
	v8 = Vec::from_hex("adcd46f5882e3747def36e956e97").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = montv.mont_to(&p);
	bngrp.generator.z = montv.mont_to(&ov);

	v8 = Vec::from_hex("36df0aafd8b8d7597ca10520d04b").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = ov.clone() + ov.clone() + ov.clone() + ov.clone();
	bngrp.curvename = SECP112r2_NAME.to_string();

	//ecsimple_log_trace!("tmpp 0x{:X} tmpa 0x{:X}",tmpp,tmpa);
	if tmpp == (tmpa.clone() + ov.clone() + ov.clone() + ov.clone()) {
		bngrp.is_minus3 = true;
		//ecsimple_log_trace!("{} is_minus3 true",SECP112r2_NAME);
	} else {
		bngrp.is_minus3 = false;
		//ecsimple_log_trace!("{} is_minus3 false",SECP112r2_NAME);
	}
	retv.insert(SECP112r2_NAME.to_string(),bngrp.clone());


	/*secp128r1*/
	v8 = Vec::from_hex("fffffffdffffffffffffffffffffffff").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	montv = MontNum::new(&bngrp.p).unwrap();
	tmpp = p.clone();
	v8 = Vec::from_hex("fffffffdfffffffffffffffffffffffc").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	tmpa = p.clone();
	bngrp.a = montv.mont_to(&p);
	v8 = Vec::from_hex("e87579c11079f43dd824993c2cee5ed3").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = montv.mont_to(&p);
	v8 = Vec::from_hex("161ff7528b899b2d0c28607ca52c5b86").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = montv.mont_to(&p);
	v8 = Vec::from_hex("cf5ac8395bafeb13c02da292dded7a83").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = montv.mont_to(&p);
	bngrp.generator.z = montv.mont_to(&ov);

	v8 = Vec::from_hex("fffffffe0000000075a30d1b9038a115").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = ov.clone();
	bngrp.curvename = SECP128r1_NAME.to_string();

	//ecsimple_log_trace!("tmpp 0x{:X} tmpa 0x{:X}",tmpp,tmpa);
	if tmpp == (tmpa.clone() + ov.clone() + ov.clone() + ov.clone()) {
		bngrp.is_minus3 = true;
		//ecsimple_log_trace!("{} is_minus3 true",SECP128r1_NAME);
	} else {
		bngrp.is_minus3 = false;
		//ecsimple_log_trace!("{} is_minus3 false",SECP128r1_NAME);
	}
	retv.insert(SECP128r1_NAME.to_string(),bngrp.clone());

	/*secp128r2*/
	v8 = Vec::from_hex("fffffffdffffffffffffffffffffffff").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	montv = MontNum::new(&bngrp.p).unwrap();
	tmpp = p.clone();
	v8 = Vec::from_hex("d6031998d1b3bbfebf59cc9bbff9aee1").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	tmpa = p.clone();
	bngrp.a = montv.mont_to(&p);
	v8 = Vec::from_hex("5eeefca380d02919dc2c6558bb6d8a5d").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = montv.mont_to(&p);
	v8 = Vec::from_hex("7b6aa5d85e572983e6fb32a7cdebc140").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = montv.mont_to(&p);
	v8 = Vec::from_hex("27b6916a894d3aee7106fe805fc34b44").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = montv.mont_to(&p);
	bngrp.generator.z = montv.mont_to(&ov);

	v8 = Vec::from_hex("3fffffff7fffffffbe0024720613b5a3").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = ov.clone() + ov.clone() + ov.clone() + ov.clone();
	bngrp.curvename = SECP128r2_NAME.to_string();

	//ecsimple_log_trace!("tmpp 0x{:X} tmpa 0x{:X}",tmpp,tmpa);
	if tmpp == (tmpa.clone() + ov.clone() + ov.clone() + ov.clone()) {
		bngrp.is_minus3 = true;
		//ecsimple_log_trace!("{} is_minus3 true",SECP128r2_NAME);
	} else {
		bngrp.is_minus3 = false;
		//ecsimple_log_trace!("{} is_minus3 false",SECP128r2_NAME);
	}
	retv.insert(SECP128r2_NAME.to_string(),bngrp.clone());

	/*secp160k1*/
	v8 = Vec::from_hex("fffffffffffffffffffffffffffffffeffffac73").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	montv = MontNum::new(&bngrp.p).unwrap();
	tmpp = p.clone();
	v8 = Vec::from_hex("0000000000000000000000000000000000000000").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	tmpa = p.clone();
	bngrp.a = montv.mont_to(&p);
	v8 = Vec::from_hex("0000000000000000000000000000000000000007").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = montv.mont_to(&p);
	v8 = Vec::from_hex("3b4c382ce37aa192a4019e763036f4f5dd4d7ebb").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = montv.mont_to(&p);
	v8 = Vec::from_hex("938cf935318fdced6bc28286531733c3f03c4fee").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = montv.mont_to(&p);
	bngrp.generator.z = montv.mont_to(&ov);

	v8 = Vec::from_hex("0100000000000000000001b8fa16dfab9aca16b6b3").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = ov.clone();
	bngrp.curvename = SECP160k1_NAME.to_string();

	//ecsimple_log_trace!("tmpp 0x{:X} tmpa 0x{:X}",tmpp,tmpa);
	if tmpp == (tmpa.clone() + ov.clone() + ov.clone() + ov.clone()) {
		bngrp.is_minus3 = true;
		//ecsimple_log_trace!("{} is_minus3 true",SECP160k1_NAME);
	} else {
		bngrp.is_minus3 = false;
		//ecsimple_log_trace!("{} is_minus3 false",SECP160k1_NAME);
	}
	retv.insert(SECP160k1_NAME.to_string(),bngrp.clone());


	/*secp160r1*/
	v8 = Vec::from_hex("ffffffffffffffffffffffffffffffff7fffffff").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	montv = MontNum::new(&bngrp.p).unwrap();
	tmpp = p.clone();
	v8 = Vec::from_hex("ffffffffffffffffffffffffffffffff7ffffffc").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	tmpa = p.clone();
	bngrp.a = montv.mont_to(&p);
	v8 = Vec::from_hex("1c97befc54bd7a8b65acf89f81d4d4adc565fa45").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = montv.mont_to(&p);
	v8 = Vec::from_hex("4a96b5688ef573284664698968c38bb913cbfc82").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = montv.mont_to(&p);
	v8 = Vec::from_hex("23a628553168947d59dcc912042351377ac5fb32").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = montv.mont_to(&p);
	bngrp.generator.z = montv.mont_to(&ov);

	v8 = Vec::from_hex("0100000000000000000001f4c8f927aed3ca752257").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = ov.clone();
	bngrp.curvename = SECP160r1_NAME.to_string();

	//ecsimple_log_trace!("tmpp 0x{:X} tmpa 0x{:X}",tmpp,tmpa);
	if tmpp == (tmpa.clone() + ov.clone() + ov.clone() + ov.clone()) {
		bngrp.is_minus3 = true;
		//ecsimple_log_trace!("{} is_minus3 true",SECP160r1_NAME);
	} else {
		bngrp.is_minus3 = false;
		//ecsimple_log_trace!("{} is_minus3 false",SECP160r1_NAME);
	}
	retv.insert(SECP160r1_NAME.to_string(),bngrp.clone());


	/*secp160r2*/
	v8 = Vec::from_hex("fffffffffffffffffffffffffffffffeffffac73").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	montv = MontNum::new(&bngrp.p).unwrap();
	tmpp = p.clone();
	v8 = Vec::from_hex("fffffffffffffffffffffffffffffffeffffac70").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	tmpa = p.clone();
	bngrp.a = montv.mont_to(&p);
	v8 = Vec::from_hex("b4e134d3fb59eb8bab57274904664d5af50388ba").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = montv.mont_to(&p);
	v8 = Vec::from_hex("52dcb034293a117e1f4ff11b30f7199d3144ce6d").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = montv.mont_to(&p);
	v8 = Vec::from_hex("feaffef2e331f296e071fa0df9982cfea7d43f2e").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = montv.mont_to(&p);
	bngrp.generator.z = montv.mont_to(&ov);

	v8 = Vec::from_hex("0100000000000000000000351ee786a818f3a1a16b").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = ov.clone();
	bngrp.curvename = SECP160r2_NAME.to_string();

	//ecsimple_log_trace!("tmpp 0x{:X} tmpa 0x{:X}",tmpp,tmpa);
	if tmpp == (tmpa.clone() + ov.clone() + ov.clone() + ov.clone()) {
		bngrp.is_minus3 = true;
		//ecsimple_log_trace!("{} is_minus3 true",SECP160r2_NAME);
	} else {
		bngrp.is_minus3 = false;
		//ecsimple_log_trace!("{} is_minus3 false",SECP160r2_NAME);
	}
	retv.insert(SECP160r2_NAME.to_string(),bngrp.clone());


	/*secp192k1*/
	v8 = Vec::from_hex("fffffffffffffffffffffffffffffffffffffffeffffee37").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	montv = MontNum::new(&bngrp.p).unwrap();
	tmpp = p.clone();
	v8 = Vec::from_hex("000000000000000000000000000000000000000000000000").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	tmpa = p.clone();
	bngrp.a = montv.mont_to(&p);
	v8 = Vec::from_hex("000000000000000000000000000000000000000000000003").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = montv.mont_to(&p);
	v8 = Vec::from_hex("db4ff10ec057e9ae26b07d0280b7f4341da5d1b1eae06c7d").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = montv.mont_to(&p);
	v8 = Vec::from_hex("9b2f2f6d9c5628a7844163d015be86344082aa88d95e2f9d").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = montv.mont_to(&p);
	bngrp.generator.z = montv.mont_to(&ov);

	v8 = Vec::from_hex("fffffffffffffffffffffffe26f2fc170f69466a74defd8d").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = ov.clone();
	bngrp.curvename = SECP192k1_NAME.to_string();

	//ecsimple_log_trace!("tmpp 0x{:X} tmpa 0x{:X}",tmpp,tmpa);
	if tmpp == (tmpa.clone() + ov.clone() + ov.clone() + ov.clone()) {
		bngrp.is_minus3 = true;
		//ecsimple_log_trace!("{} is_minus3 true",SECP192k1_NAME);
	} else {
		bngrp.is_minus3 = false;
		//ecsimple_log_trace!("{} is_minus3 false",SECP192k1_NAME);
	}
	retv.insert(SECP192k1_NAME.to_string(),bngrp.clone());

	/*secp224k1*/
	v8 = Vec::from_hex("fffffffffffffffffffffffffffffffffffffffffffffffeffffe56d").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	montv = MontNum::new(&bngrp.p).unwrap();
	tmpp = p.clone();
	v8 = Vec::from_hex("00000000000000000000000000000000000000000000000000000000").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	tmpa = p.clone();
	bngrp.a = montv.mont_to(&p);
	v8 = Vec::from_hex("00000000000000000000000000000000000000000000000000000005").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = montv.mont_to(&p);
	v8 = Vec::from_hex("a1455b334df099df30fc28a169a467e9e47075a90f7e650eb6b7a45c").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = montv.mont_to(&p);
	v8 = Vec::from_hex("7e089fed7fba344282cafbd6f7e319f7c0b0bd59e2ca4bdb556d61a5").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = montv.mont_to(&p);
	bngrp.generator.z = montv.mont_to(&ov);

	v8 = Vec::from_hex("010000000000000000000000000001dce8d2ec6184caf0a971769fb1f7").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = ov.clone();
	bngrp.curvename = SECP224k1_NAME.to_string();

	//ecsimple_log_trace!("tmpp 0x{:X} tmpa 0x{:X}",tmpp,tmpa);
	if tmpp == (tmpa.clone() + ov.clone() + ov.clone() + ov.clone()) {
		bngrp.is_minus3 = true;
		//ecsimple_log_trace!("{} is_minus3 true",SECP224k1_NAME);
	} else {
		bngrp.is_minus3 = false;
		//ecsimple_log_trace!("{} is_minus3 false",SECP224k1_NAME);
	}
	retv.insert(SECP224k1_NAME.to_string(),bngrp.clone());


	/*secp256k1*/
	v8 = Vec::from_hex("fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	montv = MontNum::new(&bngrp.p).unwrap();
	tmpp = p.clone();
	v8 = Vec::from_hex("0000000000000000000000000000000000000000000000000000000000000000").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	tmpa = p.clone();
	bngrp.a = montv.mont_to(&p);
	v8 = Vec::from_hex("0000000000000000000000000000000000000000000000000000000000000007").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = montv.mont_to(&p);
	v8 = Vec::from_hex("79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = montv.mont_to(&p);
	v8 = Vec::from_hex("483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = montv.mont_to(&p);
	bngrp.generator.z = montv.mont_to(&ov);

	v8 = Vec::from_hex("fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = ov.clone();
	bngrp.curvename = SECP256k1_NAME.to_string();

	//ecsimple_log_trace!("tmpp 0x{:X} tmpa 0x{:X}",tmpp,tmpa);
	if tmpp == (tmpa.clone() + ov.clone() + ov.clone() + ov.clone()) {
		bngrp.is_minus3 = true;
		//ecsimple_log_trace!("{} is_minus3 true",SECP256k1_NAME);
	} else {
		bngrp.is_minus3 = false;
		//ecsimple_log_trace!("{} is_minus3 false",SECP256k1_NAME);
	}
	retv.insert(SECP256k1_NAME.to_string(),bngrp.clone());


	/*wap-wsg-idm-ecid-wtls8*/
	v8 = Vec::from_hex("fffffffffffffffffffffffffde7").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	montv = MontNum::new(&bngrp.p).unwrap();
	tmpp = p.clone();
	v8 = Vec::from_hex("00").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	tmpa = p.clone();
	bngrp.a = montv.mont_to(&p);
	v8 = Vec::from_hex("03").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = montv.mont_to(&p);
	v8 = Vec::from_hex("01").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = montv.mont_to(&p);
	v8 = Vec::from_hex("02").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = montv.mont_to(&p);
	bngrp.generator.z = montv.mont_to(&ov);

	v8 = Vec::from_hex("0100000000000001ecea551ad837e9").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = ov.clone();
	bngrp.curvename = WTLS8_NAME.to_string();

	//ecsimple_log_trace!("tmpp 0x{:X} tmpa 0x{:X}",tmpp,tmpa);
	if tmpp == (tmpa.clone() + ov.clone() + ov.clone() + ov.clone()) {
		bngrp.is_minus3 = true;
		//ecsimple_log_trace!("{} is_minus3 true",WTLS8_NAME);
	} else {
		bngrp.is_minus3 = false;
		//ecsimple_log_trace!("{} is_minus3 false",WTLS8_NAME);
	}
	retv.insert(WTLS8_NAME.to_string(),bngrp.clone());

	/*wap-wsg-idm-ecid-wtls9*/
	v8 = Vec::from_hex("fffffffffffffffffffffffffffffffffffc808f").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	montv = MontNum::new(&bngrp.p).unwrap();
	tmpp = p.clone();
	v8 = Vec::from_hex("00").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	tmpa = p.clone();
	bngrp.a = montv.mont_to(&p);
	v8 = Vec::from_hex("03").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = montv.mont_to(&p);
	v8 = Vec::from_hex("01").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = montv.mont_to(&p);
	v8 = Vec::from_hex("02").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = montv.mont_to(&p);
	bngrp.generator.z = montv.mont_to(&ov);

	v8 = Vec::from_hex("0100000000000000000001cdc98ae0e2de574abf33").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = ov.clone();
	bngrp.curvename = WTLS9_NAME.to_string();

	//ecsimple_log_trace!("tmpp 0x{:X} tmpa 0x{:X}",tmpp,tmpa);
	if tmpp == (tmpa.clone() + ov.clone() + ov.clone() + ov.clone()) {
		bngrp.is_minus3 = true;
		//ecsimple_log_trace!("{} is_minus3 true",WTLS9_NAME);
	} else {
		bngrp.is_minus3 = false;
		//ecsimple_log_trace!("{} is_minus3 false",WTLS9_NAME);
	}
	retv.insert(WTLS9_NAME.to_string(),bngrp.clone());


	/*wap-wsg-idm-ecid-wtls12*/
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
	bngrp.curvename = WTLS12_NAME.to_string();

	//ecsimple_log_trace!("tmpp 0x{:X} tmpa 0x{:X}",tmpp,tmpa);
	if tmpp == (tmpa.clone() + ov.clone() + ov.clone() + ov.clone()) {
		bngrp.is_minus3 = true;
		//ecsimple_log_trace!("{} is_minus3 true",WTLS12_NAME);
	} else {
		bngrp.is_minus3 = false;
		//ecsimple_log_trace!("{} is_minus3 false",WTLS12_NAME);
	}
	retv.insert(WTLS12_NAME.to_string(),bngrp.clone());


	/*brainpoolP160r1*/
	v8 = Vec::from_hex("e95e4a5f737059dc60dfc7ad95b3d8139515620f").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	montv = MontNum::new(&bngrp.p).unwrap();
	tmpp = p.clone();
	v8 = Vec::from_hex("340e7be2a280eb74e2be61bada745d97e8f7c300").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	tmpa = p.clone();
	bngrp.a = montv.mont_to(&p);
	v8 = Vec::from_hex("1e589a8595423412134faa2dbdec95c8d8675e58").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = montv.mont_to(&p);
	v8 = Vec::from_hex("bed5af16ea3f6a4f62938c4631eb5af7bdbcdbc3").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = montv.mont_to(&p);
	v8 = Vec::from_hex("1667cb477a1a8ec338f94741669c976316da6321").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = montv.mont_to(&p);
	bngrp.generator.z = montv.mont_to(&ov);

	v8 = Vec::from_hex("e95e4a5f737059dc60df5991d45029409e60fc09").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = ov.clone();
	bngrp.curvename = BRAINPOOLP160r1_NAME.to_string();

	//ecsimple_log_trace!("tmpp 0x{:X} tmpa 0x{:X}",tmpp,tmpa);
	if tmpp == (tmpa.clone() + ov.clone() + ov.clone() + ov.clone()) {
		bngrp.is_minus3 = true;
		//ecsimple_log_trace!("{} is_minus3 true",BRAINPOOLP160r1_NAME);
	} else {
		bngrp.is_minus3 = false;
		//ecsimple_log_trace!("{} is_minus3 false",BRAINPOOLP160r1_NAME);
	}
	retv.insert(BRAINPOOLP160r1_NAME.to_string(),bngrp.clone());


	/*brainpoolP160t1*/
	v8 = Vec::from_hex("e95e4a5f737059dc60dfc7ad95b3d8139515620f").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	montv = MontNum::new(&bngrp.p).unwrap();
	tmpp = p.clone();
	v8 = Vec::from_hex("e95e4a5f737059dc60dfc7ad95b3d8139515620c").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	tmpa = p.clone();
	bngrp.a = montv.mont_to(&p);
	v8 = Vec::from_hex("7a556b6dae535b7b51ed2c4d7daa7a0b5c55f380").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = montv.mont_to(&p);
	v8 = Vec::from_hex("b199b13b9b34efc1397e64baeb05acc265ff2378").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = montv.mont_to(&p);
	v8 = Vec::from_hex("add6718b7c7c1961f0991b842443772152c9e0ad").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = montv.mont_to(&p);
	bngrp.generator.z = montv.mont_to(&ov);

	v8 = Vec::from_hex("e95e4a5f737059dc60df5991d45029409e60fc09").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = ov.clone();
	bngrp.curvename = BRAINPOOLP160t1_NAME.to_string();

	//ecsimple_log_trace!("tmpp 0x{:X} tmpa 0x{:X}",tmpp,tmpa);
	if tmpp == (tmpa.clone() + ov.clone() + ov.clone() + ov.clone()) {
		bngrp.is_minus3 = true;
		//ecsimple_log_trace!("{} is_minus3 true",BRAINPOOLP160t1_NAME);
	} else {
		bngrp.is_minus3 = false;
		//ecsimple_log_trace!("{} is_minus3 false",BRAINPOOLP160t1_NAME);
	}
	retv.insert(BRAINPOOLP160t1_NAME.to_string(),bngrp.clone());

	/*brainpoolP192r1*/
	v8 = Vec::from_hex("c302f41d932a36cda7a3463093d18db78fce476de1a86297").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	montv = MontNum::new(&bngrp.p).unwrap();
	tmpp = p.clone();
	v8 = Vec::from_hex("6a91174076b1e0e19c39c031fe8685c1cae040e5c69a28ef").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	tmpa = p.clone();
	bngrp.a = montv.mont_to(&p);
	v8 = Vec::from_hex("469a28ef7c28cca3dc721d044f4496bcca7ef4146fbf25c9").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = montv.mont_to(&p);
	v8 = Vec::from_hex("c0a0647eaab6a48753b033c56cb0f0900a2f5c4853375fd6").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = montv.mont_to(&p);
	v8 = Vec::from_hex("14b690866abd5bb88b5f4828c1490002e6773fa2fa299b8f").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = montv.mont_to(&p);
	bngrp.generator.z = montv.mont_to(&ov);

	v8 = Vec::from_hex("c302f41d932a36cda7a3462f9e9e916b5be8f1029ac4acc1").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = ov.clone();
	bngrp.curvename = BRAINPOOLP192r1_NAME.to_string();

	//ecsimple_log_trace!("tmpp 0x{:X} tmpa 0x{:X}",tmpp,tmpa);
	if tmpp == (tmpa.clone() + ov.clone() + ov.clone() + ov.clone()) {
		bngrp.is_minus3 = true;
		//ecsimple_log_trace!("{} is_minus3 true",BRAINPOOLP192r1_NAME);
	} else {
		bngrp.is_minus3 = false;
		//ecsimple_log_trace!("{} is_minus3 false",BRAINPOOLP192r1_NAME);
	}
	retv.insert(BRAINPOOLP192r1_NAME.to_string(),bngrp.clone());

	/*brainpoolP192t1*/
	v8 = Vec::from_hex("c302f41d932a36cda7a3463093d18db78fce476de1a86297").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	montv = MontNum::new(&bngrp.p).unwrap();
	tmpp = p.clone();
	v8 = Vec::from_hex("c302f41d932a36cda7a3463093d18db78fce476de1a86294").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	tmpa = p.clone();
	bngrp.a = montv.mont_to(&p);
	v8 = Vec::from_hex("13d56ffaec78681e68f9deb43b35bec2fb68542e27897b79").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = montv.mont_to(&p);
	v8 = Vec::from_hex("3ae9e58c82f63c30282e1fe7bbf43fa72c446af6f4618129").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = montv.mont_to(&p);
	v8 = Vec::from_hex("097e2c5667c2223a902ab5ca449d0084b7e5b3de7ccc01c9").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = montv.mont_to(&p);
	bngrp.generator.z = montv.mont_to(&ov);

	v8 = Vec::from_hex("c302f41d932a36cda7a3462f9e9e916b5be8f1029ac4acc1").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = ov.clone();
	bngrp.curvename = BRAINPOOLP192t1_NAME.to_string();

	//ecsimple_log_trace!("tmpp 0x{:X} tmpa 0x{:X}",tmpp,tmpa);
	if tmpp == (tmpa.clone() + ov.clone() + ov.clone() + ov.clone()) {
		bngrp.is_minus3 = true;
		//ecsimple_log_trace!("{} is_minus3 true",BRAINPOOLP192t1_NAME);
	} else {
		bngrp.is_minus3 = false;
		//ecsimple_log_trace!("{} is_minus3 false",BRAINPOOLP192t1_NAME);
	}
	retv.insert(BRAINPOOLP192t1_NAME.to_string(),bngrp.clone());

	/*brainpoolP224r1*/
	v8 = Vec::from_hex("d7c134aa264366862a18302575d1d787b09f075797da89f57ec8c0ff").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	montv = MontNum::new(&bngrp.p).unwrap();
	tmpp = p.clone();
	v8 = Vec::from_hex("68a5e62ca9ce6c1c299803a6c1530b514e182ad8b0042a59cad29f43").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	tmpa = p.clone();
	bngrp.a = montv.mont_to(&p);
	v8 = Vec::from_hex("2580f63ccfe44138870713b1a92369e33e2135d266dbb372386c400b").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = montv.mont_to(&p);
	v8 = Vec::from_hex("0d9029ad2c7e5cf4340823b2a87dc68c9e4ce3174c1e6efdee12c07d").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = montv.mont_to(&p);
	v8 = Vec::from_hex("58aa56f772c0726f24c6b89e4ecdac24354b9e99caa3f6d3761402cd").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = montv.mont_to(&p);
	bngrp.generator.z = montv.mont_to(&ov);

	v8 = Vec::from_hex("d7c134aa264366862a18302575d0fb98d116bc4b6ddebca3a5a7939f").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = ov.clone();
	bngrp.curvename = BRAINPOOLP224r1_NAME.to_string();

	//ecsimple_log_trace!("tmpp 0x{:X} tmpa 0x{:X}",tmpp,tmpa);
	if tmpp == (tmpa.clone() + ov.clone() + ov.clone() + ov.clone()) {
		bngrp.is_minus3 = true;
		//ecsimple_log_trace!("{} is_minus3 true",BRAINPOOLP224r1_NAME);
	} else {
		bngrp.is_minus3 = false;
		//ecsimple_log_trace!("{} is_minus3 false",BRAINPOOLP224r1_NAME);
	}
	retv.insert(BRAINPOOLP224r1_NAME.to_string(),bngrp.clone());

	/*brainpoolP224t1*/
	v8 = Vec::from_hex("d7c134aa264366862a18302575d1d787b09f075797da89f57ec8c0ff").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	montv = MontNum::new(&bngrp.p).unwrap();
	tmpp = p.clone();
	v8 = Vec::from_hex("d7c134aa264366862a18302575d1d787b09f075797da89f57ec8c0fc").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	tmpa = p.clone();
	bngrp.a = montv.mont_to(&p);
	v8 = Vec::from_hex("4b337d934104cd7bef271bf60ced1ed20da14c08b3bb64f18a60888d").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = montv.mont_to(&p);
	v8 = Vec::from_hex("6ab1e344ce25ff3896424e7ffe14762ecb49f8928ac0c76029b4d580").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = montv.mont_to(&p);
	v8 = Vec::from_hex("0374e9f5143e568cd23f3f4d7c0d4b1e41c8cc0d1c6abd5f1a46db4c").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = montv.mont_to(&p);
	bngrp.generator.z = montv.mont_to(&ov);

	v8 = Vec::from_hex("d7c134aa264366862a18302575d0fb98d116bc4b6ddebca3a5a7939f").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = ov.clone();
	bngrp.curvename = BRAINPOOLP224t1_NAME.to_string();

	//ecsimple_log_trace!("tmpp 0x{:X} tmpa 0x{:X}",tmpp,tmpa);
	if tmpp == (tmpa.clone() + ov.clone() + ov.clone() + ov.clone()) {
		bngrp.is_minus3 = true;
		//ecsimple_log_trace!("{} is_minus3 true",BRAINPOOLP224t1_NAME);
	} else {
		bngrp.is_minus3 = false;
		//ecsimple_log_trace!("{} is_minus3 false",BRAINPOOLP224t1_NAME);
	}
	retv.insert(BRAINPOOLP224t1_NAME.to_string(),bngrp.clone());

	/*brainpoolP256r1*/
	v8 = Vec::from_hex("a9fb57dba1eea9bc3e660a909d838d726e3bf623d52620282013481d1f6e5377").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	montv = MontNum::new(&bngrp.p).unwrap();
	tmpp = p.clone();
	v8 = Vec::from_hex("7d5a0975fc2c3057eef67530417affe7fb8055c126dc5c6ce94a4b44f330b5d9").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	tmpa = p.clone();
	bngrp.a = montv.mont_to(&p);
	v8 = Vec::from_hex("26dc5c6ce94a4b44f330b5d9bbd77cbf958416295cf7e1ce6bccdc18ff8c07b6").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = montv.mont_to(&p);
	v8 = Vec::from_hex("8bd2aeb9cb7e57cb2c4b482ffc81b7afb9de27e1e3bd23c23a4453bd9ace3262").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = montv.mont_to(&p);
	v8 = Vec::from_hex("547ef835c3dac4fd97f8461a14611dc9c27745132ded8e545c1d54c72f046997").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = montv.mont_to(&p);
	bngrp.generator.z = montv.mont_to(&ov);

	v8 = Vec::from_hex("a9fb57dba1eea9bc3e660a909d838d718c397aa3b561a6f7901e0e82974856a7").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = ov.clone();
	bngrp.curvename = BRAINPOOLP256r1_NAME.to_string();

	//ecsimple_log_trace!("tmpp 0x{:X} tmpa 0x{:X}",tmpp,tmpa);
	if tmpp == (tmpa.clone() + ov.clone() + ov.clone() + ov.clone()) {
		bngrp.is_minus3 = true;
		//ecsimple_log_trace!("{} is_minus3 true",BRAINPOOLP256r1_NAME);
	} else {
		bngrp.is_minus3 = false;
		//ecsimple_log_trace!("{} is_minus3 false",BRAINPOOLP256r1_NAME);
	}
	retv.insert(BRAINPOOLP256r1_NAME.to_string(),bngrp.clone());

	/*brainpoolP256t1*/
	v8 = Vec::from_hex("a9fb57dba1eea9bc3e660a909d838d726e3bf623d52620282013481d1f6e5377").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	montv = MontNum::new(&bngrp.p).unwrap();
	tmpp = p.clone();
	v8 = Vec::from_hex("a9fb57dba1eea9bc3e660a909d838d726e3bf623d52620282013481d1f6e5374").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	tmpa = p.clone();
	bngrp.a = montv.mont_to(&p);
	v8 = Vec::from_hex("662c61c430d84ea4fe66a7733d0b76b7bf93ebc4af2f49256ae58101fee92b04").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = montv.mont_to(&p);
	v8 = Vec::from_hex("a3e8eb3cc1cfe7b7732213b23a656149afa142c47aafbc2b79a191562e1305f4").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = montv.mont_to(&p);
	v8 = Vec::from_hex("2d996c823439c56d7f7b22e14644417e69bcb6de39d027001dabe8f35b25c9be").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = montv.mont_to(&p);
	bngrp.generator.z = montv.mont_to(&ov);

	v8 = Vec::from_hex("a9fb57dba1eea9bc3e660a909d838d718c397aa3b561a6f7901e0e82974856a7").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = ov.clone();
	bngrp.curvename = BRAINPOOLP256t1_NAME.to_string();

	//ecsimple_log_trace!("tmpp 0x{:X} tmpa 0x{:X}",tmpp,tmpa);
	if tmpp == (tmpa.clone() + ov.clone() + ov.clone() + ov.clone()) {
		bngrp.is_minus3 = true;
		//ecsimple_log_trace!("{} is_minus3 true",BRAINPOOLP256t1_NAME);
	} else {
		bngrp.is_minus3 = false;
		//ecsimple_log_trace!("{} is_minus3 false",BRAINPOOLP256t1_NAME);
	}
	retv.insert(BRAINPOOLP256t1_NAME.to_string(),bngrp.clone());

	/*brainpoolP320r1*/
	v8 = Vec::from_hex("d35e472036bc4fb7e13c785ed201e065f98fcfa6f6f40def4f92b9ec7893ec28fcd412b1f1b32e27").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.p = p.clone();
	montv = MontNum::new(&bngrp.p).unwrap();
	tmpp = p.clone();
	v8 = Vec::from_hex("3ee30b568fbab0f883ccebd46d3f3bb8a2a73513f5eb79da66190eb085ffa9f492f375a97d860eb4").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	tmpa = p.clone();
	bngrp.a = montv.mont_to(&p);
	v8 = Vec::from_hex("520883949dfdbc42d3ad198640688a6fe13f41349554b49acc31dccd884539816f5eb4ac8fb1f1a6").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.b = montv.mont_to(&p);
	v8 = Vec::from_hex("43bd7e9afb53d8b85289bcc48ee5bfe6f20137d10a087eb6e7871e2a10a599c710af8d0d39e20611").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.x = montv.mont_to(&p);
	v8 = Vec::from_hex("14fdd05545ec1cc8ab4093247f77275e0743ffed117182eaa9c77877aaac6ac7d35245d1692e8ee1").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.generator.y = montv.mont_to(&p);
	bngrp.generator.z = montv.mont_to(&ov);

	v8 = Vec::from_hex("d35e472036bc4fb7e13c785ed201e065f98fcfa5b68f12a32d482ec7ee8658e98691555b44c59311").unwrap();
	p = BigInt::from_bytes_be(Sign::Plus,&v8);
	bngrp.order = p.clone();
	bngrp.cofactor = ov.clone();
	bngrp.curvename = BRAINPOOLP320r1_NAME.to_string();

	//ecsimple_log_trace!("tmpp 0x{:X} tmpa 0x{:X}",tmpp,tmpa);
	if tmpp == (tmpa.clone() + ov.clone() + ov.clone() + ov.clone()) {
		bngrp.is_minus3 = true;
		//ecsimple_log_trace!("{} is_minus3 true",BRAINPOOLP320r1_NAME);
	} else {
		bngrp.is_minus3 = false;
		//ecsimple_log_trace!("{} is_minus3 false",BRAINPOOLP320r1_NAME);
	}
	retv.insert(BRAINPOOLP320r1_NAME.to_string(),bngrp.clone());

	retv
}

fn craete_curve_group() -> HashMap<String,ECGroup> {
	let mut retv :HashMap<String,ECGroup> = HashMap::new();
	let bngrp :HashMap<String,ECGroupBnGf2m> = create_group_bn_curves();
	for (k,v) in bngrp {
		retv.insert(k,ECGroup::new_bn_group(&v));
	}
	let primegrp :HashMap<String,ECGroupPrime> = create_group_prime_curves();
	for (k,v) in primegrp {
		retv.insert(k,ECGroup::new_prime_group(&v));
	}
	retv
}


lazy_static ! {
	static ref ECC_CURVES : HashMap<String,ECGroup> = {
		craete_curve_group()
	};
}


pub fn get_curve_group(name :&str) -> Result<ECGroup,Box<dyn Error>> {
	match ECC_CURVES.get(name) {
		Some(pv) => {
			ecsimple_log_trace!("load [{}]",name);
			return Ok(pv.clone());
		},
		_ => {
			ecsimple_new_error!{ECGroupError,"can not find [{}]",name}
		}
	}
}