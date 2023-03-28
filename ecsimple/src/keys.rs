
use crate::*;
use num_bigint::{BigInt,Sign,BigUint};
use crate::consts::*;
use crate::arithmetics::*;
use crate::utils::*;
use crate::jacobi::{PointJacobi,ECCPoint,CurveFp};
use crate::curves::*;
use crate::signature::*;
use std::error::Error;
use num_traits::{zero,one};
use crate::logger::*;

//use rand::RngCore;

#[allow(unused_imports)]
use asn1obj_codegen::{asn1_choice,asn1_obj_selector,asn1_sequence,asn1_int_choice};
use asn1obj::complex::*;
use asn1obj::strop::*;
use asn1obj::base::*;
use asn1obj::*;
use asn1obj::asn1impl::*;
use std::io::Write;


ecsimple_error_class!{EccKeyError}


#[derive(Clone)]
#[asn1_sequence()]
pub struct ECPublicKeyAbbrevElem {
	pub types :Asn1Object,
	pub ectypes :Asn1Object,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct ECPublicKeyAbbrev {
	pub elem :Asn1Seq<ECPublicKeyAbbrevElem>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct ECPublicKeyFieldIDElem {
	pub types :Asn1Object,
	pub primenum :Asn1BigNum,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct ECPublicKeyFieldID {
	pub elem :Asn1Seq<ECPublicKeyFieldIDElem>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct ECPublicKeyCurveElem {
	pub a :Asn1OctData,
	pub b :Asn1OctData,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct ECPublicKeyCurve {
	pub elem :Asn1Seq<ECPublicKeyCurveElem>,
}


#[derive(Clone)]
#[asn1_sequence()]
pub struct ECPublicKeyParamsElem {
	pub version :Asn1Integer,
	pub fieldid :ECPublicKeyFieldID,
	pub curve :ECPublicKeyCurve,
	pub basecoords :Asn1OctData,
	pub order :Asn1BigNum,
	pub cofactor :Asn1BigNum,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct ECPublicKeyParams {
	pub elem :Asn1Seq<ECPublicKeyParamsElem>,
}


#[derive(Clone)]
#[asn1_sequence()]
pub struct ECPublicKeyTotalElem {
	pub types :Asn1Object,
	pub ecparams :ECPublicKeyParams,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct ECPublicKeyTotal {
	pub elem :Asn1Seq<ECPublicKeyTotalElem>,
}

#[derive(Clone)]
#[asn1_int_choice(selector=typei,abbrev=1,total=2)]
pub struct ECPublicKeyChoiceElem {
	pub typei :i32,
	pub abbrev :ECPublicKeyAbbrev,
	pub total :ECPublicKeyTotal,
}

#[derive(Clone)]
#[asn1_int_choice(selector=typei,simple=1,total=2)]
pub struct ECPublicKeySimpChoiceElem {
	pub typei :i32,
	pub simple :Asn1Object,
	pub total :ECPublicKeyParams,
}



#[derive(Clone)]
#[asn1_sequence()]
pub struct ECPublicKeyAsn1Elem {
	pub params :ECPublicKeyChoiceElem,
	pub coords :Asn1BitData,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct ECPublicKeyAsn1 {
	pub elem :Asn1Seq<ECPublicKeyAsn1Elem>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct ECPrivateKeyAsn1Elem {
	pub version :Asn1Integer,
	pub privkey :Asn1OctData,
	pub pubkey :Asn1Imp<ECPublicKeySimpChoiceElem,0>,
	pub pubdata :Asn1Imp<Asn1BitData,1>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct ECPrivateKeyAsn1 {
	pub elem :Asn1Seq<ECPrivateKeyAsn1Elem>,
}

#[derive(Clone,Debug)]
pub struct PublicKey {
	pub curve :ECCCurve,
	pub pubkey :PointJacobi,
}


fn _from_der_x_y_uncompressed(_curve :&CurveFp,data :&[u8]) -> Result<(BigInt,BigInt),Box<dyn Error>> {
	let x :BigInt;
	let y :BigInt;

	if data.len() < 2 {
		ecsimple_new_error!{EccKeyError,"len [{}] < 2", data.len()}
	}


	let midlen :usize = (data.len() -1 ) / 2;
	let vecs :Vec<u8> = data[1..(1 + midlen)].to_vec().clone();
	x = BigInt::from_bytes_be(Sign::Plus,&vecs);
	let vecs :Vec<u8> = data[(1+midlen)..].to_vec().clone();
	y = BigInt::from_bytes_be(Sign::Plus,&vecs);
	Ok((x,y))
}

fn _from_der_x_y_hybrid(_curve :&CurveFp,data :&[u8]) -> Result<(BigInt,BigInt),Box<dyn Error>> {
	let x :BigInt;
	let y :BigInt;
	let ov :BigInt = one();
	let tv :BigInt = &ov + &ov;

	if data.len() < 2 {
		ecsimple_new_error!{EccKeyError,"len [{}] < 2", data.len()}
	}

	let midlen :usize = (data.len() -1 ) / 2;
	let vecs :Vec<u8> = data[1..(1 + midlen)].to_vec().clone();
	x = BigInt::from_bytes_be(Sign::Plus,&vecs);
	let vecs :Vec<u8> = data[(1+midlen)..].to_vec().clone();
	y = BigInt::from_bytes_be(Sign::Plus,&vecs);

	if data[0] == 0x7 && ((&y) & (&tv)) != ov {
		ecsimple_new_error!{EccKeyError,"y [0x{:x}] not odd", y}
	} else if data[0] == 0x6 && ((&y) & (&tv)) == ov {
		ecsimple_new_error!{EccKeyError,"y [0x{:x}] not even", y}
	}

	Ok((x,y))
}

fn _from_der_x_y_compressed(curve :&CurveFp,data :&[u8]) -> Result<(BigInt,BigInt),Box<dyn Error>> {
	let x :BigInt;
	let y :BigInt;
	let ov :BigInt = one();
	let tv :BigInt = &ov + &ov;
	let threev :BigInt = &tv + &ov;

	if data.len() < 2 {
		ecsimple_new_error!{EccKeyError,"len [{}] < 2", data.len()}
	}

	let vecs :Vec<u8> = data[1..data.len()].to_vec().clone();
	x = BigInt::from_bytes_be(Sign::Plus,&vecs);
	let p = curve.p();
	let a = curve.a();
	let b = curve.b();
	let y2 = ((x.modpow(&threev,&p) + (&a) * (&x)) + (&b)) % (&p);
	y = square_root_mod_prime(&y2,&p)?;


	ecsimple_log_trace!(" data[0] 0x{:x} x 0x{:x} y 0x{:x}", data[0],x,y);
	if data[0] == 0x3 && (((&y) % (&tv)) != ov ){
		ecsimple_new_error!{EccKeyError,"y [0x{:x}] not odd", y}
	} else if data[0] == 0x2 && (((&y) % (&tv)) == ov ) {
		ecsimple_new_error!{EccKeyError,"y [0x{:x}] not even", y}
	}

	Ok((x,y))
}


fn _from_der_x_y(curve :&CurveFp,data :&[u8]) -> Result<(BigInt,BigInt),Box<dyn Error>> {
	if data.len() < 1 {
		ecsimple_new_error!{EccKeyError,"data len [{}] < 1" , data.len()}
	}
	if data[0] == 0x4 {
		return _from_der_x_y_uncompressed(curve,data);
	} else if data[0] == 0x2 || data[0] == 0x3 {
		return _from_der_x_y_compressed(curve,data);
	} else if data[0] == 0x7 || data[0] == 0x6 {
		return _from_der_x_y_hybrid(curve,data);
	}
	ecsimple_new_error!{EccKeyError,"not supported type [0x{:x}]", data[0]}
}


fn _to_der_compressed(x:&BigInt, y :&BigInt) -> Result<Vec<u8>,Box<dyn Error>> {
	let mut retv :Vec<u8> = Vec::new();
	let zv :BigInt = zero();
	let ov :BigInt = one();
	ecsimple_log_trace!("x 0x{:x} y 0x{:x}", x,y);
	if (y & &ov) != zv {
		retv.push(0x3);
	} else {
		retv.push(0x2);
	}
	let (_,vecs) = x.to_bytes_be();
	retv.extend(vecs);

	Ok(retv)
}


fn _to_der_uncompressed(x:&BigInt, y :&BigInt) -> Result<Vec<u8>,Box<dyn Error>> {
	let mut retv :Vec<u8> = Vec::new();
	retv.push(0x4);
	let (_,vecs) = x.to_bytes_be();
	retv.extend(vecs);
	let (_,vecs) = y.to_bytes_be();
	retv.extend(vecs);
	Ok(retv)
}

fn _to_der_hybrid(x:&BigInt, y :&BigInt) -> Result<Vec<u8>,Box<dyn Error>> {
	let mut retv :Vec<u8> = Vec::new();
	let zv :BigInt = zero();
	let ov :BigInt = one();
	if (y & &ov) != zv {
		retv.push(0x7);
	} else {
		retv.push(0x6);
	}
	let vecs = _to_der_uncompressed(x,y)?;
	retv.extend(vecs);

	Ok(retv)		
}

fn _to_der_x_y(types :&str,x :&BigInt, y :&BigInt) -> Result<Vec<u8>,Box<dyn Error>> {
	if types == EC_COMPRESSED {
		return  _to_der_compressed(&x,&y);
	} else if types == EC_UNCOMPRESSED {
		return  _to_der_uncompressed(&x,&y);
	} else if types == EC_HYBRID {
		return _to_der_hybrid(&x,&y);
	} 
	ecsimple_new_error!{EccKeyError,"not valid types [{}]",types}
}


#[allow(non_snake_case)]
impl PublicKey {
	pub fn new(curve :&ECCCurve, pt :&ECCPoint) -> Result<Self,Box<dyn Error>> {
		Ok(PublicKey {
			curve :curve.clone(),
			pubkey : PointJacobi::from_affine(pt,false),
		})
	}


	pub fn from_der(buf :&[u8]) -> Result<Self,Box<dyn Error>> {
		let mut pubkasn1 :ECPublicKeyAsn1 = ECPublicKeyAsn1::init_asn1();
		let _ = pubkasn1.decode_asn1(buf)?;
		let curveelem :ECPublicKeyChoiceElem;
		let mut pubk :PointJacobi;
		if pubkasn1.elem.val.len() != 1 {
			ecsimple_new_error!{EccKeyError,"not pubkasn1 [{}] != 1" , pubkasn1.elem.val.len()}
		}
		let pubkelem = pubkasn1.elem.val[0].clone();

		let curve :ECCCurve;
		curveelem = pubkelem.params.clone();
		if curveelem.typei == 1 {
			let abbrevelem :ECPublicKeyAbbrevElem ;
			if curveelem.abbrev.elem.val.len() != 1 {
				ecsimple_new_error!{EccKeyError,"not abbrev [{}] != 1" , curveelem.abbrev.elem.val.len()}
			}
			abbrevelem = curveelem.abbrev.elem.val[0].clone();

			let oids = abbrevelem.ectypes.get_value();
			let types = get_ecc_name_by_oid(&oids)?;
			curve = get_ecc_curve_by_name(&types)?;
			ecsimple_log_trace!("[{}] curve generator {:?}", types, curve.generator);
			let (x,y) = _from_der_x_y(&(curve.curve),&pubkelem.coords.data)?;
			pubk = curve.generator.clone();
			let _ = pubk.set_x_y(&x,&y)?;
		} else {
			let totalelem :ECPublicKeyTotalElem;
			if curveelem.total.elem.val.len() != 1 {
				ecsimple_new_error!{EccKeyError,"not total [{}] != 1" , curveelem.total.elem.val.len()}	
			}
			totalelem = curveelem.total.elem.val[0].clone();
			let oids = totalelem.types.get_value();
			if oids != EC_PUBLIC_KEY_OID {
				ecsimple_new_error!{EccKeyError,"type oid [{}] != EC_PUBLIC_KEY_OID [{}]", oids,EC_PUBLIC_KEY_OID}
			}
			let ecparams :ECPublicKeyParamsElem;
			if totalelem.ecparams.elem.val.len() != 1 {
				ecsimple_new_error!{EccKeyError,"ecparams  [{}] != 1" , totalelem.ecparams.elem.val.len()}		
			}
			ecparams = totalelem.ecparams.elem.val[0].clone();
			if ecparams.version.val != 1 {
				ecsimple_new_error!{EccKeyError,"version [{}] != 1",ecparams.version.val}
			}
			let fieldidelem :ECPublicKeyFieldIDElem;
			if ecparams.fieldid.elem.val.len() != 1 {
				ecsimple_new_error!{EccKeyError,"fieldid  [{}] != 1" , ecparams.fieldid.elem.val.len()}
			}
			fieldidelem = ecparams.fieldid.elem.val[0].clone();
			let oids = fieldidelem.types.get_value();
			if oids != ID_PRIME_FIELD_OID {
				ecsimple_new_error!{EccKeyError,"type oid [{}] != ID_PRIME_FIELD_OID [{}]", oids,ID_PRIME_FIELD_OID}	
			}
			let vecs :Vec<u8> = fieldidelem.primenum.val.to_bytes_be();
			let p :BigInt = BigInt::from_bytes_be(Sign::Plus,&vecs);
			let curveparamelem :ECPublicKeyCurveElem;
			if ecparams.curve.elem.val.len() != 1 {
				ecsimple_new_error!{EccKeyError,"curve  [{}] != 1" , ecparams.curve.elem.val.len()}
			}
			curveparamelem = ecparams.curve.elem.val[0].clone();
			let vecs :Vec<u8> = curveparamelem.a.data.clone();
			let a :BigInt = BigInt::from_bytes_be(Sign::Plus,&vecs);
			let vecs :Vec<u8> = curveparamelem.b.data.clone();
			let b :BigInt = BigInt::from_bytes_be(Sign::Plus,&vecs);
			let vecs :Vec<u8> = ecparams.order.val.to_bytes_be();
			let order :BigInt = BigInt::from_bytes_be(Sign::Plus,&vecs);
			let vecs :Vec<u8> = ecparams.cofactor.val.to_bytes_be();
			let cofactor :BigInt = BigInt::from_bytes_be(Sign::Plus,&vecs);
			let ncurve :CurveFp = CurveFp::new(&p,&a,&b,&cofactor);
			let (x,y) = _from_der_x_y(&ncurve,&(ecparams.basecoords.data))?;
			let oo :Option<BigInt> = Some(order.clone());
			let njacobi :PointJacobi = PointJacobi::new(&ncurve,&x,&y,&cofactor,oo,false);
			curve = ECCCurve::new("",&njacobi);
			let (x,y) = _from_der_x_y(&ncurve,&pubkelem.coords.data)?;
			pubk = curve.generator.clone();
			let _ = pubk.set_x_y(&x,&y)?;
		}

		Ok(PublicKey {
			curve : curve.clone(),
			pubkey : pubk.clone(),
		})
	}



	pub fn to_der(&self,types :&str,paramstype :&str) -> Result<Vec<u8>,Box<dyn Error>> {
		let mut curveelem :ECPublicKeyChoiceElem = ECPublicKeyChoiceElem::init_asn1();
		let mut pubkasn1elem :ECPublicKeyAsn1Elem = ECPublicKeyAsn1Elem::init_asn1();
		let mut pubkasn1  :ECPublicKeyAsn1 = ECPublicKeyAsn1::init_asn1();
		let coordvecs :Vec<u8>;
		let oid :String;
		let typeec :String = format!("{}",self.curve.name);
		if typeec.len() != 0 && paramstype != EC_PARAMS_EXLICIT {
			oid = get_ecc_oid_by_name(&typeec)?;
			curveelem.typei = 1;
			let mut abbrevelem :ECPublicKeyAbbrevElem = ECPublicKeyAbbrevElem::init_asn1();
			let _ = abbrevelem.types.set_value(EC_PUBLIC_KEY_OID)?;
			let _ = abbrevelem.ectypes.set_value(&oid)?;
			let x = self.pubkey.x();
			let y = self.pubkey.y();
			coordvecs = _to_der_x_y(types,&x,&y)?;
			curveelem.abbrev.elem.val.push(abbrevelem);
		} else {
			/*now to give */
			let mut totalelem :ECPublicKeyTotalElem = ECPublicKeyTotalElem::init_asn1();
			let mut ecparams :ECPublicKeyParamsElem = ECPublicKeyParamsElem::init_asn1();
			let mut fieldid :ECPublicKeyFieldIDElem = ECPublicKeyFieldIDElem::init_asn1();
			let mut pubk :ECPublicKeyCurveElem = ECPublicKeyCurveElem::init_asn1();
			let _ = totalelem.types.set_value(EC_PUBLIC_KEY_OID)?;
			let x :BigInt = self.curve.generator.x();
			let y :BigInt = self.curve.generator.y();
			curveelem.typei = 2;
			ecparams.version.val = 1;
			let _ = fieldid.types.set_value(ID_PRIME_FIELD_OID)?;
			let (_ ,vecs) = self.curve.generator.curve().p().to_bytes_be();
			fieldid.primenum.val = BigUint::from_bytes_be(&vecs);
			ecparams.fieldid.elem.val.push(fieldid);
			let (_, vecs) = self.curve.curve.a().to_bytes_be();
			pubk.a.data = vecs.clone();
			let (_, vecs) = self.curve.curve.b().to_bytes_be();
			pubk.b.data = vecs.clone();
			ecparams.curve.elem.val.push(pubk);
			let vecs = _to_der_x_y(types,&x,&y)?;
			ecparams.basecoords.data = vecs.clone();
			let (_, vecs) = self.curve.order.to_bytes_be();
			ecparams.order.val = BigUint::from_bytes_be(&vecs);
			let vecs :Vec<u8> = vec![0x1];
			ecparams.cofactor.val = BigUint::from_bytes_be(&vecs);
			let x = self.pubkey.x();
			let y = self.pubkey.y();
			coordvecs = _to_der_x_y(types,&x,&y)?;
			totalelem.ecparams.elem.val.push(ecparams);
			curveelem.total.elem.val.push(totalelem);
		}

		pubkasn1elem.params = curveelem.clone();
		pubkasn1elem.coords.data = coordvecs.clone();
		pubkasn1.elem.val.push(pubkasn1elem);

		return pubkasn1.encode_asn1();
	}

	pub fn verify_digest(&self,hashcode :&[u8],sig :&ECCSignature) -> bool {
		return self.verify_base(hashcode,sig);
	}


	pub fn verify_base(&self,hashcode :&[u8],sig :&ECCSignature) -> bool {
		let mut G :PointJacobi = self.curve.generator.clone();
		let n :BigInt = G.order();
		let r :BigInt = sig.r.clone();
		let s :BigInt = sig.s.clone();
		let hash :BigInt = BigInt::from_bytes_be(Sign::Plus,hashcode);
		let mut pubkey :PointJacobi = self.pubkey.clone();

		ecsimple_log_trace!("G {:?}",G);
		if r < one() || r >= n {
			ecsimple_log_trace!("r 0x{:x} n 0x{:x}", r, n);
			return false;
		}

		if s < one() || s >= n {
			ecsimple_log_trace!(" ");
			return false;
		}
		let c :BigInt = inverse_mod(&s,&n);
		let u1 :BigInt = ((&hash) * (&c)) % (&n);
		let u2 :BigInt = ((&r) * (&c)) % (&n);
		let u1g :PointJacobi = G.mul_int(&u1);
		let u2g :PointJacobi = pubkey.mul_int(&u2);
		let xy :PointJacobi = u1g.add_jacobi(&u2g);
		let v :BigInt = xy.x() % (&n);
		if v == r {
			return true;
		}
		return false;
	}
}


#[derive(Clone)]
pub struct PrivateKey {
	curve :ECCCurve,
	keynum :BigInt,
	pubkey :PointJacobi,
	randname :Option<String>,
}

#[allow(non_snake_case)]
impl PrivateKey {
	pub fn generate(curve :&ECCCurve,fname :Option<String>) -> Result<Self,Box<dyn Error>> {
		let mut bname :Option<String> = None;
		let mut rname :Option<String> = None;
		if fname.is_some() {
			bname = Some(format!("{}",fname.as_ref().unwrap()));
			rname = Some(format!("{}",fname.as_ref().unwrap()));
		}
		let mut rdops = RandOps::new(bname)?;
		let bitlen :usize = bit_length(&curve.order);
		let bs :usize = (bitlen + 7) / 8;
		let vecs = rdops.get_bytes(bs)?;
		let knum :BigInt = BigInt::from_bytes_be(Sign::Plus,&vecs);
		let mut bptr :PointJacobi = curve.generator.clone();
		let pubkey :PointJacobi = bptr.mul_int(&knum);

		Ok(PrivateKey {
			curve : curve.clone(),
			keynum : knum.clone(),
			pubkey : pubkey,
			randname : rname,
		})
	}

	fn _get_ec_priv_simp(&self,types :&str,exps :&str) -> Result<ECPublicKeySimpChoiceElem,Box<dyn Error>> {
		let mut simpelem :ECPublicKeySimpChoiceElem = ECPublicKeySimpChoiceElem::init_asn1();
		if exps == EC_PARAMS_EXLICIT {
			simpelem.typei = 2;




		} else {
			simpelem.typei = 1;
			let oid = get_ecc_oid_by_name(&self.curve.name)?;
			let _ = simpelem.simple.set_value(&oid)?;
		}
		Ok(simpelem)
	}

	pub fn to_der(&self, types :&str, asn1s :&str , exps :&str) -> Result<Vec<u8>,Box<dyn Error>> {
		let mut privkey :ECPrivateKeyAsn1 = ECPrivateKeyAsn1::init_asn1();
		let mut privelem :ECPrivateKeyAsn1Elem = ECPrivateKeyAsn1Elem::init_asn1();
		privelem.version.val = 1;
		let (_, mut vecs) = self.keynum.to_bytes_be();
		let bitsize = bit_length(&self.curve.generator.order());
		let bs = (bitsize + 7 ) / 8;
		while vecs.len() < bs {
			vecs.insert(0,0 as u8);
		}

		privelem.privkey.data = vecs.clone();
		let simpelem = self._get_ec_priv_simp(types,exps)?;

		privkey.elem.val.push(privelem);
		if asn1s == EC_SSLEAY_TYPE{
			return privkey.encode_asn1();
		} 

		Ok(Vec::new())
	}

	pub fn set_rand_file(&mut self, fname :Option<String>) {		
		self.randname = None;
		if fname.is_some() {
			self.randname = Some(format!("{}",fname.as_ref().unwrap()));
		}
		return;
	}


	pub fn new(curve :&ECCCurve, secnum :&BigInt) -> Result<Self,Box<dyn Error >> {
		let bitlen :usize = bit_length(&curve.order);
		let (_ ,vecs) = secnum.to_bytes_be();
		let mut vlen :usize = 0;
		if vecs.len() > 0 {
			vlen = (vecs.len() - 1) * 8;
			let mut uv :u8 = vecs[0];
			while uv > 0 {
				vlen += 1;
				uv >>= 1;
			}
		}

		if vlen > bitlen {
			ecsimple_new_error!{EccKeyError,"secnum [{}] < order [{}]", vlen,bitlen}
		}
		let mut gen :PointJacobi = curve.generator.clone();
		let pubkey :PointJacobi = gen.mul_int(&secnum);
		Ok (PrivateKey {
			curve : curve.clone(),
			keynum : secnum.clone(),
			pubkey : pubkey,
			randname : None,
		})
	}

	pub fn sign_digest(&self, digestcode :&[u8]) -> Result<ECCSignature,Box<dyn Error>> {
		let mut bname :Option<String> = None;
		if self.randname.is_some() {
			bname = Some(format!("{}", self.randname.as_ref().unwrap()));
		}
		let mut rdops = RandOps::new(bname)?;
		let bitsize :usize = bit_length(&self.curve.generator.clone().order());
		let bs :usize = bitsize / 8;
		let vecs = rdops.get_bytes(bs)?;
		let randkey :BigInt = BigInt::from_bytes_be(Sign::Plus,&vecs);
		return self.sign_base(digestcode,&randkey);
	}


	pub fn sign_base(&self, hashcode :&[u8], randkey :&BigInt) -> Result<ECCSignature,Box<dyn Error>> {
		let n :BigInt;
		let mut G :PointJacobi = self.curve.generator.clone();
		n = G.order();
		let k :BigInt = randkey % (&n);
		let ks :BigInt = &k + &n;
		let kt :BigInt = &ks + &n;
		let p1 :PointJacobi;
		let r :BigInt;
		let s :BigInt;
		let hash :BigInt = BigInt::from_bytes_be(Sign::Plus,hashcode);

		if bit_length(&ks) == bit_length(&n) {
			p1 = G.mul_int(&kt);
		} else {
			p1 = G.mul_int(&ks);
		}

		r = p1.x() % (&n);
		if r == zero() {
			ecsimple_new_error!{EccKeyError,"randkey [{}] r zeroized", randkey}
		}
		s = (inverse_mod(&k,&n) * (((&hash) + &(self.keynum) * &r) % (&n) ) ) % (&n);
		if s == zero() {
			ecsimple_new_error!{EccKeyError,"randkey [{}] s zeroized", randkey}
		}
		ecsimple_log_trace!("r 0x{:x} s 0x{:x}",r, s);
		Ok (ECCSignature::new(&r,&s))
	}


	pub fn get_public_key(&self) -> PublicKey {
		PublicKey {
			curve : self.curve.clone(),
			pubkey : self.pubkey.clone(),
		}
	}
}