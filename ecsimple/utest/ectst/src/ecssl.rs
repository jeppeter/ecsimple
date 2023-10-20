 #[allow(unused_imports)]
 use extargsparse_codegen::{extargs_load_commandline,ArgSet,extargs_map_function};
 #[allow(unused_imports)]
 use extargsparse_worker::{extargs_error_class,extargs_new_error};
 #[allow(unused_imports)]
 use extargsparse_worker::namespace::{NameSpaceEx};
 #[allow(unused_imports)]
 use extargsparse_worker::argset::{ArgSetImpl};
 use extargsparse_worker::parser::{ExtArgsParser};
 use extargsparse_worker::funccall::{ExtArgsParseFunc};

 use std::cell::RefCell;
 use std::sync::Arc;
 use std::error::Error;
 use std::boxed::Box;
 #[allow(unused_imports)]
 use regex::Regex;
 #[allow(unused_imports)]
 use std::any::Any;

 use lazy_static::lazy_static;
 use std::collections::HashMap;

 use super::loglib::*;
 #[allow(unused_imports)]
 use super::fileop::*;
 use super::pemlib::*;
 #[allow(unused_imports)]
 use std::io::Write;
 use super::*;

//use num_bigint::{BigInt,Sign};

//use ecsimple::group::{ECGroupPrime,get_prime_group_curve};
use ecsimple::group::{ECGroup,ecc_get_curve_group,ecc_get_curve_names};
use ecsimple::signature::{ECSignature};
use ecsimple::keys::{ECPublicKey, ECPrivateKey,to_der_sm2};
use ecsimple::logger::*;
use super::strop::{parse_to_bigint};
use num_bigint::{BigInt};
//use ecsimple::consts::*;
use sha1::{Sha1,Digest};
use sha2::{Sha256,Sha512};
use sm3::{Sm3};


extargs_error_class!{EcsslError}

fn ecgen_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");
	let eccmprtype :String = ns.get_string("eccmprtype");
	let ecparamenc :String = ns.get_string("ecparamenc");
	let sm2privformat :bool = ns.get_bool("sm2privformat");

	init_log(ns.clone())?;

	if sarr.len() < 1 {
		extargs_new_error!{EcsslError,"need ecname"}
	}

	let ecname :String = format!("{}",sarr[0]);

	let grp :ECGroup = ecc_get_curve_group(&ecname)?;
	let privkey :ECPrivateKey ;
	if sarr.len() > 1 {
		let bn : BigInt = parse_to_bigint(&sarr[1])?;
		privkey = ECPrivateKey::new(&grp,&bn);
	} else {
		privkey = ECPrivateKey::generate(&grp);	
	}
	
	let pubkey :ECPublicKey = privkey.export_pubkey();
	let ecpub :String = ns.get_string("ecpub");
	if ecpub.len() > 0 {
		let pubdata = pubkey.to_der(&eccmprtype,&ecparamenc)?;
		let pubs :String = der_to_pem(&pubdata,"PUBLIC KEY")?;
		write_file_bytes(&ecpub,pubs.as_bytes())?;
	}

	let ecpriv :String = ns.get_string("ecpriv");
	if ecpriv.len() > 0 {
		let privdata = privkey.to_der(&eccmprtype,&ecparamenc)?;
		let privs :String ;
		if  privkey.is_sm2() {
			if sm2privformat {
				privs = der_to_pem(&privdata,"SM2 PRIVATE KEY")?;
			} else {
				let sm2privdata = to_der_sm2(&privdata)?;
				privs = der_to_pem(&sm2privdata,"PRIVATE KEY")?;
			}
			
		} else {
			privs = der_to_pem(&privdata,"EC PRIVATE KEY")?;
		}

		
		write_file_bytes(&ecpriv,privs.as_bytes())?;
	}


	Ok(())
}

fn ecprivload_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");
	let eccmprtype :String = ns.get_string("eccmprtype");
	let ecparamenc :String = ns.get_string("ecparamenc");
	let output :String = ns.get_string("output");
	let sm2privformat :bool = ns.get_bool("sm2privformat");
	init_log(ns.clone())?;
	debug_trace!("eccmprtype [{}] ecparamenc [{}]",eccmprtype,ecparamenc);
	for f in sarr.iter() {
		let privdata = read_file_into_der(f)?;
		let privkey :ECPrivateKey = ECPrivateKey::from_der(&privdata)?;
		debug_trace!("{}", privkey);
		let data :Vec<u8> = privkey.to_der(&eccmprtype,&ecparamenc)?;

		let outs :String;
		if privkey.is_sm2() {
			if sm2privformat {
				outs = der_to_pem(&data,"SM2 PRIVATE KEY")?;
			} else {
				let sm2privdata = to_der_sm2(&data)?;
				outs = der_to_pem(&sm2privdata,"PRIVATE KEY")?;
			}			
		} else {
			outs = der_to_pem(&data,"EC PRIVATE KEY")?;	
		}
		
		if output.len() > 0 {
			let _ = write_file_bytes(&output,outs.as_bytes())?;	
		}		
	}

	Ok(())
}

