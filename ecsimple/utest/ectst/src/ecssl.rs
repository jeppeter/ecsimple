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

//use num_bigint::{BigInt,Sign};

//use ecsimple::group::{ECGroupPrime,get_prime_group_curve};
use ecsimple::group::{ECGroup,ecc_get_curve_group};
//use ecsimple::signature::{ECSignature};
use ecsimple::keys::{ECPublicKey, ECPrivateKey};
//use ecsimple::consts::*;


extargs_error_class!{EcsslError}

fn ecgen_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");
	let eccmprtype :String = ns.get_string("eccmprtype");
	let ecparamenc :String = ns.get_string("ecparamenc");

	init_log(ns.clone())?;

	if sarr.len() < 1 {
		extargs_new_error!{EcsslError,"need ecname"}
	}

	let ecname :String = format!("{}",sarr[0]);

	let grp :ECGroup = ecc_get_curve_group(&ecname)?;
	let privkey :ECPrivateKey = ECPrivateKey::generate(&grp);
	let pubkey :ECPublicKey = privkey.export_pubkey();
	let ecpub :String = ns.get_string("ecpub");
	if ecpub.len() > 0 {
		let pubdata = pubkey.to_bin(&eccmprtype)?;
		write_file_bytes(&ecpub,&pubdata)?;
	}

	let ecpriv :String = ns.get_string("ecpriv");
	if ecpriv.len() > 0 {
		let privdata = privkey.to_der(&eccmprtype,&ecparamenc)?;
		write_file_bytes(&ecpriv,&privdata)?;
	}


	Ok(())
}

fn ecprivload_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");
	let eccmprtype :String = ns.get_string("eccmprtype");
	let ecparamenc :String = ns.get_string("ecparamenc");
	let output :String = ns.get_string("output");
	println!("eccmprtype [{}] ecparamenc [{}]",eccmprtype,ecparamenc);
	init_log(ns.clone())?;
	for f in sarr.iter() {
		let privdata = read_file_into_der(f)?;
		let privkey :ECPrivateKey = ECPrivateKey::from_der(&privdata)?;
		println!("{}", privkey);
		let data :Vec<u8> = privkey.to_der(&eccmprtype,&ecparamenc)?;

		let outs :String;
		if privkey.is_sm2() {
			outs = der_to_pem(&data,"SM2 PRIVATE KEY")?;
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
		println!("{}", pubkey);
		let data :Vec<u8> = pubkey.to_der(&eccmprtype,&ecparamenc)?;
		let outs :String;
		outs = der_to_pem(&data,"PUBLIC KEY")?;
		
		if output.len() > 0 {
			let _ = write_file_bytes(&output,outs.as_bytes())?;	
		}
		
	}

	Ok(())
}


#[extargs_map_function(ecgen_handler,ecprivload_handler,ecpubload_handler)]
pub fn ec_ssl_parser(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = format!(r#"
	{{
		"ecgen<ecgen_handler>##ecname to generate ec private key##" : {{
			"$" : 1
		}},
		"ecprivload<ecprivload_handler>##ecprivpem ... to load private key##" : {{
			"$" : "+"
		}},
		"ecpubload<ecpubload_handler>##ecpubpem ... to load ecpub key##" : {{
			"$" : "+"
		}}
	}}
	"#);
	extargs_load_commandline!(parser,&cmdline)?;
	Ok(())
}