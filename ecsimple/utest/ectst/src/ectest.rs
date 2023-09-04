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
use super::strop::*;
use super::*;
#[allow(unused_imports)]
use std::io::Write;

use num_bigint::{BigInt,Sign};

use ecsimple::group::{ECGroupPrime,get_prime_group_curve};
use ecsimple::point::ECPrimePoint;
use ecsimple::signature::{ECSignature};
use ecsimple::keys::{ECPrimePrivateKey,ECPrimePubKey};


extargs_error_class!{EcError}

fn ecgen_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");

	init_log(ns.clone())?;

	if sarr.len() < 2 {
		extargs_new_error!{EcError,"need ecname and private name"}
	}

	let ecname :String = format!("{}",sarr[0]);
	let bn : BigInt = parse_to_bigint(&sarr[1])?;

	let grp :ECGroupPrime = get_prime_group_curve(&ecname)?;
	let pnt : ECPrimePoint = ECPrimePoint::new(&grp);

	let pubpnt :ECPrimePoint = pnt.mul_op(&bn,false);

	println!("from {} * 0x{:x} = {}",pnt,bn, pubpnt);
	Ok(())
}

fn ecsignbase_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");

	init_log(ns.clone())?;

	if sarr.len() < 3 {
		extargs_new_error!{EcError,"need ecname and private number and hashnumber"}
	}

	let ecname :String = format!("{}",sarr[0]);
	let bn : BigInt = parse_to_bigint(&sarr[1])?;
	let hashnum :BigInt = parse_to_bigint(&sarr[2])?;
	let (_, mut bs) = hashnum.to_bytes_be();

	if sarr.len() > 3 {
		let  hashlen : u64 = parse_u64(&sarr[3])?;
		while bs.len() < (hashlen as usize) {
			bs.insert(0,0);
		}
	}

	let grp :ECGroupPrime = get_prime_group_curve(&ecname)?;
	let privkey :ECPrimePrivateKey = ECPrimePrivateKey::new(&grp,&bn);

	let sig :ECSignature = privkey.sign_base(&bs)?;

	println!("{}", sig);



	Ok(())
}

fn ecvfybase_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");

	init_log(ns.clone())?;

	if sarr.len() < 4 {
		extargs_new_error!{EcError,"need ecname and private number and hashnumber and signbin"}
	}

	let ecname = format!("{}",sarr[0]);
	let ecpubfile = format!("{}",sarr[1]);
	let hashnum :BigInt = parse_to_bigint(&sarr[2])?;
	let signbin = format!("{}",sarr[3]);


	let grp :ECGroupPrime = get_prime_group_curve(&ecname)?;
	let rdata :Vec<u8> = read_file_bytes(&ecpubfile)?;
	let pubkey :ECPrimePubKey = ECPrimePubKey::from_der(&grp,&rdata)?;
	let sigdata :Vec<u8> = read_file_bytes(&signbin)?;
	let sig :ECSignature = ECSignature::decode_asn1(&sigdata)?;
	println!("sig.r 0x{:X} sig.s 0x{:X}",sig.r,sig.s);
	let ok :bool = pubkey.verify_base(&sig,&hashnum)?;
	println!("verify 0x{:X} with signature [{}] {:?}", hashnum,signbin,ok);

	Ok(())
}


fn ecpubload_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");

	init_log(ns.clone())?;

	if sarr.len() < 2 {
		extargs_new_error!{EcError,"need ecname and ecpub"}
	}

	let ecname = format!("{}",sarr[0]);
	let ecfile = format!("{}",sarr[1]);


	let grp :ECGroupPrime = get_prime_group_curve(&ecname)?;
	let rdata :Vec<u8> = read_file_bytes(&ecfile)?;
	let ecpub :ECPrimePubKey = ECPrimePubKey::from_der(&grp,&rdata)?;	
	println!("load {} {} succ\n{}", ecname, ecfile,ecpub);

	Ok(())
}


fn extractsign_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {

	init_log(ns.clone())?;
	let input :String = ns.get_string("input");
	let output :String = ns.get_string("output");

	let sigdata :Vec<u8> = read_file_bytes(&input)?;
	let sig :ECSignature = ECSignature::decode_asn1(&sigdata)?;
	let mut outdata :Vec<u8> = Vec::new();
	let (_,r) = sig.r.to_bytes_be();
	let (_,s) = sig.s.to_bytes_be();
	outdata.extend(r);
	outdata.extend(s);
	if output.len() > 0 {
		write_file_bytes(&output,&outdata)?;
	} else {
		debug_buffer_trace!(outdata.as_ptr(),outdata.len(),"outdata ");
	}


	Ok(())
}

fn encapsign_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {

	init_log(ns.clone())?;
	let input :String = ns.get_string("input");
	let output :String = ns.get_string("output");

	let sigdata :Vec<u8> = read_file_bytes(&input)?;
	let siglen :usize = sigdata.len() >> 1;
	let mut r :Vec<u8> = Vec::new();
	let mut idx :usize;
	idx = 0;
	while idx < siglen {
		r.push(sigdata[idx]);
		idx += 1;
	}
	let mut s :Vec<u8> = Vec::new();
	idx = siglen;
	while idx < sigdata.len() {
		s.push(sigdata[idx]);
		idx += 1;
	}
	let rbn :BigInt = BigInt::from_bytes_be(Sign::Plus,&r);
	let sbn :BigInt = BigInt::from_bytes_be(Sign::Plus,&s);
	let sig :ECSignature = ECSignature::new(&rbn,&sbn);
	let data :Vec<u8> = sig.encode_asn1()?;
	if output.len() > 0 {
		write_file_bytes(&output,&data)?;
	} else {
		debug_buffer_trace!(data.as_ptr(),data.len(),"outdata ");
	}


	Ok(())
}


#[extargs_map_function(ecgen_handler,ecsignbase_handler,ecvfybase_handler,ecpubload_handler,extractsign_handler,encapsign_handler)]
pub fn ec_load_parser(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"ecgen<ecgen_handler>##ecname privatenum to generate ec private key##" : {
			"$" : "+"
		},
		"ecsignbase<ecsignbase_handler>##ecname privatenum hashnum [hashlen] to generate sign values##" : {
			"$" : "+"
		},
		"ecvfybase<ecvfybase_handler>##ecname privatenum hashnum signbin to verify sign##" : {
			"$" : "+"
		},
		"ecpubload<ecpubload_handler>##ecname pubbin to load ec public key##" : {
			"$" : 2
		},
		"extractsign<extractsign_handler>##input input and output for output##" : {
			"$" : 0
		},
		"encapsign<encapsign_handler>##input input and output for output for input sign##" : {
			"$" : 0
		}
	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}