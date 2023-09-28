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

#[allow(unused_imports)]
use super::loglib::*;
#[allow(unused_imports)]
use super::fileop::*;
#[allow(unused_imports)]
use super::strop::*;
#[allow(unused_imports)]
use super::*;
use super::pemlib::*;
#[allow(unused_imports)]
use std::io::Write;

//use num_bigint::{BigInt,Sign};

//use ecsimple::group::{ECGroupPrime,get_prime_group_curve};
//use ecsimple::group::{ECGroup,ecc_get_curve_group};
//use ecsimple::signature::{ECSignature};
//use ecsimple::keys::{ECPublicKey, ECPrivateKey};
//use ecsimple::consts::*;

#[allow(unused_imports)]
use asn1obj_codegen::*;
#[allow(unused_imports)]
use asn1obj::base::*;
use asn1obj::complex::*;
use asn1obj::strop::*;
use asn1obj::asn1impl::*;
#[allow(unused_imports)]
use asn1obj::{asn1obj_error_class,asn1obj_new_error};

extargs_error_class!{EcAsn1Error}

#[derive(Clone)]
#[asn1_sequence()]
pub struct X9_62_PENTANOMIALELem {
	pub k1 :Asn1BigNum,
	pub k2 :Asn1BigNum,
	pub k3 :Asn1BigNum,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct X9_62_PENTANOMIAL {
	pub elem :Asn1Seq<X9_62_PENTANOMIALELem>,
}

#[derive(Clone)]
#[asn1_obj_selector(other=default,onBasis="1.2.840.10045.1.2.3.1",tpBasis="1.2.840.10045.1.2.3.2",ppBasis="1.2.840.10045.1.2.3.3")]
pub struct X962Selector  {
	pub val :Asn1Object,
}

#[derive(Clone)]
#[asn1_choice(selector=otype)]
pub struct X9_62_CHARACTERISTIC_TWO_ELEM_CHOICE {
	pub otype : X962Selector,
	pub onBasis : Asn1Null,
	pub tpBasis : Asn1BigNum,
	pub ppBasis : X9_62_PENTANOMIAL,
	pub other :Asn1Any,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct X9_62_CHARACTERISTIC_TWO_ELEM {
	pub m :Asn1BigNum,
	pub elemchoice : X9_62_CHARACTERISTIC_TWO_ELEM_CHOICE,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct X9_62_CHARACTERISTIC_TWO {
	pub elem :Asn1Seq<X9_62_CHARACTERISTIC_TWO_ELEM>,
}


#[derive(Clone)]
#[asn1_obj_selector(prime="1.2.840.10045.1.1",char_two="1.2.840.10045.1.2")]
pub struct X964FieldSelector {
	pub val :Asn1Object,
}

#[derive(Clone)]
#[asn1_choice(selector=fieldType)]
pub struct X9_62_FIELDIDElem {
	pub fieldType :X964FieldSelector,
	pub prime : Asn1BigNum,
	pub char_two :X9_62_CHARACTERISTIC_TWO,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct X9_62_FIELDID {
	pub elem :Asn1Seq<X9_62_FIELDIDElem>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct X9_62_CURVEElem {
	pub a :Asn1OctData,
	pub b :Asn1OctData,
	pub seed :Asn1Opt<Asn1BitDataFlag>,
}


#[derive(Clone)]
#[asn1_sequence()]
pub struct X9_62_CURVE {
	pub elem :Asn1Seq<X9_62_CURVEElem>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct ECPARAMETERSElem {
	pub version : Asn1Integer,
	pub fieldID : X9_62_FIELDID,
	pub curve :X9_62_CURVE,
	pub base :Asn1OctData,
	pub order :Asn1BigNum,
	pub cofactor : Asn1Opt<Asn1BigNum>,

}

#[derive(Clone)]
#[asn1_sequence()]
pub struct ECPARAMETERS {
	pub elem :Asn1Seq<ECPARAMETERSElem>,
}

#[asn1_int_choice(debug=0,selector=itype,named_curve=0,parameters=1,implicitCA=2)]
#[derive(Clone)]
pub struct ECPKPARAMETERS {
	pub itype :i32,
	pub named_curve :Asn1Object,
	pub parameters : ECPARAMETERS,
	pub implicitCA : Asn1Null,
}


#[derive(Clone)]
#[asn1_sequence()]
pub struct ECPrivateKeyAsn1Elem {
	pub version :Asn1Integer,
	pub privkey :Asn1OctData,
	pub paramters :Asn1Opt<Asn1ImpSet<ECPKPARAMETERS,0>>,
	pub pubkey : Asn1ImpSet<Asn1BitDataFlag,1>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct ECPrivateKeyAsn1 {
	pub elem :Asn1Seq<ECPrivateKeyAsn1Elem>,
}


#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1X509AttributeElem {
	pub object :Asn1Object,
	pub set :Asn1Any,
}

//#[asn1_sequence(debug=enable)]
#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1X509Attribute {
	pub elem : Asn1Seq<Asn1X509AttributeElem>,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1X509AlgorElem {
	pub algorithm : Asn1Object,
	pub parameters : Asn1Opt<Asn1Any>,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1X509Algor {
	pub elem : Asn1Seq<Asn1X509AlgorElem>,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1Pkcs8PrivKeyInfoElem {
	pub version :Asn1Integer,
	pub pkeyalg : Asn1X509Algor,
	pub pkey : Asn1OctData,
	pub attributes : Asn1Opt<Asn1ImpSet<Asn1X509Attribute,0>>,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1Pkcs8PrivKeyInfo {
	pub elem : Asn1Seq<Asn1Pkcs8PrivKeyInfoElem>,
}


fn ecprivkeydec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String>;
	let mut sout = std::io::stdout();
	sarr = ns.get_array("subnargs");
	if sarr.len() < 1 {
		extargs_new_error!{EcAsn1Error,"no file specified"}
	}

	for f in sarr.iter() {
		let data = read_file_into_der(f)?;
		let mut privkey :ECPrivateKeyAsn1 = ECPrivateKeyAsn1::init_asn1();
		let ores = privkey.decode_asn1(&data);
		if ores.is_err() {
			let mut pk8info :Asn1Pkcs8PrivKeyInfo = Asn1Pkcs8PrivKeyInfo::init_asn1();
			let _ = pk8info.decode_asn1(&data)?;
			if pk8info.elem.val.len() != 1 {
				extargs_new_error!{EcAsn1Error,"not valid val.len [{}]",pk8info.elem.val.len()}
			}
			let _ = privkey.decode_asn1(&pk8info.elem.val[0].pkey.data)?;
			pk8info.print_asn1("Asn1Pkcs8PrivKeyInfo",0,&mut sout)?;
		}
		privkey.print_asn1("ECPrivateKeyAsn1",0,&mut sout)?;
	}
	Ok(())
}



#[extargs_map_function(ecprivkeydec_handler)]
pub fn ec_asn1_parser(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = format!(r#"
	{{
		"ecprivdec<ecprivkeydec_handler>##ecfile ... to decode ecprivkey##" : {{
			"$" : "+"
		}}
	}}
	"#);
	extargs_load_commandline!(parser,&cmdline)?;
	Ok(())
}