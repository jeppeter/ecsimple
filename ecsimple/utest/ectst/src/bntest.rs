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
use super::*;
#[allow(unused_imports)]
use super::fileop::*;
use super::strop::*;
#[allow(unused_imports)]
use std::io::Write;

use ecsimple::bngf2m::*;
use ecsimple::randop::*;
use ecsimple::mont::*;

use num_bigint::{BigInt};
use num_traits::{one,zero};
//use std::ops::{Add,Mul,Div,Rem};


extargs_error_class!{BinError}

fn get_bngf2m(s :&str) -> Result<BnGf2m,Box<dyn Error>> {
	let bn :BigInt = parse_to_bigint(s)?;
	Ok(BnGf2m::new_from_bigint(&bn))
}

fn binbnload_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");

	init_log(ns.clone())?;

	for v in sarr.iter() {
		let bn :BigInt = parse_to_bigint(v)?;
		let (_,vv) = bn.to_bytes_be();
		let bebn :BnGf2m = BnGf2m::new_from_be(&vv);
		let lebn :BnGf2m = BnGf2m::new_from_le(&vv);
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
	let aval :BnGf2m = get_bngf2m(&sarr[0])?;
	let bval :BnGf2m = get_bngf2m(&sarr[1])?;

	let cval :BnGf2m = &aval + &bval;
	println!("0x{:x} + 0x{:x} = 0x{:x}",aval,bval,cval);

	Ok(())
}

fn binmul_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");

	init_log(ns.clone())?;

	if sarr.len() < 2 {
		extargs_new_error!{BinError,"need anum and bnum"}
	}
	let aval :BnGf2m = get_bngf2m(&sarr[0])?;
	let bval :BnGf2m = get_bngf2m(&sarr[1])?;

	let cval :BnGf2m = &aval * &bval;
	let mut cformat :String = format!("{:X}",cval);
	if (cformat.len() % 2) != 0 {
		cformat = format!("0{}",cformat);
	}
	println!("0x{:x} * 0x{:x} = 0x{}",aval,bval,cformat);

	Ok(())
}

fn binmod_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");

	init_log(ns.clone())?;

	if sarr.len() < 2 {
		extargs_new_error!{BinError,"need anum and pnum"}
	}
	let aval :BnGf2m = get_bngf2m(&sarr[0])?;
	let pval :BnGf2m = get_bngf2m(&sarr[1])?;

	let pnum :BigInt = pval.to_bigint();
	let ov :BigInt = one();
	if (pnum.clone() & ov.clone()) != ov.clone() {
		extargs_new_error!{BinError," 0x{:x} not odd pnum",pnum}
	}

	let cval :BnGf2m = &aval % &pval;
	let mut cformat :String = format!("{:X}",cval);
	if (cformat.len() % 2) != 0 {
		cformat = format!("0{}",cformat);
	}
	println!("0x{:x} % 0x{:x} = 0x{}",aval,pval,cformat);

	Ok(())
}

fn binlshift_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");

	init_log(ns.clone())?;

	if sarr.len() < 2 {
		extargs_new_error!{BinError,"need anum and pnum"}
	}
	let aval :BnGf2m = get_bngf2m(&sarr[0])?;
	let shiftnum :u64 = parse_u64(&sarr[1])?;


	let cval :BnGf2m = &aval << (shiftnum as i32);
	println!("0x{:x} << {} = 0x{:x}", aval,shiftnum,cval);

	Ok(())
}

fn binrshift_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");

	init_log(ns.clone())?;

	if sarr.len() < 2 {
		extargs_new_error!{BinError,"need anum and pnum"}
	}
	let aval :BnGf2m = get_bngf2m(&sarr[0])?;
	let shiftnum :u64 = parse_u64(&sarr[1])?;


	let cval :BnGf2m = &aval >> (shiftnum as i32);
	println!("0x{:x} >> {} = 0x{:x}", aval,shiftnum,cval);

	Ok(())
}

