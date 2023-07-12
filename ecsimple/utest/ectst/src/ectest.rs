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

use ecsimple::group::{ECGroupBnGf2m,get_bn_group_curve};
use ecsimple::point::ECGf2mPoint;
use ecsimple::signature::{ECSignature};
use ecsimple::keys::{ECGf2mPrivateKey};


extargs_error_class!{EcError}

fn ecgen_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");

	init_log(ns.clone())?;

	if sarr.len() < 2 {
		extargs_new_error!{EcError,"need ecname and private name"}
	}

	let ecname :String = format!("{}",sarr[0]);
	let bn : BigInt = parse_to_bigint(&sarr[1])?;

	let grp :ECGroupBnGf2m = get_bn_group_curve(&ecname)?;
	let pnt : ECGf2mPoint = ECGf2mPoint::new(&grp);

	let pubpnt :ECGf2mPoint = pnt.mul_op(&bn);

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

	let grp :ECGroupBnGf2m = get_bn_group_curve(&ecname)?;
	let privkey :ECGf2mPrivateKey = ECGf2mPrivateKey::new(&grp,&bn);

	let sig :ECSignature = privkey.sign_base(&bs)?;

	println!("{}", sig);



	Ok(())
}


#[extargs_map_function(ecgen_handler,ecsignbase_handler)]
pub fn ec_load_parser(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"ecgen<ecgen_handler>##ecname privatenum to generate ec private key##" : {
			"$" : "+"
		},
		"ecsignbase<ecsignbase_handler>##ecname privatenum hashnum [hashlen] to generate sign values##" : {
			"$" : "+"
		}
	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}