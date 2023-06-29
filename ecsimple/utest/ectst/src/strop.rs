#[allow(unused_imports)]
use super::{debug_trace,debug_buffer_trace,format_buffer_log};
#[allow(unused_imports)]
use super::loglib::{log_get_timestamp,log_output_function,init_log};

#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};

use base64;
use std::error::Error;
use num_bigint::{BigInt};
use num_traits::{zero};


extargs_error_class!{StrOpError}


pub fn encode_base64(bb :&[u8]) -> String {
	return base64::encode(bb);
}

#[allow(dead_code)]
pub fn decode_base64(instr :&str) -> Result<Vec<u8>,Box<dyn Error>> {
	let res = base64::decode(instr);
	if res.is_err() {
		let err = res.err().unwrap();
		extargs_new_error!{StrOpError,"can not parse [{}] for base64 error [{:?}]", instr,err}
	}
	let bv = res.unwrap();
	Ok(bv)
}

pub fn parse_u64(instr :&str) -> Result<u64,Box<dyn Error>> {
	let mut cparse = format!("{}",instr);
	let mut base :u32 = 10;
	let retv :u64;
	if cparse.starts_with("0x") || cparse.starts_with("0X") {
		cparse = cparse[2..].to_string();
		base = 16;
	} else if cparse.starts_with("x") || cparse.starts_with("X") {
		cparse = cparse[1..].to_string();
		base = 16;
	}

	match u64::from_str_radix(&cparse,base) {
		Ok(v) => {
			retv = v;
		},
		Err(e) => {
			extargs_new_error!{StrOpError, "parse [{}] error [{:?}]", instr, e}
		}
	}
	Ok(retv)
}

pub fn parse_to_bigint(instr :&str) -> Result<BigInt,Box<dyn Error>> {
	let mut _cparse = format!("{}",instr);
	let mut base :u32 = 10;
	let mut retv :BigInt = zero();
	let mut curv :BigInt ;
	let mut curvi :i32;
	let mut addi :i32 = 0;
	let mut bbchar :String;
	let mut negv :bool = false;
	let cparse :Vec<u8>;
	if _cparse.starts_with("0x") || _cparse.starts_with("0X") {
		_cparse = _cparse[2..].to_string();
		base = 16;
		addi += 2;
	} else if _cparse.starts_with("x") || _cparse.starts_with("X") {
		_cparse = _cparse[1..].to_string();
		base = 16;
		addi += 1;
	}

	if _cparse.starts_with("-") {
		_cparse = _cparse[1..].to_string();
		negv = true;
		addi += 1;
	}

	cparse = _cparse.as_bytes().to_vec();

	if cparse.len() == 0 {
		extargs_new_error!{StrOpError,"not valid [{}]",instr};
	}

	let mut lasti :usize = 0;
	let mut idx :i32 = 0;
	while lasti < cparse.len() {
		if base == 10 {
			if cparse[lasti] >= ('0' as u8) && cparse[lasti] <= ('9' as u8) {
				curvi = (cparse[lasti] - ('0' as u8)) as i32;
			} else {
				bbchar = "".to_string();
				if cparse[lasti] >= 0x20 && cparse[lasti] <= 0x7e {
					bbchar.push(cparse[lasti] as char);
				} else {
					bbchar.push_str(&format!("char[0x{:x}]",cparse[lasti]));
				}
				
				extargs_new_error!{StrOpError,"[{}] character not valid [{}]", idx + addi,bbchar}
			}
			curv = curvi.into();
			retv *= 10;
			retv += curv;
		} else {
			if cparse[lasti] >= ('0'  as u8) && cparse[lasti] <= ('9' as u8) {
				curvi = (cparse[lasti] - ('0' as u8)) as i32;
			} else if cparse[lasti] >= ('a'  as u8) && cparse[lasti] <= ('f' as u8){
				curvi = (cparse[lasti] - ('a' as u8)) as i32 + 10;
			} else if cparse[lasti] >= ('A' as u8 ) && cparse[lasti] <= ('F' as u8)  { 
				curvi = (cparse[lasti] - ('A' as u8)) as i32 + 10;
			} else {
				bbchar = "".to_string();
				if cparse[lasti] >= 0x20 && cparse[lasti] <= 0x7e {
					bbchar.push(cparse[lasti] as char);
				} else {
					bbchar.push_str(&format!("char[0x{:x}]",cparse[lasti]));
				}

				extargs_new_error!{StrOpError,"[{}] character not valid [{}]", idx + addi,bbchar}
			}
			curv = curvi.into();
			retv *= 16;
			retv += curv;
		}
		lasti += 1;
		idx += 1;
	}

	if negv {
		retv = -retv;
	}
	Ok(retv)
}