fn bindiv_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");

	init_log(ns.clone())?;

	if sarr.len() < 2 {
		extargs_new_error!{BinError,"need anum and pnum"}
	}
	let aval :BnGf2m = get_bngf2m(&sarr[0])?;
	let pval :BnGf2m = get_bngf2m(&sarr[1])?;

	let pnum :BigInt = pval.to_bigint();
	let ov :BigInt = one();
	if (pnum.clone() & ov.clone()) != ov.clone() {
		extargs_new_error!{BinError," 0x{:x} not odd pnum",pnum}
	}

	let cval :BnGf2m = &aval / &pval;
	let mut cformat :String = format!("{:X}",cval);
	if (cformat.len() % 2) != 0 {
		cformat = format!("0{}",cformat);
	}
	println!("0x{:x} / 0x{:x} = 0x{}",aval,pval,cformat);

	Ok(())
}

fn bininv_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");

	init_log(ns.clone())?;

	if sarr.len() < 2 {
		extargs_new_error!{BinError,"need anum and pnum"}
	}
	let aval :BnGf2m = get_bngf2m(&sarr[0])?;
	let pval :BnGf2m = get_bngf2m(&sarr[1])?;

	let pnum :BigInt = pval.to_bigint();
	let ov :BigInt = one();
	if (pnum.clone() & ov.clone()) != ov.clone() {
		extargs_new_error!{BinError," 0x{:x} not odd pnum",pnum}
	}

	let cval :BnGf2m = aval.inv_op(&pval)?;
	println!("0x{:x} = 1 /  0x{:x} % 0x{:x}",cval,aval,pval);

	Ok(())
}

fn randpriv_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");
	let mut bits :u64 = 163;
	let mut top : i32 = -1;
	let mut bottom :i32 = 0;
	let mut num :i32 = 1;
	let mut idx :i32 = 0;
	let mut rn :BigInt;

	init_log(ns.clone())?;

	if sarr.len() > 0 {
		bits = s_atoi(&sarr[0])? as u64;
	}
	if sarr.len() > 1 {
		top = s_atoi(&sarr[1])?;
	}
	if sarr.len() > 2 {
		bottom = s_atoi(&sarr[2])?;
	}
	if sarr.len() > 3 {
		num = s_atoi(&sarr[3])?;
	}

	while idx < num {
		rn = ecsimple_rand_bits(bits,top,bottom);
		println!("{} 0x{:X}", idx, rn);
		idx += 1;
	}


	Ok(())
}

fn randmod_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");
	let order :BigInt;
	let mut num :i32 = 1;
	let mut idx :i32 = 0;

	init_log(ns.clone())?;

	if sarr.len() < 1 {
		extargs_new_error!{BinError,"need order at least"}
	}

	order = parse_to_bigint(&sarr[0])?;
	if sarr.len() > 1 {
		num = s_atoi(&sarr[1])?;
	}


	while idx < num {
		let rn = ecsimple_rand_range(&order);
		println!("{} 0x{:X}", idx, rn);
		idx += 1;
	}

	Ok(())
}

fn bnmodpow_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");
	let anum :BigInt;
	let cnum :BigInt;
	let pnum :BigInt;

	init_log(ns.clone())?;

	if sarr.len() < 3 {
		extargs_new_error!{BinError,"need anum enum pnum"}
	}

	anum = parse_to_bigint(&sarr[0])?;
	cnum = parse_to_bigint(&sarr[1])?;
	pnum = parse_to_bigint(&sarr[2])?;

	let cval :BigInt = anum.modpow(&cnum,&pnum);
	println!("cval 0x{:X} = anum 0x{:X} ^ cnum 0x{:X} % pnum 0x{:X}", cval,anum,cnum,pnum);

	Ok(())
}