fn ecpubload_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");
	let eccmprtype :String = ns.get_string("eccmprtype");
	let ecparamenc :String = ns.get_string("ecparamenc");
	let output :String = ns.get_string("output");

	init_log(ns.clone())?;
	for f in sarr.iter() {
		let pubdata = read_file_into_der(f)?;
		let pubkey :ECPublicKey = ECPublicKey::from_der(&pubdata)?;
		//println!("{}", pubkey);
		let data :Vec<u8> = pubkey.to_der(&eccmprtype,&ecparamenc)?;
		let outs :String;
		outs = der_to_pem(&data,"PUBLIC KEY")?;
		
		if output.len() > 0 {
			let _ = write_file_bytes(&output,outs.as_bytes())?;	
		}
		
	}

	Ok(())
}

fn get_file_digest(infile :&str,dgsttype :&str) -> Result<Vec<u8>,Box<dyn Error>> {
	let blob = read_file_bytes(infile)?;
	let retv :Vec<u8>;
	if dgsttype == "sha1" {
		let mut hasher = Sha1::new();
		hasher.update(&blob);
		retv = hasher.finalize().to_vec();
	} else if dgsttype == "sha256" {
		let mut hasher = Sha256::new();
		hasher.update(&blob);
		retv = hasher.finalize().to_vec();
	} else if dgsttype == "sha512" {
		let mut hasher = Sha512::new();
		hasher.update(&blob);
		retv = hasher.finalize().to_vec();
	} else if dgsttype == "sm3" {
		let mut hasher = Sm3::new();
		hasher.update(&blob);
		retv = hasher.finalize().to_vec();
		debug_buffer_trace!(retv.as_ptr(),retv.len(),"sm3 data");
	} else {
		extargs_new_error!{EcsslError,"not support digesttype [{}]",dgsttype}
	}
	Ok(retv)
}

fn get_sm3_code(pubk :&ECPublicKey,infile :&str) -> Result<Vec<u8>,Box<dyn Error>> {
	let idv :Vec<u8> = Vec::new();
	let precode :Vec<u8> = pubk.get_sm3_hashcode(&idv)?;
	let fdata :Vec<u8> = read_file_bytes(infile)?;
	let mut hasher :Sm3 = Sm3::new();
	hasher.update(&precode);
	hasher.update(&fdata);
	Ok(hasher.finalize().to_vec())
}


fn format_digest(dgsttype :&str,file :&str, data :&[u8]) -> String {
	let mut rets :String = "".to_string();
	let mut idx :usize=0;
	let mut lasti :usize=0;
	rets.push_str(&format!("[{}]digest [{}]",file,dgsttype));
	for _ in data.iter() {
		if (idx % 16) == 0 {
			if idx > 0 {
				rets.push_str("    ");
				while lasti != idx {
					if data[lasti] >= ' ' as u8 && data[lasti] <= '~' as u8 {
						rets.push(data[lasti] as char);
					} else {
						rets.push_str(".");
					}
					lasti += 1;
				}
			}
			rets.push_str(&format!("\n0x{:08x}:",idx));
		}
		rets.push_str(&format!(" 0x{:02x}",data[idx]));
		idx += 1;
	}

	if idx != lasti {
		while (idx % 16) != 0 {
			rets.push_str("     ");
			idx += 1;
		}
		rets.push_str("    ");
		while lasti < data.len() {
			if data[lasti] >= ' ' as u8 && data[lasti] <= '~' as u8 {
				rets.push(data[lasti] as char);
			} else {
				rets.push_str(".");
			}
			lasti += 1;
		}
	}

	rets.push_str("\n");

	return rets;
}

fn ecsign_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");
	let ecpriv :String;
	let dgsttype :String;
	init_log(ns.clone())?;

	if sarr.len() < 1 {
		extargs_new_error!{EcsslError,"need one file blob"}
	}
	dgsttype = ns.get_string("digesttype");
	ecpriv = ns.get_string("ecpriv");
	if ecpriv.len() == 0 {
		extargs_new_error!{EcsslError,"not set ecpriv"}
	}
	let privdata :Vec<u8> = read_file_into_der(&ecpriv)?;
	let privkey :ECPrivateKey = ECPrivateKey::from_der(&privdata)?;
	set_ecsimple_logger_disable();
	let pubkey :ECPublicKey = privkey.export_pubkey();
	let hashbytes :Vec<u8>;
	if dgsttype == "sm3" {		
		hashbytes = get_sm3_code(&pubkey,&sarr[0])?;
	} else {
		hashbytes = get_file_digest(&sarr[0],&dgsttype)?;	
	}	
	let sig :ECSignature ;
	let sigdata :Vec<u8>;
	set_ecsimple_logger_enable();
	if dgsttype == "sm3" {
		sig = privkey.sign_sm2_base(&hashbytes)?;
	} else {
		sig = privkey.sign_base(&hashbytes)?;	
	}	
	sigdata = sig.encode_asn1()?;
	let output = ns.get_string("output");
	if output.len() > 0 {
		write_file_bytes(&output,&sigdata)?;
	}
	Ok(())
}

