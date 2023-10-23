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
use ecsimple::utils::*;

use num_bigint::{BigInt,Sign};
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

	println!("will 0x{:x} >> {}", aval,shiftnum);
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

pub (crate) fn get_max_bits(bn :&BigInt) -> i64 {
	let mut retv : i64 = -1;
	let mut idx : i64 = 0 ;
	let zv :BigInt = zero();
	let mut cv : BigInt = one();

	while bn >= &cv {
		if (&cv & bn) != zv {
			/*for expand*/
			retv = idx + 1;
		}
		idx += 1;
		cv <<= 1;
	}
	return retv;
}


fn wnaf_value(bn :&BigInt,w :i32) -> Result<Vec<u8>,Box<dyn Error>> {
	let mut retv :Vec<u8> = Vec::new();
	let zv :BigInt = zero();
	let ov :BigInt = one();
	if w < 1 || w > 7 {
		panic!("w {} < 1 || > 7",w);
	}
	let bit :BigInt = ov.clone() << w;
	let next_bit :BigInt = bit.clone() << 1;
	let mask :BigInt = next_bit.clone() - ov.clone();
	let mut window_val :BigInt;
	let mut j :i32 = 0;
	let lenv :i32;
	window_val = bn.clone() & mask.clone();
	lenv = get_max_bits(bn) as i32;
	while window_val != zv || (j+ w + 1) < lenv {
		let mut digit : BigInt = zv.clone();
		if (window_val.clone() & ov.clone()) != zv {
			if (window_val.clone() & bit.clone()) != zv {
				digit = window_val.clone() - next_bit.clone();
				if (j + w + 1) >= lenv {
					digit = window_val.clone() & (mask.clone() >> 1);
				}
			} else {
				digit = window_val.clone();
			}
			if digit.clone() <= - bit.clone() || digit.clone() >=bit.clone() ||  (digit.clone() & ov.clone()) == zv {
				extargs_new_error!{BinError,"internal error on digit"}
			}
			window_val -= digit.clone();
		}
		let (_,vecs) = digit.to_bytes_le();
		if digit >= zv {
			retv.push(vecs[0]);		
		} else {
			retv.push((0xff - vecs[0] + 1) as u8);
		}
		
		j += 1;
		window_val = window_val.clone() >> 1;
		if (bn.clone() & (ov.clone() << (j + w))) != zv  {
			window_val += bit.clone();
		}

		if window_val > next_bit.clone() {
			extargs_new_error!{BinError,"window_val 0x{:X} > next_bit 0x{:X}", window_val,next_bit}
		}
	}

	if j > (lenv + 1) {
		extargs_new_error!{BinError,"j {} > lenv {} + 1",j,lenv}
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


fn bnusub(anum :&BigInt,bnum :&BigInt) -> BigInt {
	let retv :BigInt;
	if anum > bnum {
		retv = anum - bnum;
	} else {
		let curv :BigInt = anum - bnum;
		let mut curvecs :Vec<u8>;
		let mut maskvecs :Vec<u8> = Vec::new();
		let mut resvecs :Vec<u8> = Vec::new();
		let mut idx :usize;
		(_ , curvecs) = curv.to_bytes_le();
		while (curvecs.len() % 8) != 0 {
			curvecs.push(0);
		}

		while maskvecs.len() < curvecs.len() {
			maskvecs.push(0xff);
			resvecs.push(0);
		}
		idx = 0;
		while idx < maskvecs.len() {
			resvecs[idx] = maskvecs[idx] ^ curvecs[idx];
			idx += 1;
		}
		resvecs[0] += 1;
		retv = BigInt::from_bytes_le(Sign::Plus,&resvecs);
	}

	return retv;
}

fn bnusub_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");

	init_log(ns.clone())?;

	if sarr.len() < 2 {
		extargs_new_error!{BinError,"need anum enum and pnum"}
	}
	let aval :BigInt = parse_to_bigint(&sarr[0])?;
	let bval :BigInt = parse_to_bigint(&sarr[1])?;
	let rval :BigInt;
	rval = bnusub(&aval,&bval);
	println!("BN_usub(0x{:X},0x{:X},0x{:X})",rval,aval,bval);
	Ok(())
}

fn bnmodsqrt_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");

	init_log(ns.clone())?;

	if sarr.len() < 2 {
		extargs_new_error!{BinError,"need anum enum and pnum"}
	}
	let aval :BigInt = parse_to_bigint(&sarr[0])?;
	let bval :BigInt = parse_to_bigint(&sarr[1])?;
	let rval :BigInt;
	rval = mod_sqrt(&aval,&bval)?;
	println!("BN_mod_sqrt(0x{:X},0x{:X},0x{:X})",rval,aval,bval);
	Ok(())
}

#[extargs_map_function(binbnload_handler,binadd_handler,binmul_handler,binmod_handler,binlshift_handler,binrshift_handler,bindiv_handler,bininv_handler,randpriv_handler,randmod_handler,randmod_handler,bnmodpow_handler,bndivmod_handler,bnquadmod_handler,montto_handler,montfrom_handler,montmul_handler,montpow_handler,wnaf_handler,bnusub_handler,bnmodsqrt_handler)]
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
        },
        "bnusub<bnusub_handler>##anum bnum to call BN_usub(rval,anum,bnum)##" : {
        	"$" : 2
        },
        "bnmodsqrt<bnmodsqrt_handler>##anum pnum call BN_mod_sqrt(rval,anum,pnum)##" : {
        	"$" : 2
        }
	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}