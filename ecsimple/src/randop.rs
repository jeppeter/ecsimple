

use rand;
use crate::fileop::*;
use crate::*;
use crate::logger::*;
use std::error::Error;
use num_bigint::{BigInt,Sign};
use num_traits::{zero};
//use rand_core::CryptoRng;
use rand_core::RngCore;
use std::env;
use lazy_static::lazy_static;
use std::sync::RwLock;


pub struct RandOps  {
	gencore : Option<rand::rngs::ThreadRng>,
	filerand : Option<RandFile>,
	begen : bool,
	pos :usize,
}

impl RandOps {
	pub fn new(fname :Option<String>) -> Result<Self,Box<dyn Error>> {
		let mut retv = RandOps {
			gencore : None,
			filerand : None,
			begen : true,
			pos : 0,
		};
		if fname.is_none() {
			retv.gencore = Some(rand::thread_rng());
		} else {
			retv.filerand = Some(RandFile::new(fname.as_ref().unwrap())?);
			retv.begen = false;
		}
		Ok(retv)
	}

	pub fn get_bytes(&mut self, num :usize) -> Result<Vec<u8>,Box<dyn Error>> {
		let mut buf :Vec<u8> = Vec::new();
		for _ in 0..num {
			buf.push(0x0);
		}
		if self.begen {
			self.gencore.as_mut().unwrap().try_fill_bytes(&mut buf)?;
		} else {
			self.filerand.as_mut().unwrap().try_fill_bytes(&mut buf)?;
		}
		//ecsimple_log_trace!("get pos [0x{:x}] size [0x{:x}]",self.pos,num);
		self.pos += num;
		Ok(buf)
	}
}

impl rand_core::CryptoRng  for RandOps {
}


impl rand_core::RngCore for RandOps {
	fn next_u32(&mut self) -> u32 {
		if self.begen {
			return self.gencore.as_mut().unwrap().next_u32();
		} else {
			return self.filerand.as_mut().unwrap().next_u32();
		}
	}

	fn next_u64(&mut self) -> u64 {
		if self.begen {
			return self.gencore.as_mut().unwrap().next_u64();
		}
		return self.filerand.as_mut().unwrap().next_u64();	
	}

	fn fill_bytes(&mut self, dest: &mut [u8]) {
		if self.begen {
			return self.gencore.as_mut().unwrap().fill_bytes(dest);
		}
		return self.filerand.as_mut().unwrap().fill_bytes(dest);	
	}

	fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(),rand_core::Error> {
		if self.begen {
			return self.gencore.as_mut().unwrap().try_fill_bytes(dest);
		}
		return self.filerand.as_mut().unwrap().try_fill_bytes(dest);	
	}
}

unsafe impl Send for RandOps {}
unsafe impl Sync for RandOps {}

fn create_randop() -> RwLock<RandOps> {
	let envname :String = "ECSIMPLE_RANDOP".to_string();
	let randop : RandOps;

	match env::var(&envname) {
		Ok(v) => {
			let randfile = Some(format!("{}",v));
			//ecsimple_log_trace!("file {}",v);
			randop = RandOps::new(randfile).unwrap();
		},
		Err(_e) => {
			//ecsimple_log_trace!("none file");
			randop = RandOps::new(None).unwrap();
		}
	}

	return RwLock::new(randop);
}

lazy_static ! {
	static ref EC_SIMPLE_RANDOP : RwLock<RandOps> = {
		create_randop()	
	};
}


pub (crate) fn ecsimple_rand_bits(bits :u64) -> Vec<u8> {
	let rnbytes : usize = ((bits+ 7) >> 3) as usize;
	let retv = EC_SIMPLE_RANDOP.write().unwrap().get_bytes(rnbytes).unwrap();
	//ecsimple_debug_buffer_trace!(retv.as_ptr(),retv.len(),"get value");
	return retv;
}

pub (crate) fn ecsimple_rand_range(buflen :i64, rangeval :&BigInt) -> BigInt {
	loop {
		let retv = EC_SIMPLE_RANDOP.write()	.unwrap().get_bytes(buflen as usize).unwrap();
		//ecsimple_debug_buffer_trace!(retv.as_ptr(),retv.len(),"get value");
		let mut bv = BigInt::from_bytes_be(Sign::Plus,&retv);
		ecsimple_log_trace!("random out 0x{:X}", bv);
		bv = bv % rangeval;
		if bv != zero() {
			return bv;	
		}		
	}
	
}