fn ecvfy_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");
	let ecpub :String;
	let dgsttype :String;
	init_log(ns.clone())?;

	if sarr.len() < 1 {
		extargs_new_error!{EcsslError,"need one file blob"}
	}
	dgsttype = ns.get_string("digesttype");
	ecpub = ns.get_string("ecpub");
	if ecpub.len() == 0 {
		extargs_new_error!{EcsslError,"not set ecpub"}
	}
	let pubdata :Vec<u8> = read_file_into_der(&ecpub)?;
	println!("pub data ");
	let pubkey :ECPublicKey = ECPublicKey::from_der(&pubdata)?;
	println!("PublicKey");
	let hashbytes :Vec<u8>;
	if dgsttype == "sm3" {
		hashbytes = get_sm3_code(&pubkey,&sarr[0])?;
	} else {
		hashbytes = get_file_digest(&sarr[0],&dgsttype)?;	
	}
	
	let sigfile = ns.get_string("input");
	if sigfile.len() == 0 {
		extargs_new_error!{EcsslError,"no input for signbin"}
	}
	let sigdata = read_file_bytes(&sigfile)?;
	let sig :ECSignature = ECSignature::decode_asn1(&sigdata)?;	
	let retval :bool;
	if dgsttype == "sm3" {
		retval  = pubkey.verify_sm2_base(&sig,&hashbytes)?;
	} else {
		retval  = pubkey.verify_base(&sig,&hashbytes)?;	
	}	
	if !retval  {
		extargs_new_error!{EcsslError,"verify ecpub[{}] with file [{}] sign[{}] not valid", ecpub,sarr[0],sigfile}
	}
	println!("verify ecpub[{}] with file [{}] sign[{}] succ", ecpub,sarr[0],sigfile);
	Ok(())
}

fn digest_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");
	let dgsttype :String;
	init_log(ns.clone())?;

	if sarr.len() < 1 {
		extargs_new_error!{EcsslError,"need one file blob"}
	}
	dgsttype = ns.get_string("digesttype");
	for f in sarr.iter() {
		let digdata = get_file_digest(f,&dgsttype)?;
		let rets = format_digest(&dgsttype,f,&digdata);
		print!("{}",rets);
	}
	Ok(())
}

fn eclist_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let maxnum :i64 = ns.get_int("maxnum");
	let mut maxlen : usize = 1;
	let ecnames :Vec<String>;
	let mut curname :String;
	let mut curline :String;
	init_log(ns.clone())?;
	ecnames = ecc_get_curve_names();
	let mut idx :usize = 0;
	while idx < ecnames.len() {
		if ecnames[idx].len() > maxlen {
			maxlen = ecnames[idx].len();
		}
		idx += 1;
	}

	curline = "".to_string();
	idx = 0;
	while idx < ecnames.len() {
		if (idx % maxnum as usize) == 0 && idx != 0 {
			println!("{}", curline);
			curline = "".to_string();
		}
		curname = format!("{}",ecnames[idx]);
		while curname.len() < maxlen {
			curname.push(' ');
		}
		if curline.len() > 0 {
			curline.push_str(" ");
		}
		curline.push_str(&format!("{}",curname));
		idx += 1;
	}

	if curline.len() > 0 {
		println!("{}", curline);
	}
	Ok(())
}


#[extargs_map_function(ecgen_handler,ecprivload_handler,ecpubload_handler,ecsign_handler,ecvfy_handler,digest_handler,eclist_handler)]
pub fn ec_ssl_parser(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = format!(r#"
		{{
			"sm2privformat" : true,
			"maxnum" : 5,
			"digesttype##only support sha1 sha256 sha512 sm3##" : "sha1",
			"ecgen<ecgen_handler>##ecname to generate ec private key##" : {{
				"$" : "+"
			}},
			"ecprivload<ecprivload_handler>##ecprivpem ... to load private key##" : {{
				"$" : "+"
			}},
			"ecpubload<ecpubload_handler>##ecpubpem ... to load ecpub key##" : {{
				"$" : "+"
			}},
			"ecsign<ecsign_handler>##file  the file blob to sign and output is sign ecpriv is private key##" : {{
				"$" : 1
			}},
			"ecvfy<ecvfy_handler>##file the file blob to verify input is sign ecpub is public key##" : {{
				"$" : 1
			}},
			"digest<digest_handler>##file ... to make digest value##" : {{
				"$" : "+"
			}},
			"eclist<eclist_handler>##to list all support ec types##" : {{
				"$" : 0
			}}
		}}
		"#);
	extargs_load_commandline!(parser,&cmdline)?;
	Ok(())
}