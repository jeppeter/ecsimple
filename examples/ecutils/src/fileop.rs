#[allow(unused_imports)]
use super::{debug_trace,debug_buffer_trace,format_buffer_log};
#[allow(unused_imports)]
use super::loglib::{log_get_timestamp,log_output_function,init_log};

#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};

use std::io;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use rand::RngCore;


use std::error::Error;

extargs_error_class!{FileOpError}

pub fn write_stdout_bytes(note :&str,byts :&[u8]) -> Result<(),Box<dyn Error>> {
	let mut stdf = io::stdout();
	let mut ws :String= "".to_string();
	let mut i :usize;
	let mut lasti :usize;
	ws.push_str(note);

	lasti = 0;
	i = 0;
	while i < byts.len() {
		if (i % 16) == 0 {
			if i > 0 {
				ws.push_str("    ");
				while lasti < i {
					if byts[lasti] >= 0x20 && byts[lasti] <= 0x7e {
						ws.push(byts[lasti] as char);
					} else {
						ws.push_str(".");
					}
					lasti += 1;
				}
			}
			ws.push_str(&format!("\n0x{:08x}:",i));
		}
		ws.push_str(&format!(" 0x{:02x}",byts[i]));
		i += 1;
	}

	if i != lasti {
		while (i % 16) != 0 {
			ws.push_str("     ");
			i += 1;
		}
		while lasti < byts.len() {
			if byts[lasti] >= 0x20 && byts[lasti] <= 0x7e {
				ws.push(byts[lasti] as char);
			} else {
				ws.push_str(".");
			}
			lasti += 1;			
		}
		ws.push_str("\n");
	}
	let res = stdf.write_all(ws.as_bytes());
	if res.is_err() {
		let err = res.err().unwrap();
		extargs_new_error!{FileOpError,"write [stdout] len[{}] error[{:?}]", byts.len(),err}
	}
	Ok(())

}

pub fn write_file_bytes(fname :&str, byts :&[u8]) -> Result<(),Box<dyn Error>> {
	if fname.len() == 0 {
		let res = io::stdout().write_all(byts);
		if res.is_err() {
			let err = res.err().unwrap();
			extargs_new_error!{FileOpError,"write [stdout] len[{}] error[{:?}]", byts.len(),err}	
		}
	} else {
		let fo  = fs::File::create(fname);
		if fo.is_err() {
			let err = fo.err().unwrap();
			extargs_new_error!{FileOpError,"create [{}] error[{:?}]", fname,err}
		}
		let mut fp :fs::File = fo.unwrap();
		let res = fp.write_all(byts);
		if res.is_err() {
			let err = res.err().unwrap();
			extargs_new_error!{FileOpError,"write [{}] len[{}] error[{:?}]", fname, byts.len(),err}	
		}
	}
	Ok(())
}



pub fn read_file_bytes(fname :&str) -> Result<Vec<u8>,Box<dyn Error>> {
	if fname.len() == 0 {
		let f = io::stdin();
		let mut reader = BufReader::new(f);
		let mut buf :Vec<u8> = Vec::new();
		let res = reader.read_to_end(&mut buf);
		if res.is_err() {
			let err = res.err().unwrap();
			extargs_new_error!{FileOpError,"read [{}] error [{:?}]", fname,err}
		}
		Ok(buf)
	} else {
		let fo = fs::File::open(fname);
		if fo.is_err() {
			let err = fo.err().unwrap();
			extargs_new_error!{FileOpError,"can not open [{}] error[{:?}]", fname, err}
		}
		let f = fo.unwrap();
		let mut reader = BufReader::new(f);
		let mut buf :Vec<u8> = Vec::new();
		let res = reader.read_to_end(&mut buf);
		if res.is_err() {
			let err = res.err().unwrap();
			extargs_new_error!{FileOpError,"read [{}] error [{:?}]", fname,err}
		}

		Ok(buf)		
	}
}

#[allow(dead_code)]
pub fn read_file(fname :&str) -> Result<String,Box<dyn Error>> {
	if fname.len() == 0 {
		let f = io::stdin();
		let mut reader = BufReader::new(f);
		let mut retv :String = String::new();
		let res = reader.read_to_string(&mut retv);
		if res.is_err() {
			let err = res.err().unwrap();
			extargs_new_error!{FileOpError,"read [{}] error [{:?}]", fname,err}
		}
		Ok(retv)
	} else {
		let fo = fs::File::open(fname);
		if fo.is_err() {
			let err = fo.err().unwrap();
			extargs_new_error!{FileOpError,"can not open [{}] error[{:?}]", fname, err}
		}
		let f = fo.unwrap();
		let mut reader = BufReader::new(f);
		let mut retv :String = String::new();
		let res = reader.read_to_string(&mut retv);
		if res.is_err() {
			let err = res.err().unwrap();
			extargs_new_error!{FileOpError,"read [{}] error [{:?}]", fname,err}
		}

		Ok(retv)		
	}
}

#[allow(dead_code)]
pub fn get_rand_bytes(nbytes :i32) -> Vec<u8> {

	let mut r : rand::rngs::ThreadRng = rand::thread_rng();
	let mut buf :Vec<u8> = Vec::new();
	for _ in 0..nbytes {
		buf.push(0x0);
	}

	let _ = r.try_fill_bytes(&mut buf);
	return buf;
}

