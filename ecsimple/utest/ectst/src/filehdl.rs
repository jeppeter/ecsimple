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
//use std::ops::{Add,Mul,Div,Rem};


extargs_error_class!{FileHdlError}



fn rnadwr_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");
	let mut bs :Vec<u8> = Vec::new();
	let fname :String;

	init_log(ns.clone())?;

	for v in sarr.iter() {
		let bn :BigInt = parse_to_bigint(v)?;
		let (_,vv) = bn.to_bytes_be();
		bs.extend(&vv);
	}

	fname = ns.get_string("output");
	let _ = write_file_bytes(&fname,&bs)?;
	Ok(())
}


#[extargs_map_function(rnadwr_handler)]
pub fn file_load_parser(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"randwr<rnadwr_handler>##bins ... to write bytes##" : {
			"$" : "+"
		}
	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}