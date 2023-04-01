
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
	pub pubkey :Asn1ImpSet<ECPublicKeySimpChoiceElem,0>,
	pub pubdata :Asn1ImpSet<Asn1BitData,1>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct ECPrivateKeyAsn1 {
	pub elem :Asn1Seq<ECPrivateKeyAsn1Elem>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct ECPrivateKeySimpElem {
	pub version :Asn1Integer,
	pub secnum :Asn1OctData,
	pub pubcoords :Asn1ImpSet<Asn1BitData,1>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct ECPrivateKeySimp {
	pub elem :Asn1Seq<ECPrivateKeySimpElem>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct ECPrivateKeyPkcs8Elem {
	pub version :Asn1Integer,
	pub pubkey :ECPublicKeyChoiceElem,
	pub privdata :Asn1OctData,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct ECPrivateKeyPkcs8 {
	pub elem :Asn1Seq<ECPrivateKeyPkcs8Elem>,
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

#[derive(Clone,Debug)]
pub struct PublicKey {
	pub curve :ECCCurve,
	pub pubkey :PointJacobi,
	pub randname :Option<String>,
}

impl std::cmp::PartialEq<PublicKey> for PublicKey {
    fn eq(&self,other :&Self) -> bool {
        if self.curve != other.curve  || self.pubkey != other.pubkey  {
            return false;
        }
        return true;
    }

    fn ne(&self, other :&Self) -> bool {
        return  !self.eq(other);
    }
}

#[allow(non_snake_case)]
impl PublicKey {
	pub fn new(curve :&ECCCurve, pt :&ECCPoint) -> Result<Self,Box<dyn Error>> {
		Ok(PublicKey {
			curve :curve.clone(),
			pubkey : PointJacobi::from_affine(pt,false),
			randname : None,
		})
	}

	fn _ecnrypt_one_pack(&self,curval :&BigInt,randkey :&BigInt) -> Result<ECCSignature,Box<dyn Error>> {
		let mut generator :PointJacobi = self.curve.generator.clone();
		let mut pubpoint :PointJacobi = self.pubkey.clone();
		let mut rja :PointJacobi = generator.mul_int(randkey);
		let mut sja :PointJacobi = pubpoint.mul_int(randkey);

		let rs :ECCPoint = rja.to_affine();
		let ss :ECCPoint = sja.to_affine();

		let r :BigInt = rs.x();
		let s :BigInt = curval + ss.x();

		if r == zero() || s == zero() {
			ecsimple_new_error!{EccKeyError,"zero for r [0x{:x}] or s [0x{:x}]", r,s}
		}

		Ok(ECCSignature::new(&r,&s))
	}

	fn _splice_pack(&self,rdata :&[u8],maxsize :usize) -> Result<(BigInt,usize),Box<dyn Error>> {
		/*
		for packet 
		[0] = 

		*/
		let mut rdsize :usize = maxsize - 3;
		let mut sizeb :usize = 1;
		let mut curi :usize;
		let mut idx :usize;
		assert!(maxsize < EC_ENC_DATA_4_BYTE_MAX);
		if rdsize > rdata.len() {
			rdsize = rdata.len();
		}

		if rdsize > EC_ENC_DATA_1_BYTE_MAX {
			sizeb += 1;
			if rdsize < rdata.len() {
				rdsize -= 1;	
			}
			
		}

		if rdsize > EC_ENC_DATA_2_BYTE_MAX {
			sizeb += 1;
			if rdsize < rdata.len() {
				rdsize -= 1;	
			}
		}

		if rdsize > EC_ENC_DATA_3_BYTE_MAX {
			sizeb += 1;
			if rdsize < rdata.len() {
				rdsize -= 1;	
			}
		} 

		if (rdsize + sizeb + 2 ) > maxsize {
			rdsize = maxsize - sizeb - 2;
		}

		/**/
		let mut retv :Vec<u8> = Vec::new();
		/*reserve size*/
		retv.push((EC_ENC_DATA_SIMPLE | ((((sizeb - 1) as u8) & EC_ENC_DATA_SIZE_MASK ) << EC_ENC_DATA_SIZE_SHIFT)) as u8);
		curi = 1;
		if rdsize > EC_ENC_DATA_3_BYTE_MAX {
			retv.push(((rdsize >> 24) & 0xff) as u8);
			curi += 1;
		}

		if rdsize > EC_ENC_DATA_2_BYTE_MAX {
			retv.push(((rdsize >> 16) & 0xff) as u8);
			curi += 1;
		}

		if rdsize > EC_ENC_DATA_1_BYTE_MAX {
			retv.push(((rdsize >> 8) & 0xff) as u8);
			curi += 1;
		}

		retv.push( (rdsize & 0xff) as u8);
		curi += 1;

		idx = 0;
		while idx < rdsize {
			retv.push(rdata[idx]);
			idx += 1;
		}

		curi += rdsize;
		/*now we at last to calculate the crc8*/
		let mut c16 :u16 = 0;

		for k in 0..curi {
			c16 += retv[k] as u16;
			if c16 > 0xff {
				c16 &= 0xff;
			}
		}

		retv.push(c16 as u8);
		let r :BigInt = BigInt::from_bytes_be(Sign::Plus,&retv);

		Ok((r,rdsize))
	}

	pub fn encrypt(&self, data :&[u8]) -> Result<Vec<ECCSignature>, Box<dyn Error>> {
		let bitsize :usize = bit_length(&(self.curve.order));
		let mut bs :usize = bitsize / 8;
		let mut rdsize :usize = 0;
		let mut rdops :RandOps;
		let mut retv :Vec<ECCSignature> = Vec::new();
		let bname :Option<String>;
		if bitsize == (bs * (8 as usize)) {
			/*we should make sure every size is less*/
			bs -= 1;
		}

		if self.randname.is_some() {
			bname = Some(format!("{}",self.randname.as_ref().unwrap()));
		} else {
			bname = None;
		}

		rdops = RandOps::new(bname)?;

		while rdsize < data.len() {
			let mut retok :Result<ECCSignature,Box<dyn Error>> = Ok(ECCSignature::new(&(zero()),&(zero())));
			let (curval,curlen) = self._splice_pack(&data[rdsize..],bs)?;
			let mut trycnt :i32 = 0;
			while trycnt < 3 {
				let vecs = rdops.get_bytes(bs)?;
				let randval :BigInt = BigInt::from_bytes_be(Sign::Plus,&vecs);
				retok = self._ecnrypt_one_pack(&curval,&randval);
				if retok.is_ok() {
					break;
				}
				trycnt += 1;
			}

			if trycnt >= 3 {
				if retok.is_err() {
					return Err(retok.err().unwrap());
				}
			}

			retv.push(retok.unwrap());
			rdsize += curlen;
		}

		Ok(retv)
	}

	pub fn get_ec_pub(&self,types :&str, exps :&str) -> Result<ECPublicKeyChoiceElem,Box<dyn Error>> {
		let typeec :String = format!("{}",self.curve.name);
		let mut curveelem :ECPublicKeyChoiceElem = ECPublicKeyChoiceElem::init_asn1();
		let oid :String;
		if typeec.len() != 0 && exps != EC_PARAMS_EXLICIT {
			oid = get_ecc_oid_by_name(&typeec)?;
			curveelem.typei = 1;
			let mut abbrevelem :ECPublicKeyAbbrevElem = ECPublicKeyAbbrevElem::init_asn1();
			let _ = abbrevelem.types.set_value(EC_PUBLIC_KEY_OID)?;
			let _ = abbrevelem.ectypes.set_value(&oid)?;
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
			totalelem.ecparams.elem.val.push(ecparams);
			curveelem.total.elem.val.push(totalelem);
		}
		Ok(curveelem)
	}

	pub fn extract_from_pub_choice(curveelem :&ECPublicKeyChoiceElem,coordata :&[u8]) -> Result<Self,Box<dyn Error>> {
		let mut pubk :PointJacobi;
		let curve :ECCCurve;
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
			let (x,y) = _from_der_x_y(&(curve.curve),coordata)?;
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
			let (x,y) = _from_der_x_y(&ncurve,coordata)?;
			pubk = curve.generator.clone();
			let _ = pubk.set_x_y(&x,&y)?;
		}

		Ok(PublicKey {
			curve : curve.clone(),
			pubkey : pubk.clone(),
			randname : None,
		})

	}

	pub fn from_der(buf :&[u8]) -> Result<Self,Box<dyn Error>> {
		let mut pubkasn1 :ECPublicKeyAsn1 = ECPublicKeyAsn1::init_asn1();
		let _ = pubkasn1.decode_asn1(buf)?;
		let curveelem :ECPublicKeyChoiceElem;
		if pubkasn1.elem.val.len() != 1 {
			ecsimple_new_error!{EccKeyError,"not pubkasn1 [{}] != 1" , pubkasn1.elem.val.len()}
		}
		let pubkelem = pubkasn1.elem.val[0].clone();

		curveelem = pubkelem.params.clone();
		return Self::extract_from_pub_choice(&curveelem,&pubkelem.coords.data);
	}



	pub fn to_der(&self,types :&str,exps :&str) -> Result<Vec<u8>,Box<dyn Error>> {
		let mut pubkasn1elem :ECPublicKeyAsn1Elem = ECPublicKeyAsn1Elem::init_asn1();
		let mut pubkasn1  :ECPublicKeyAsn1 = ECPublicKeyAsn1::init_asn1();
		let x = self.pubkey.x();
		let y = self.pubkey.y();
		let coordvecs = _to_der_x_y(types,&x,&y)?;
		
		let curveelem = self.get_ec_pub(types,exps)?;
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

impl std::fmt::Debug for PrivateKey {
    fn fmt(&self,f :&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    	let mut s :String = "{".to_string();
    	s.push_str(&(format!("curve : {:?},\n",self.curve)));
    	s.push_str(&(format!("keynum : 0x{:x},\n",self.keynum)));
    	s.push_str(&(format!("pubkey : {:?},\n",self.pubkey)));
    	if self.randname.is_some() {
    		s.push_str(&(format!("randname : {}", self.randname.as_ref().unwrap())));
    	} else {
    		s.push_str("randname : null");
    	}
    	s.push_str("}");
    	write!(f,"{}",s)
    }
}

impl PartialEq  for PrivateKey {
    fn eq(&self,other :&Self) -> bool {
        if self.curve != other.curve  || self.keynum != other.keynum {
            return false;
        }
        return true;
    }

    fn ne(&self, other :&Self) -> bool {
        return  !self.eq(other);
    }
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

	fn _extract_pub_key_simp(simpelem :&ECPublicKeySimpChoiceElem,coordata :&[u8]) -> Result<PublicKey,Box<dyn Error>> {
		let curve :ECCCurve;
		let x :BigInt;
		let y :BigInt;

		if simpelem.typei == 1 {
			let oid = simpelem.simple.get_value();
			let ecname = get_ecc_name_by_oid(&oid)?;
			curve = get_ecc_curve_by_name(&ecname)?;
			(x,y) = _from_der_x_y(&(curve.curve),coordata)?;
		} else if simpelem.typei == 2 {
			if simpelem.total.elem.val.len() != 1 {
				ecsimple_new_error!{EccKeyError,"total [{}] != 1", simpelem.total.elem.val.len()}
			}
			let ecparamselem :ECPublicKeyParamsElem = simpelem.total.elem.val[0].clone();
			if ecparamselem.version.val != 1 {
				ecsimple_new_error!{EccKeyError,"version [{}] != 1", ecparamselem.version.val}
			}
			if ecparamselem.fieldid.elem.val.len() != 1 {
				ecsimple_new_error!{EccKeyError,"fieldid len [{}] != 1", ecparamselem.fieldid.elem.val.len()}
			}
			let fieldidelem = ecparamselem.fieldid.elem.val[0].clone();
			if fieldidelem.types.get_value() != ID_PRIME_FIELD_OID {
				ecsimple_new_error!{EccKeyError,"types [{}] != ID_PRIME_FIELD_OID [{}]", fieldidelem.types.get_value(), ID_PRIME_FIELD_OID}
			}

			let p = BigInt::from_bytes_be(Sign::Plus,&(fieldidelem.primenum.val.to_bytes_be()));
			if ecparamselem.curve.elem.val.len() != 1 {
				ecsimple_new_error!{EccKeyError,"curve elem [{}] != 1",ecparamselem.curve.elem.val.len()}
			}
			let curveelem = ecparamselem.curve.elem.val[0].clone();

			let a = BigInt::from_bytes_be(Sign::Plus,&(curveelem.a.data));
			let b = BigInt::from_bytes_be(Sign::Plus,&(curveelem.b.data));
			let cofactor = BigInt::from_bytes_be(Sign::Plus,&(ecparamselem.cofactor.val.to_bytes_be()));
			let fp :CurveFp = CurveFp::new(&p,&a,&b,&cofactor);
			let (px,py) = _from_der_x_y(&fp,&(ecparamselem.basecoords.data))?;
			let order = BigInt::from_bytes_be(Sign::Plus,&(ecparamselem.order.val.to_bytes_be()));

			let jap :PointJacobi = PointJacobi::new(&fp,&px,&py,&cofactor,Some(order),false);
			curve = ECCCurve::new("",&jap);
			(x,y) = _from_der_x_y(&fp,coordata)?;
		} else {
			ecsimple_new_error!{EccKeyError,"typei [{}] not supported", simpelem.typei}
		}

		let order = curve.order.clone();
		let eccpnt :ECCPoint = ECCPoint::new(Some(curve.curve.clone()),Some(x.clone()),Some(y.clone()),Some(order.clone()));
		PublicKey::new(&curve,&eccpnt)
	}

	pub fn from_der(inv8 :&[u8]) -> Result<Self,Box<dyn Error>> {
		let mut privkey :ECPrivateKeyAsn1 = ECPrivateKeyAsn1::init_asn1();
		let ores = privkey.decode_asn1(&inv8);
		let knum :BigInt;
		let curve :ECCCurve ;
		let pubkey :PublicKey;
		if ores.is_err() {
			let mut pkcs8 :ECPrivateKeyPkcs8 = ECPrivateKeyPkcs8::init_asn1();
			let _ = pkcs8.decode_asn1(&inv8)?;
			/*now to give the pkcs8 values*/
			if pkcs8.elem.val.len() != 1 {
				ecsimple_new_error!{EccKeyError,"no pkcs8 elem [{}] != 1", pkcs8.elem.val.len()}
			}
			let pkcs8elem :ECPrivateKeyPkcs8Elem = pkcs8.elem.val[0].clone();
			if pkcs8elem.version.val != 1 {
				ecsimple_new_error!{EccKeyError,"version pkcs8 [{}] != 1",pkcs8elem.version.val}
			}
			let pubkeyelem :ECPublicKeyChoiceElem = pkcs8elem.pubkey.clone();
			let mut privkeypkcs8 :ECPrivateKeySimp = ECPrivateKeySimp::init_asn1();
			let _ = privkeypkcs8.decode_asn1(&(pkcs8elem.privdata.data))?;
			if privkeypkcs8.elem.val.len() != 1 {
				ecsimple_new_error!{EccKeyError,"privkeypkcs8 [{}] != 1", privkeypkcs8.elem.val.len()}
			}
			let privsimpelem = privkeypkcs8.elem.val[0].clone();
			if privsimpelem.pubcoords.val.len() != 1 {
				ecsimple_new_error!{EccKeyError,"privsimpelem pubcoords [{}] != 1",privsimpelem.pubcoords.val.len()}
			}
			let getpubkey :PublicKey = PublicKey::extract_from_pub_choice(&pubkeyelem,&(privsimpelem.pubcoords.val[0].data))?;
			knum = BigInt::from_bytes_be(Sign::Plus,&(privsimpelem.secnum.data));
			curve = getpubkey.curve.clone();
			/*now to get the calculate values*/
			let testself :PrivateKey = PrivateKey::new(&curve,&knum)?;
			pubkey = testself.get_public_key();
			if pubkey != getpubkey {
				ecsimple_new_error!{EccKeyError,"{:?} != {:?}", pubkey,getpubkey}
			}
		} else {
			if privkey.elem.val.len() != 1 {
				ecsimple_new_error!{EccKeyError,"privkeyelem [{}] != 1" ,privkey.elem.val.len()}
			}
			let privkeyelem = privkey.elem.val[0].clone();
			if privkeyelem.version.val != 1 {
				ecsimple_new_error!{EccKeyError,"version [{}] != 1",privkeyelem.version.val}
			}
			let pubkeyelem :ECPublicKeySimpChoiceElem;
			let pubcoords :Asn1BitData;

			if privkeyelem.pubkey.val.len() != 1 {
				ecsimple_new_error!{EccKeyError,"pubkey [{}] != 1", privkeyelem.pubkey.val.len()}
			}
			pubkeyelem = privkeyelem.pubkey.val[0].clone();

			if privkeyelem.pubdata.val.len() != 1 {
				ecsimple_new_error!{EccKeyError,"pubdata [{}] != 1",privkeyelem.pubdata.val.len()}
			}
			pubcoords = privkeyelem.pubdata.val[0].clone();

			knum = BigInt::from_bytes_be(Sign::Plus,&(privkeyelem.privkey.data));
			pubkey = Self::_extract_pub_key_simp(&pubkeyelem,&(pubcoords.data))?;
			curve = pubkey.curve.clone();

			let testself :PrivateKey = PrivateKey::new(&curve,&knum)?;
			if testself.get_public_key() != pubkey {
				ecsimple_new_error!{EccKeyError,"{:?} != {:?}", testself.get_public_key(),pubkey}
			}
		}
		Ok(PrivateKey {
			curve : curve.clone(),
			keynum : knum.clone(),
			pubkey : pubkey.pubkey.clone(),
			randname : None,
		})
	}


	fn _get_ec_pub_simp(&self,types :&str,exps :&str) -> Result<ECPublicKeySimpChoiceElem,Box<dyn Error>> {
		let mut simpelem :ECPublicKeySimpChoiceElem = ECPublicKeySimpChoiceElem::init_asn1();
		if exps == EC_PARAMS_EXLICIT {
			simpelem.typei = 2;
			let mut pubkey :ECPublicKeyParamsElem = ECPublicKeyParamsElem::init_asn1();
			let mut fieldid :ECPublicKeyFieldIDElem = ECPublicKeyFieldIDElem::init_asn1();
			let mut  curveelem :ECPublicKeyCurveElem = ECPublicKeyCurveElem::init_asn1();
			pubkey.version.val = 1;
			let _ = fieldid.types.set_value(ID_PRIME_FIELD_OID)?;
			let (_ ,vecs) = self.curve.generator.curve().p().to_bytes_be();
			fieldid.primenum.val = BigUint::from_bytes_be(&vecs);
			pubkey.fieldid.elem.val.push(fieldid);
			let (_ ,vecs) = self.curve.generator.curve().a().to_bytes_be();
			curveelem.a.data = vecs.clone();
			let (_ ,vecs) = self.curve.generator.curve().b().to_bytes_be();
			curveelem.b.data = vecs.clone();
			pubkey.curve.elem.val.push(curveelem);
			let x :BigInt = self.curve.generator.x();
			let y :BigInt = self.curve.generator.y();
			let vecs = _to_der_x_y(types,&x,&y)?;
			pubkey.basecoords.data = vecs.clone();
			let (_, vecs) = self.curve.order.to_bytes_be();
			pubkey.order.val = BigUint::from_bytes_be(&vecs);
			let vecs :Vec<u8> = vec![0x1];
			pubkey.cofactor.val = BigUint::from_bytes_be(&vecs);
			simpelem.total.elem.val.push(pubkey);
		} else {
			simpelem.typei = 1;
			let oid = get_ecc_oid_by_name(&self.curve.name)?;
			let _ = simpelem.simple.set_value(&oid)?;
		}
		Ok(simpelem)
	}

	pub fn to_der(&self, types :&str, asn1s :&str , exps :&str) -> Result<Vec<u8>,Box<dyn Error>> {
		if asn1s == EC_SSLEAY_TYPE {
			let mut privkey :ECPrivateKeyAsn1 = ECPrivateKeyAsn1::init_asn1();
			let mut privelem :ECPrivateKeyAsn1Elem = ECPrivateKeyAsn1Elem::init_asn1();
			privelem.version.val = 1;
			let (_, mut vecs) = self.keynum.to_bytes_be();
			let bitsize = bit_length(&self.curve.generator.order());
			let bs = (bitsize + 7 ) / 8;
			let mut pubdata :Asn1BitData = Asn1BitData::init_asn1();
			while vecs.len() < bs {
				vecs.insert(0,0 as u8);
			}

			ecsimple_debug_buffer_trace!(vecs.as_ptr(),vecs.len(),"private key ");

			privelem.privkey.data = vecs.clone();
			let simpelem = self._get_ec_pub_simp(types,exps)?;
			privelem.pubkey.val.push(simpelem);
			let x = self.pubkey.x();
			let y = self.pubkey.y();
			let coordvecs = _to_der_x_y(types,&x,&y)?;
			pubdata.data = coordvecs.clone();
			privelem.pubdata.val.push(pubdata);

			privkey.elem.val.push(privelem);
			return privkey.encode_asn1();
		} else if asn1s == EC_PKCS8_TYPE {
			let mut pkcs8 :ECPrivateKeyPkcs8 = ECPrivateKeyPkcs8::init_asn1();
			let mut pkcs8elem :ECPrivateKeyPkcs8Elem = ECPrivateKeyPkcs8Elem::init_asn1();
			pkcs8elem.version.val = 1;
			pkcs8elem.pubkey = self.get_public_key().get_ec_pub(types,exps)?;
			let x = self.pubkey.x();
			let y = self.pubkey.y();
			let coordvecs = _to_der_x_y(types,&x,&y)?;
			let mut privelem:ECPrivateKeySimpElem = ECPrivateKeySimpElem::init_asn1();
			let mut privkey :ECPrivateKeySimp = ECPrivateKeySimp::init_asn1();
			let mut pubcoords :Asn1BitData = Asn1BitData::init_asn1();
			privelem.version.val = 1;
			let (_,mut vecs) = self.keynum.to_bytes_be();
			let bitsize = bit_length(&self.curve.generator.order());
			let bs = (bitsize + 7) / 8;
			while vecs.len() < bs {
				vecs.insert(0,0 as u8);
			}
			privelem.secnum.data = vecs.clone();
			pubcoords.data = coordvecs.clone();
			privelem.pubcoords.val.push(pubcoords.clone());
			privkey.elem.val.push(privelem);
			let rdata = privkey.encode_asn1()?;
			pkcs8elem.privdata.data = rdata.clone();
			pkcs8.elem.val.push(pkcs8elem);
			return pkcs8.encode_asn1();
		}

		ecsimple_new_error!{EccKeyError,"not support asn1s [{}]", asn1s}
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
			randname : None,
		}
	}

	fn _check_enc_data(&self,data :&[u8]) -> Result<Vec<u8>,Box<dyn Error>> {
		/*now to check for the data*/
		if data.len() < 4 {
			ecsimple_new_error!{EccKeyError,"data.len [{}] < 4", data.len()}
		}
		let sizeb :usize = (((data[0] >> EC_ENC_DATA_SIZE_SHIFT) & EC_ENC_DATA_SIZE_MASK) + 1) as usize;
		if (data[0] & EC_ENC_DATA_MASK) == 0 {
			ecsimple_new_error!{EccKeyError,"[0] & 0x3f == 0"}
		}
		if (data[0] & EC_ENC_DATA_MASK) == EC_ENC_DATA_SIMPLE {
			let mut rdsize :usize = 0;
			let mut curi :usize;
			curi = 1;
			if sizeb == 1 {
				rdsize += data[curi] as usize;
				curi += 1;
			} else if sizeb == 2 {
				rdsize += (data[curi] as usize) << 8;
				curi += 1;
				rdsize += data[curi] as usize;
				curi += 1;
			} else if sizeb == 3 {
				rdsize += (data[curi] as usize) << 16;
				curi += 1;
				rdsize += (data[curi] as usize) << 8;
				curi += 1;
				rdsize += data[curi] as usize;
				curi += 1;
			} else {
				rdsize += (data[curi] as usize) << 24;
				curi += 1;
				rdsize += (data[curi] as usize) << 16;
				curi += 1;
				rdsize += (data[curi] as usize) << 8;
				curi += 1;
				rdsize += data[curi] as usize;
				curi += 1;
			}

			if data.len() != (rdsize + 3) {
				ecsimple_debug_buffer_error!(data.as_ptr(),data.len(),"data extracted sizeb [0x{:x}]",sizeb);
				ecsimple_new_error!{EccKeyError,"rdsize [0x{:x}] + 3 != data.len [0x{:x}]", rdsize, data.len()}
			}
			let mut crcv :u16 = 0;
			for k in 0..(curi + rdsize) {
				crcv += data[k] as u16;
				if crcv > 0xff {
					crcv &= 0xff;
				}
			}

			if (crcv as u8 ) != data[(curi+ rdsize)] {
				ecsimple_new_error!{EccKeyError,"crc [0x{:x}] != get [0x{:x}]", crcv,data[(curi+rdsize)]}
			}
			let retv :Vec<u8> = data[curi..(curi+rdsize)].to_vec();
			return Ok(retv);
		} else {
			ecsimple_new_error!{EccKeyError,"mask [0x{:x}] not supported", data[0] & EC_ENC_DATA_MASK}
		}
	}


	fn _extract_data(&self,nv :&BigInt, p :&BigInt) -> Result<Vec<u8>,Box<dyn Error>> {
		let (_, vecs) = nv.to_bytes_be();
		let ores = self._check_enc_data(&vecs);
		let retv :Vec<u8>;
		if ores.is_err() {
			let c = (nv + p) % p;
			let (_, cvecs) = c.to_bytes_be();
			retv = self._check_enc_data(&cvecs)?;
		} else {
			retv = ores.unwrap();
		}
		Ok(retv)
	}

	fn _decript_one_pack(&self, ecsig :&ECCSignature) -> Result<Vec<u8>,Box<dyn Error>> {
		let a :BigInt = self.curve.curve.a();
		let b :BigInt = self.curve.curve.b();
		let p :BigInt = self.curve.curve.p();
		let r :BigInt = ecsig.r.clone();
		let s :BigInt = ecsig.s.clone();
		let alpha :BigInt = (&r * &r * &r + &a * &r + &b) % &p;
		let y :BigInt = square_root_mod_prime(&alpha,&p)?;
		let nrp :ECCPoint = ECCPoint::new(Some(self.curve.generator.curve()),Some(r.clone()),Some(y.clone()),Some(self.curve.generator.order()));
		let mut nrj :PointJacobi = PointJacobi::from_affine(&nrp,false);
		let mut orr :PointJacobi = nrj.mul_int(&self.keynum);
		let opt :ECCPoint = orr.to_affine();
		let nv :BigInt = s - opt.x();
		let retv = self._extract_data(&nv,&p)?;
		Ok(retv)
	}

	pub fn decrypt(&self,sigs :&[ECCSignature]) -> Result<Vec<u8>,Box<dyn Error>> {
		let mut retv :Vec<u8> = Vec::new();
		for ecsig in sigs.iter() {
			let curv :Vec<u8> = self._decript_one_pack(ecsig)?;
			retv.extend(curv);
		}
		Ok(retv)
	}

	pub fn ecdh_value(&self,pubk :&PublicKey) -> Result<BigInt,Box<dyn Error>> {
		let mut pk :PublicKey = pubk.clone();
		let vpnt :PointJacobi = pk.pubkey.mul_int(&self.keynum);
		if vpnt.isinfinity() {
			ecsimple_new_error!{EccKeyError,"infinity"}
		}
		Ok(vpnt.x())
	}
}