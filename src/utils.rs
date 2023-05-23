
use crate::*;
use std::error::Error;
use std::io::{Read};
use rand::RngCore;


ecsimple_error_class!{EcsimpleUtilsError}

pub struct RandFile {
	f : std::fs::File,
	fname :String,
}

impl RandFile {
	pub fn new(name :&str) -> Result<RandFile,Box<dyn Error>> {
		let ores = std::fs::File::open(name);
		if ores.is_err() {
			let e = ores.err().unwrap();
			ecsimple_new_error!{EcsimpleUtilsError,"open {} error {:?}", name,e}
		}
		let f = ores.unwrap();
		Ok(RandFile {
			f : f,
			fname : format!("{}",name),
		})
	}
}

impl rand_core::CryptoRng  for RandFile {
}

impl rand_core::RngCore for RandFile {
	fn next_u32(&mut self) -> u32 {
		let mut buf = [0u8; 4];
		let ores = self.f.read(&mut buf);
		if ores.is_err() {
			let e = ores.err().unwrap();
			panic!("read [{}] error[{:?}]",self.fname,e);
		}
		let cnt = ores.unwrap();
		if cnt != 4 {
			panic!("can not read [{}]", self.fname);
		}
		let mut retv :u32 = 0;
		for i in 0..buf.len() {
			retv |= (buf[i] as u32) << (i * 8);
		}
		retv
	}

	fn next_u64(&mut self) -> u64 {
		let mut buf = [0u8; 8];
		let ores = self.f.read(&mut buf);
		if ores.is_err() {
			let e = ores.err().unwrap();
			panic!("read [{}] error[{:?}]",self.fname,e);
		}
		let cnt = ores.unwrap();
		if cnt != 8 {
			panic!("can not read [{}]", self.fname);
		}
		let mut retv :u64 = 0;
		for i in 0..buf.len() {
			retv |= (buf[i] as u64) << (i * 8);
		}
		retv
	}

	fn fill_bytes(&mut self, dest: &mut [u8]) {
		let ores = self.f.read(dest);
		if ores.is_err() {
			let e = ores.err().unwrap();
			panic!("read [{}] error[{:?}]",self.fname,e);
		}
		let cnt = ores.unwrap();
		if cnt != dest.len() {
			panic!("can not read [{}]", self.fname);	
		}
		return;
	}

	fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(),rand_core::Error> {
		let ores = self.f.read(dest);
		if ores.is_err() {
			let e = ores.err().unwrap();
			let e2 = EcsimpleUtilsError::create(&format!("read {} error {:?}",self.fname,e));
			return Err(rand_core::Error::new(e2));
		}
		let cnt = ores.unwrap();
		if cnt != dest.len() {
			let e2 = EcsimpleUtilsError::create(&format!("read {} cnt {} != {}",self.fname,cnt,dest.len()));
			return Err(rand_core::Error::new(e2));
		}
		Ok(())
	}
}

pub struct RandOps {
	gencore : Option<rand::rngs::ThreadRng>,
	filerand : Option<RandFile>,
	begen : bool,
}

impl RandOps {
	pub fn new(fname :Option<String>) -> Result<Self,Box<dyn Error>> {
		let mut retv = RandOps {
			gencore : None,
			filerand : None,
			begen : true,
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