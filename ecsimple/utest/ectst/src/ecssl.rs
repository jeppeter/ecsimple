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

//use ecsimple::group::{ECGroupPrime,get_prime_group_curve};
use ecsimple::group::{ECGroup,ecc_get_curve_group};
use ecsimple::signature::{ECSignature};
use ecsimple::keys::{ECPublicKey, ECPrivateKey};
use ecsimple::consts::*;


extargs_error_class!{EcsslError}

fn ecgen_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");
	let eccmprtype :String = ns.get_string("eccmprtype");

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
		let privdata = privkey.to_der(&eccmprtype)?;
		write_file_bytes(&ecpriv,&privdata)?;
	}


	Ok(())
}



#[extargs_map_function(ecgen_handler)]
pub fn ec_ssl_parser(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = format!(r#"
	{{
		"ecgen<ecgen_handler>##ecname to generate ec private key##" : {{
			"$" : 1
		}}
	}}
	"#);
	extargs_load_commandline!(parser,&cmdline)?;
	Ok(())
}