fn bndivmod_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");

	init_log(ns.clone())?;

	if sarr.len() < 3 {
		extargs_new_error!{BinError,"need anum bnum and pnum"}
	}
	let aval :BnGf2m = get_bngf2m(&sarr[0])?;
	let bval :BnGf2m = get_bngf2m(&sarr[1])?;
	let pval :BnGf2m = get_bngf2m(&sarr[2])?;


	let cval :BnGf2m = bval.inv_op(&pval)?;
	println!("0x{:X} * 0x{:X} = 1 % 0x{:X}", cval,bval,pval);

	let dval :BnGf2m = cval.mul_op(&aval).mod_op(&pval);
	println!("0x{:X} = ( 0x{:X}  * 0x{:X} ) % 0x{:X}",dval,cval,aval,pval );
	println!("BN_GF2m_mod_div(0x{:X},0x{:X},0x{:X},0x{:X})",dval,aval,bval,pval);
	Ok(())
}


fn bnquadmod_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");

	init_log(ns.clone())?;

	if sarr.len() < 2 {
		extargs_new_error!{BinError,"need anum and pnum"}
	}
	let aval :BnGf2m = get_bngf2m(&sarr[0])?;
	let pval :BnGf2m = get_bngf2m(&sarr[1])?;


	let cval :BnGf2m = aval.sqrt_quad_op(&pval)?;
	println!("BN_GF2m_mod_solve_quad(0x{:X},0x{:X},0x{:X})",cval,aval,pval);
	Ok(())
}

fn montto_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");

	init_log(ns.clone())?;

	if sarr.len() < 2 {
		extargs_new_error!{BinError,"need anum and pnum"}
	}
	let aval :BigInt = parse_to_bigint(&sarr[0])?;
	let pval :BigInt = parse_to_bigint(&sarr[1])?;

	let montv :MontNum = MontNum::new(&pval)?;
	let cval :BigInt = montv.mont_to(&aval);
	println!("BN_to_montgomery(0x{:X},0x{:X},0x{:X})",cval,aval,pval);
	Ok(())
}

fn montfrom_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");

	init_log(ns.clone())?;

	if sarr.len() < 2 {
		extargs_new_error!{BinError,"need anum and pnum"}
	}
	let aval :BigInt = parse_to_bigint(&sarr[0])?;
	let pval :BigInt = parse_to_bigint(&sarr[1])?;

	let montv :MontNum = MontNum::new(&pval)?;
	let cval :BigInt = montv.mont_from(&aval);
	println!("BN_from_montgomery(0x{:X},0x{:X},0x{:X})",cval,aval,pval);
	Ok(())
}

fn montmul_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");

	init_log(ns.clone())?;

	if sarr.len() < 3 {
		extargs_new_error!{BinError,"need anum bnum and pnum"}
	}
	let aval :BigInt = parse_to_bigint(&sarr[0])?;
	let bval :BigInt = parse_to_bigint(&sarr[1])?;
	let pval :BigInt = parse_to_bigint(&sarr[2])?;

	let montv :MontNum = MontNum::new(&pval)?;
	let cval :BigInt = montv.mont_mul(&aval,&bval);
	println!("BN_mod_mul_montgomery(0x{:X},0x{:X},0x{:X},0x{:X})",cval,aval,bval,pval);
	Ok(())
}

fn montpow_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");

	init_log(ns.clone())?;

	if sarr.len() < 3 {
		extargs_new_error!{BinError,"need anum enum and pnum"}
	}
	let aval :BigInt = parse_to_bigint(&sarr[0])?;
	let eval :BigInt = parse_to_bigint(&sarr[1])?;
	let pval :BigInt = parse_to_bigint(&sarr[2])?;

	let montv :MontNum = MontNum::new(&pval)?;
	let cval :BigInt = montv.mont_pow(&aval,&eval);
	println!("BN_mod_exp_mont(0x{:X},0x{:X},0x{:X},0x{:X})",cval,aval,eval,pval);
	Ok(())
}

