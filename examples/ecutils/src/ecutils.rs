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

use ecsimple::keys::{PrivateKey,PublicKey};
use ecsimple::consts::*;
use ecsimple::signature::{ECCSignature};
use ecsimple::{ecsimple_error_class,ecsimple_new_error};
use ecsimple::curves::{get_ecc_curve_by_name};


extargs_error_class!{EcError}

fn ecgen_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");

	init_log(ns.clone())?;

	if sarr.len() < 1 {
		extargs_new_error!{EcError,"need ecname and private name"}
	}

	let ecname :String = format!("{}",sarr[0]);
	let bn :BigInt;
	let privkey :PrivateKey;
	let eccurve = get_ecc_curve_by_name(&ecname)?;
	let eccmprtype = ns.get_string("eccmprtype");
	let ectype = ns.get_string("ectype");
	let ecexplicit = ns.get_string("ecexplicit");

	if sarr.len() > 1 {
		bn = parse_to_bigint(&sarr[1])?;
		privkey = PrivateKey::new(&eccurve,&bn)?;
	} else {
		privkey = PrivateKey::generate(&eccurve,None)?;
	}
	let pubk = privkey.get_public_key();
	let pubbytes :Vec<u8> = pubk.to_der(&eccmprtype,&ecexplicit)?;
	let privbytes :Vec<u8> = privkey.to_der(&eccmprtype,&ectype,&ecexplicit)?;

	let ecprivfile = ns.get_string("ecpriv");
	let ecpubfile = ns.get_string("ecpub");
	if ecprivfile.len() == 0 {
		write_stdout_bytes("ecpriv",&privbytes)?;
	} else {
		write_file_bytes(&ecprivfile,&privbytes)?;
	}

	if ecpubfile.len() == 0 {
		write_stdout_bytes("ecpub",&pubbytes)?;
	} else {
		write_file_bytes(&ecpubfile,&pubbytes)?;
	}
	Ok(())
}

fn ecsignbase_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");

	init_log(ns.clone())?;
	if sarr.len() < 2 {
		ecsimple_new_error!{EcError,"need hashnum randkey"}
	}
	let ecprivfile = ns.get_string("ecpriv");
	if ecprivfile.len() == 0 {
		ecsimple_new_error!{EcError,"need ecpriv"}
	}
	let privbin = read_file_bytes(&ecprivfile)?;
	let privkey :PrivateKey = PrivateKey::from_der(&privbin)?;
	let bn :BigInt = parse_to_bigint(&sarr[0])?;
	let randkey :BigInt = parse_to_bigint(&sarr[1])?;
	let (_,bnvecs) = bn.to_bytes_be();
	loop {
		let ores = privkey.sign_base(&bnvecs,&randkey);
		if ores.is_ok() {
			let sigv :ECCSignature = ores.unwrap();
			let asn1sigv = sigv.to_der()?;
			let ofile = ns.get_string("output");
			if ofile.len() == 0 {
				write_stdout_bytes("signature",&asn1sigv)?;
			} else {
				write_file_bytes(&ofile,&asn1sigv)?;
			}
			break;
		}
	}
	Ok(())
}

fn ecvfybase_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");

	init_log(ns.clone())?;
	if sarr.len() < 2 {
		ecsimple_new_error!{EcError,"need hashnum signbin"}
	}
	let ecpubfile = ns.get_string("ecpub");
	if ecpubfile.len() == 0 {
		ecsimple_new_error!{EcError,"need ecpub"}
	}
	let pubbin = read_file_bytes(&ecpubfile)?;
	let pubkey :PublicKey= PublicKey::from_der(&pubbin)?;
	let bn :BigInt = parse_to_bigint(&sarr[0])?;
	let signfile :String = format!("{}",sarr[1]);
	let (_,bnvecs) = bn.to_bytes_be();
	let signdata :Vec<u8> = read_file_bytes(&signfile)?;
	let sig :ECCSignature = ECCSignature::from_der(&signdata)?;
	let bval :bool = pubkey.verify_base(&bnvecs,&sig);
	if bval {
		println!("verify {} succ", signfile);
	} else {
		println!("verify {} failed", signfile);
	}
	Ok(())
}


fn ecpubload_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	Ok(())
}
#[extargs_map_function(ecgen_handler,ecsignbase_handler,ecvfybase_handler,ecpubload_handler)]
pub fn ecutil_load_args(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"eccmprtype" : "compressed",
		"ectype" : "ssleay",
		"ecexplicit" : "explicit",
		"ecpriv" : null,
		"ecpub" : null,
		"ecgen<ecgen_handler>##ecname [privatenum] to generate ec private key##" : {
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