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
#[allow(unused_imports)]
use std::io::Write;

use num_bigint::{BigInt};

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
#[extargs_map_function(ecgen_handler,ecsignbase_handler,ecvfybase_handler,ecpubload_handler)]
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
		}
	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}