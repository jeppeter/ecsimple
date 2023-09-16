
use std::error::Error;
use std::boxed::Box;
#[allow(unused_imports)]
use regex::Regex;

use extargsparse_worker::{extargs_error_class,extargs_new_error};
use super::strop::{decode_base64,encode_base64};

extargs_error_class!{PemLibError}

pub fn pem_to_der(ins :&str) -> Result<(Vec<u8>,String),Box<dyn Error>> {
	let retv :Vec<u8>;
	let sarr :Vec<&str> = ins.split("\n").collect();
	let mut bstr :String = "".to_string();
	let regstr = "[\\-]+BEGIN\\s([^\\-]+)[\\-]+".to_string();	
	let mut notice :String = "".to_string();

	let ro = Regex::new(&regstr);
	if ro.is_err() {
		let e = ro.err().unwrap();
		extargs_new_error!{PemLibError,"compile [{}] error[{:?}]", regstr,e}
	}
	let re = ro.unwrap();

	for l in sarr.iter() {
		let mut c :String = format!("{}",l);
		c = c.trim_end_matches("\r").to_string();
		if !c.starts_with("---") {
			bstr.push_str(&format!("{}",c));
		} else {
			let caps = re.captures(&c);
			if caps.is_some() {
				let cp = caps.unwrap();
				notice = format!("{}", cp.get(1).map_or("", |m| m.as_str()));
			}
		}
	}
	retv = decode_base64(&bstr)?;
	Ok((retv,notice))
}

const DEFAULT_PEM_LENGTH :usize = 64;

pub fn der_to_pem(inb :&[u8],notice :&str) -> Result<String,Box<dyn Error>> {
	let outs :String;
	let mut rets :String = "".to_string();
	let mut idx :usize;
	let mut perlen :usize;

	outs = encode_base64(inb);
	rets.push_str(&format!("-----BEGIN {}-----\n",notice));
	idx = 0;
	while idx < outs.len() {
		perlen = DEFAULT_PEM_LENGTH;
		if (idx + perlen) > outs.len() {
			perlen = outs.len() - idx;
		}
		rets.push_str(&format!("{}\n",&outs[idx..(idx+perlen)]));
		idx += perlen;
	}
	rets.push_str(&format!("-----END {}-----\n",notice));
	Ok(rets)
}