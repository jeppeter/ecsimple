#[allow(unused_imports)]
use asn1obj_codegen::*;
#[allow(unused_imports)]
use asn1obj::base::*;
use asn1obj::complex::*;
use asn1obj::strop::*;
use asn1obj::asn1impl::*;
#[allow(unused_imports)]
use asn1obj::{asn1obj_error_class,asn1obj_new_error};

#[allow(unused_imports)]
use std::io::Write;
use std::error::Error;


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
	pub seed :Asn1Opt<Asn1BitData>,
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
	pub parameters :Asn1Opt<Asn1ImpSet<ECPKPARAMETERS,0>>,
	pub pubkey : Asn1ImpSet<Asn1BitData,1>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub (crate) struct ECPrivateKeyAsn1 {
	pub elem :Asn1Seq<ECPrivateKeyAsn1Elem>,
}