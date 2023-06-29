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

use ecsimple::binbn::*;

use num_bigint::{BigInt};


extargs_error_class!{BinError}

fn get_binbn(s :&str) -> Result<BinBn,Box<dyn Error>> {
	let bn :BigInt = parse_to_bigint(s)?;
	Ok(BinBn::new_from_bigint(&bn))
}

fn binbnload_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");

	init_log(ns.clone())?;

	for v in sarr.iter() {
		let bn :BigInt = parse_to_bigint(v)?;
		let (_,vv) = bn.to_bytes_be();
		let bebn :BinBn = BinBn::new_from_be(&vv);
		let lebn :BinBn = BinBn::new_from_le(&vv);
		println!("v {} bebn 0x{:x} lebn 0x{:x}", v,bebn,lebn);
		//println!("v {} bebn 0x{:x}", v,bebn);
	}
	Ok(())
}


fn binadd_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");

	init_log(ns.clone())?;

	if sarr.len() < 2 {
		extargs_new_error!{BinError,"need anum and bnum"}
	}
	let aval :BinBn = get_binbn(&sarr[0])?;
	let bval :BinBn = get_binbn(&sarr[1])?;

	let cval :BinBn = aval.add_op(&bval);
	println!("0x{:x} + 0x{:x} = 0x{:x}",aval,bval,cval);

	Ok(())
}

fn binmul_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");

	init_log(ns.clone())?;

	if sarr.len() < 2 {
		extargs_new_error!{BinError,"need anum and bnum"}
	}
	let aval :BinBn = get_binbn(&sarr[0])?;
	let bval :BinBn = get_binbn(&sarr[1])?;

	let cval :BinBn = aval.mul_op(&bval);
	println!("0x{:x} * 0x{:x} = 0x{:X}",aval,bval,cval);

	Ok(())
}


#[extargs_map_function(binbnload_handler,binadd_handler,binmul_handler)]
pub fn bn_load_parser(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"binbnload<binbnload_handler>" : {
			"$" : "+"
		},
		"binadd<binadd_handler>## anum + bnum in bin mode##" : {
			"$" : 2
		},
		"binmul<binmul_handler>## anum * bnum in bin mode##" : {
			"$" : "+"
		}
	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}