fn wnaf_value(bn :&BigInt,shifti :i32) -> Result<Vec<u8>,Box<dyn Error>> {
	let mut retv :Vec<u8> = Vec::new();
	let zv :BigInt = zero();
	let mut cv :BigInt = bn.clone();
	let ov :BigInt = one();
	let tv :BigInt = ov.clone() + ov.clone();
	let maskv :BigInt = ov.clone() << shifti;
	while cv > zv {
		let dv :u8;

		if (cv.clone() % tv.clone()) != zv {
			let dc = cv.clone() % maskv.clone();
			let (_,vv) = dc.to_bytes_le();
			dv = vv[0];
		} else {
			dv = 0;
		}
		retv.push(dv);
		cv = cv.clone() >> 1;		
	}
	Ok(retv)
}

fn wnaf_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");

	init_log(ns.clone())?;

	if sarr.len() < 2 {
		extargs_new_error!{BinError,"need anum enum and pnum"}
	}
	let aval :BigInt = parse_to_bigint(&sarr[0])?;
	let shifval :BigInt = parse_to_bigint(&sarr[1])?;
	let (_,bvecs) = shifval.to_bytes_le();
	let mut idx :usize = 0;
	let mut shifti :i32 = 0;
	while idx < bvecs.len() && idx < 4 {
		shifti |= (bvecs[idx] as i32) << (idx * 8);
		idx += 1;
	}
	let retv :Vec<u8> = wnaf_value(&aval,shifti)?;
	debug_buffer_trace!(retv.as_ptr(),retv.len(),"to 0x{:X} 0x{:x} wnaf",aval,shifti);
	Ok(())
}

#[extargs_map_function(binbnload_handler,binadd_handler,binmul_handler,binmod_handler,binlshift_handler,binrshift_handler,bindiv_handler,bininv_handler,randpriv_handler,randmod_handler,randmod_handler,bnmodpow_handler,bndivmod_handler,bnquadmod_handler,montto_handler,montfrom_handler,montmul_handler,montpow_handler,wnaf_handler)]
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
		},
		"binmod<binmod_handler>## anum % pnum in bin mode ##" : {
			"$" : "+"
		},
		"binlshift<binlshift_handler>##anum shiftnum in left shift##" : {
			"$" : "+"
		},
		"binrshift<binrshift_handler>##anum shiftnum in right shift##" : {
			"$" : "+"
		},
		"bindiv<bindiv_handler>##anum / bnum to bin divide##" : {
			"$" : "+"
		},
		"bininv<bininv_handler>##anum modnum to bin divide##" : {
			"$" : "+"
		},
		"randpriv<randpriv_handler>##[bits] [top] [bottom] [num] to default bits 163 top -1 bottom 0 num 1##" : {
			"$" : "*"
		},
		"randmod<randmod_handler>##rangevalue to set range value##" : {
			"$" : "+"
		},
		"bnmodpow<bnmodpow_handler>##anum cnum pnum to anum.modpow(cnum,pnum)##" : {
			"$" : 3
		},
		"bndivmod<bndivmod_handler>##anum bnum pnum to (anum / bnum) % pnum for BnGf2m##" : {
			"$" : 3
		},
		"bnquadmod<bnquadmod_handler>##anum pnum to get BN_GF2m_mod_solve_quad for BnGf2m##" : {
			"$" : 2
		},
		"montto<montto_handler>##anum pnum to mimic for BN_to_montgomery(r,a,p)##" : {
			"$" : 2
		},
		"montfrom<montfrom_handler>##anum pnum to mimic for BN_from_montgomery(r,a,p)##" : {
			"$" : 2
		},
        "montmul<montmul_handler>##anum bnum pnum for BN_mod_mul_montgomery##" : {
            "$" : 3
        },
        "montpow<montpow_handler>##anum enum pnum for BN_mod_exp_mont##" : {
        	"$" : 3
        },
        "wnaf<wnaf_handler>##anum wnum for bn_compute_wNAF##" : {
        	"$" : 2
        }
	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}