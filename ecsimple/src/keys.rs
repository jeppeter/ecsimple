

use crate::consts::*;
use crate::point::*;
use crate::group::*;
use crate::bngf2m::*;
use crate::signature::*;
use crate::utils::*;
use crate::randop::*;
use crate::logger::*;
use crate::*;
use num_bigint::{BigInt,Sign};
use num_traits::{zero,one};

use std::error::Error;

ecsimple_error_class!{EcKeyError}


#[derive(Clone)]
pub struct ECGf2mPubKey {
	base :ECGf2mPoint,
	pubk :ECGf2mPoint,
}

impl ECGf2mPubKey {
	pub fn new(grp :&ECGroupBnGf2m,x :&BigInt,y :&BigInt) -> ECGf2mPubKey {
		let b = ECGf2mPoint::new(grp);
		let xn :BnGf2m = BnGf2m::new_from_bigint(x);
		let yn :BnGf2m = BnGf2m::new_from_bigint(y);
		let zn :BnGf2m = BnGf2m::one();
		ECGf2mPubKey {
			base : b,
			pubk : ECGf2mPoint::new_point(&xn,&yn,&zn,grp),
		}
	}

	fn uncompress_x_point(grp :&ECGroupBnGf2m, x_ :&BigInt, ybit :u8) -> Result<BigInt,Box<dyn Error>> {
		let b = ECGf2mPoint::new(grp);
		let xb :BnGf2m = BnGf2m::new_from_bigint(&x_);
		let field :BnGf2m = BnGf2m::new_from_bigint(&b.group.p);
		let x :BnGf2m = &xb % &field;
		let y :BigInt;
		let mut yn :BnGf2m;
		let mut tmp :BnGf2m;
		let z :BnGf2m;
		let z0 :u8;
		ecsimple_log_trace!("x 0x{:X} = x_ 0x{:X} % group->field 0x{:X}",x,x_,field);
		if x.is_zero() {
			let yn = &b.group.b.mul_op(&b.group.b).mod_op(&field);
			y = yn.to_bigint();
			ecsimple_log_trace!("y 0x{:X} = group->b 0x{:X} ^ 2 % field 0x{:X}",y,b.group.b,field);
		} else {
			tmp = b.field_sqr(&x);
			tmp = b.field_div(&b.group.b,&tmp)?;
			tmp = tmp.add_op(&b.group.a);
			ecsimple_log_trace!("tmp 0x{:X} group->a 0x{:X}",tmp,b.group.a);
			tmp = tmp.add_op(&x);
			ecsimple_log_trace!("tmp 0x{:X} x 0x{:X}",tmp,x);
			z = tmp.sqrt_quad_op(&field)?;
			ecsimple_log_trace!("z 0x{:X}",z);
			if z.is_odd() {
				z0 = 1;
			} else {
				z0 = 0;
			}
			yn = b.field_mul(&x,&z);
			if z0 != ybit {
				yn = yn.add_op(&x);
				ecsimple_log_trace!("y 0x{:X} x 0x{:X}",yn,x);
			}
			y = yn.to_bigint();
		}
		Ok(y)
	}

	pub fn from_der(grp :&ECGroupBnGf2m, dercode :&[u8]) -> Result<Self,Box<dyn Error>> {
		let b = ECGf2mPoint::new(grp);
		let mut pubk :ECGf2mPoint = b.clone();
		if dercode.len() < 1 {
			ecsimple_new_error!{EcKeyError,"code [{}] < 1", dercode.len()}
		}
		let code :u8 = dercode[0] & EC_CODE_MASK;
		let ybit :u8 = dercode[0] & EC_CODE_YBIT;
		let degr :i64 = grp.degree();
		let fieldsize :usize = ((degr + 7) >> 3) as usize;
		let x :BigInt;
		let y :BigInt;

		if code == EC_CODE_UNCOMPRESSED {
			if dercode.len() < (1 + 2 *fieldsize) {
				ecsimple_new_error!{EcKeyError,"len [{}] < 1 + {} * 2", dercode.len(), fieldsize}
			}
			x = BigInt::from_bytes_be(Sign::Plus,&dercode[1..(fieldsize+1)]);
			ecsimple_log_trace!("x 0x{:X}",x);
			y = BigInt::from_bytes_be(Sign::Plus,&dercode[(fieldsize+1)..(2*fieldsize+1)]);
		} else if code == EC_CODE_COMPRESSED {
			if dercode.len() < (1 + fieldsize) {
				ecsimple_new_error!{EcKeyError,"len [{}] < 1 + {} ", dercode.len(), fieldsize}	
			}
			x = BigInt::from_bytes_be(Sign::Plus,&dercode[1..(fieldsize+1)]);
			ecsimple_log_trace!("x 0x{:X}",x);
			y = ECGf2mPubKey::uncompress_x_point(grp,&x,ybit)?;
		} else if code == EC_CODE_HYBRID {
			if dercode.len() < (1 + 2 * fieldsize) {
				ecsimple_new_error!{EcKeyError,"len [{}] < 1 + {} * 2", dercode.len(), fieldsize}	
			}
			x = BigInt::from_bytes_be(Sign::Plus,&dercode[1..(fieldsize+1)]);
			ecsimple_log_trace!("x 0x{:X}",x);
			y = BigInt::from_bytes_be(Sign::Plus,&dercode[(fieldsize+1)..(2*fieldsize+1)]);
			ecsimple_log_trace!("y 0x{:X}",y);
			if x == zero() && ybit != 0{
				ecsimple_new_error!{EcKeyError,"x == 0 and ybit set"}
			} else {
				let xb :BnGf2m = BnGf2m::new_from_bigint(&x);
				let yb :BnGf2m = BnGf2m::new_from_bigint(&y);
				let ybi :BnGf2m = b.field_div(&yb,&xb)?;
				ecsimple_log_trace!("yxi 0x{:X} y 0x{:X} x 0x{:X}",ybi,yb,xb);
				if (ybit != 0 && !ybi.is_odd()) || (ybit == 0 && ybi.is_odd()) {
					ecsimple_new_error!{EcKeyError,"ybi 0x{:X} not match ybit 0x{:X}", ybi,ybit}
				}
			}
		} else {
			ecsimple_new_error!{EcKeyError,"unsupport code [0x{:X}] for public point", dercode[0]}
		}
		let mut bval :BnGf2m;
		bval = BnGf2m::new_from_bigint(&x);
		pubk.set_x(&bval);
		bval = BnGf2m::new_from_bigint(&y);
		pubk.set_y(&bval);
		bval = BnGf2m::one();
		pubk.set_z(&bval);
		let _ = pubk.check_on_curve()?;
		ecsimple_log_trace!("x 0x{:X} y 0x{:X}", x,y);

		Ok(Self {
			base : b.clone(),
			pubk : pubk.clone(),
		})
	}

	pub fn verify_base(&self,sig :&ECSignature, hashnum :&BigInt) -> Result<bool,Box<dyn Error>> {
		let mut u2 :BigInt;
		let order :BigInt = self.base.group.order.clone();
		let vfypnt :ECGf2mPoint;
		ecsimple_log_trace!("sig.r 0x{:X} sig.s 0x{:X}", sig.r,sig.s);
		if sig.r == zero() || sig.s == zero() {
			ecsimple_new_error!{EcKeyError,"sig.r 0x{:X} or sig.s 0x{:X} zero",sig.r,sig.s}
		}
	
		let e :BigInt = &order - 2;
		u2 = sig.s.modpow(&e,&order);
		ecsimple_log_trace!("s 0x{:X} u2 0x{:X}",sig.s,u2);
		let m :BigInt = format_bigint_as_order(hashnum,&order);
		ecsimple_log_trace!("dgst 0x{:X}",m);

		let mut u1 :BigInt = (&u2 * &m) % &order;
		ecsimple_log_trace!("u1 0x{:X} = m 0x{:X} * tmp 0x{:X} % order 0x{:X}", u1,m,u2,order);

		u2 = &(&u2 * &sig.r) % &order;
		ecsimple_log_trace!("u2 0x{:X} sig->r 0x{:X} order 0x{:X}", u2,sig.r,order);

		vfypnt = self.pubk.mulex_op(&u1,&u2)?;
		let xn :BigInt = vfypnt.x().to_bigint();
		u1 = &xn % &order;
		ecsimple_log_trace!("u1 0x{:X} = X 0x{:X} % order 0x{:X} sig->r 0x{:X}",u1,xn,order,sig.r);
		if u1 != sig.r {
			return Ok(false);
		}
		Ok(true)
	}

}

impl std::fmt::Display for ECGf2mPubKey {
	fn fmt(&self, f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f,"base {};\npoint {};\n",self.base,self.pubk)
	}
}



#[derive(Clone)]
pub struct ECGf2mPrivateKey {
	base : ECGf2mPoint,
	privnum :BigInt,
}

impl ECGf2mPrivateKey {
	pub fn new(grp :&ECGroupBnGf2m , privnum :&BigInt) -> ECGf2mPrivateKey {
		let b :ECGf2mPoint = ECGf2mPoint::new(grp);
		ECGf2mPrivateKey {
			base : b,
			privnum : privnum.clone(),
		}
	}

	pub fn export_pubkey(&self) -> ECGf2mPubKey {
		let ck : ECGf2mPoint;
		ck = self.base.mul_op(&self.privnum,false);
		let retv :ECGf2mPubKey = ECGf2mPubKey {
			base : self.base.clone(),
			pubk : ck.clone(),
		};
		retv
	}

	#[allow(non_snake_case)]
	fn setup_sign(&self) -> Result<(BigInt,BigInt),Box<dyn Error>> {
		let mut r :BigInt;
		let mut tmppnt :ECGf2mPoint = self.base.clone();
		let zv :BnGf2m = BnGf2m::zero();
		let ov :BigInt = one();
		let order :BnGf2m;
		tmppnt.set_x(&zv);
		tmppnt.set_y(&zv);
		tmppnt.set_z(&zv);
		let mut k  :BigInt ;
		let mut X :BnGf2m;
		let blen = get_max_bits(&self.base.group.order);
		ecsimple_log_trace!("tmp.x 0x{:X} tmp.y 0x{:X}, tmp.z 0x{:X}", tmppnt.x(),tmppnt.y(),tmppnt.z());
		ecsimple_log_trace!("order 0x{:X}",self.base.group.order);
		k = ov << blen;
		order = BnGf2m::new_from_bigint(&self.base.group.order);
		loop {
			ecsimple_log_trace!("k 0x{:X}",k);
			k = ecsimple_rand_range(&self.base.group.order);
			ecsimple_log_trace!("k 0x{:X} order 0x{:X} dlen 0x{:x}", k, self.base.group.order,((blen + 7 ) >> 3) as i64);

			ecsimple_log_trace!("group.x 0x{:X} group.y 0x{:X} group.z 0x{:X}", self.base.group.generator.x,self.base.group.generator.y,self.base.group.generator.z);
			tmppnt = self.base.mul_op(&k,false);
			ecsimple_log_trace!("tmp.x 0x{:X} tmp.y 0x{:X} tmp.z 0x{:X}", tmppnt.x(),tmppnt.y(),tmppnt.z());

			(X,_) = tmppnt.get_affine_points()?;

			ecsimple_log_trace!("tmp.x 0x{:X} tmp.y 0x{:X} tmp.z 0x{:X}", tmppnt.x(),tmppnt.y(),tmppnt.z());
			ecsimple_log_trace!("X 0x{:X} order 0x{:X}",X,order);


			let xb :BigInt = X.to_bigint();

			r = xb % &self.base.group.order;


			if r != zero() {
				break;
			}
		}

		ecsimple_log_trace!("X 0x{:X} r 0x{:X}", X,r);

		let e :BigInt = &self.base.group.order - 2;
		let kn = k.clone();

		k = k.modpow(&e,&self.base.group.order);
		ecsimple_log_trace!("(x 0x{:X} ^ e 0x{:X}) = (r 0x{:X}) = 1 % order 0x{:X}",kn,e,k,self.base.group.order);
		ecsimple_log_trace!("k 0x{:X} r 0x{:X}",k,r);
		Ok((k,r))
	}

	pub fn sign_base(&self,hashnum :&[u8]) -> Result<ECSignature,Box<dyn Error>> {
		let bn :BigInt = BigInt::from_bytes_be(Sign::Plus,hashnum);
		ecsimple_log_trace!("begin sign");
		let mut s :BigInt = zero();
		let mut r :BigInt = zero();
		ecsimple_log_trace!("r 0x{:X} s 0x{:X}",r,s);
		ecsimple_log_trace!("order 0x{:X}", self.base.group.order);

		let realhash = format_bigint_as_order(&bn,&self.base.group.order);
		ecsimple_log_trace!("dgst 0x{:X}", realhash);

		assert!(realhash <= self.base.group.order);
		let kinv :BigInt;
		(kinv,r) = self.setup_sign()?;
		ecsimple_log_trace!("ckinv 0x{:X} r 0x{:X}",kinv,r);
		s = (&realhash + &self.privnum * &r) % &self.base.group.order;
		ecsimple_log_trace!("s 0x{:X}",s);
		s = (&s * &kinv) % &self.base.group.order;
		ecsimple_log_trace!("s 0x{:X}",s);


		let retv :ECSignature = ECSignature::new(&r,&s);
		Ok(retv)
	}
}


#[derive(Clone)]
pub struct ECPrimePubKey {
	base :ECPrimePoint,
	pubk :ECPrimePoint,
}

impl ECPrimePubKey {
	pub fn new(grp :&ECGroupPrime,x :&BigInt,y :&BigInt) -> ECPrimePubKey {
		let b = ECPrimePoint::new(grp);
		let zn :BigInt = one();
		ECPrimePubKey {
			base : b,
			pubk : ECPrimePoint::new_point(&x,&y,&zn,grp),
		}
	}


	fn uncompress_x_point(grp :&ECGroupPrime, x_ :&BigInt, ybit :u8) -> Result<BigInt,Box<dyn Error>> {
		let b = ECPrimePoint::new(grp);
		let field :BigInt = b.group.p.clone();
		let x :BigInt = nmod(&x_,&grp.p);
		ecsimple_log_trace!("nnmod(x 0x{:X},x_ 0x{:X},group.field 0x{:X})", x,x_,grp.p);
		let mut y :BigInt;
		let mut tmp2 :BigInt;
		let mut tmp1 :BigInt;
		let mut kbit :i32;
		let zv :BigInt = zero();
		tmp2 = (x_.clone() * x_.clone()) % &field;
		ecsimple_log_trace!("mod_sqr(tmp2 0x{:X},x_ 0x{:X},group.field 0x{:X})",tmp2,x_,grp.p);
		tmp1 = (tmp2.clone() * x_.clone()) % &field;
		ecsimple_log_trace!("mod_mul(tmp1 0x{:X},tmp2 0x{:X},x_ 0x{:X},group.field 0x{:X})",tmp1,tmp2,x_,field);
		if grp.is_minus3 {
			tmp2 = b.lshift1_mod_quick(&x,&field);
			ecsimple_log_trace!("lshift1_mod_quick(tmp2 0x{:X},x 0x{:X},group.field 0x{:X})",tmp2,x,field);
			tmp2 = b.add_mod_quick(&tmp2,&x,&field);
			ecsimple_log_trace!("add_mod_quick(tmp2 0x{:X},tmp2,x 0x{:X},group.field 0x{:X})",tmp2,x,field);
			tmp1 = b.sub_mod_quick(&tmp1,&tmp2,&field);
			ecsimple_log_trace!("sub_mod_quick(tmp1 0x{:X},tmp1,tmp2 0x{:X},group.field 0x{:X})",tmp1,tmp2,field);
		} else {
			tmp2 = b.field_decode(&grp.a);
			tmp2 = (tmp2.clone() * x.clone()) % &field;
			ecsimple_log_trace!("mod_mul(tmp2 0x{:X},tmp2,x 0x{:X},group.field 0x{:X})",tmp2,x,field);

			tmp1 = b.add_mod_quick(&tmp1,&tmp2,&field);
			ecsimple_log_trace!("add_mod_quick(tmp1 0x{:X},tmp1,tmp2 0x{:X},group.field 0x{:X})",tmp1,tmp2,field);
		}

		tmp2 = b.field_decode(&grp.b);
		tmp1 = b.add_mod_quick(&tmp1,&tmp2,&field);
		ecsimple_log_trace!("add_mod_quick(tmp1 0x{:X},tmp1,tmp2 0x{:X},group.field 0x{:X})",tmp1,tmp2,field);

		y = mod_sqrt(&tmp1,&field)?;
		ecsimple_log_trace!("mod_sqr(y 0x{:X},tmp1 0x{:X},group.field 0x{:X})",y,tmp1,field);
		kbit = get_bit_set(&y,0);
		if kbit != ybit as i32 {
			if y == zv {
				ecsimple_new_error!{EcKeyError,"not valid y 0x{:X}",y}
			}
			y = &field - &y;
			ecsimple_log_trace!("usub(y 0x{:X},group.field 0x{:X},y)",y,field);
		}
		kbit = get_bit_set(&y,0);
		if kbit != ybit as i32 {
			ecsimple_new_error!{EcKeyError,"y 0x{:X} not valid for bit",y}
		}

		Ok(y)
	}

	pub fn from_der(grp :&ECGroupPrime, dercode :&[u8]) -> Result<Self,Box<dyn Error>> {
		let b = ECPrimePoint::new(grp);
		let pubk :ECPrimePoint = b.clone();
		if dercode.len() < 1 {
			ecsimple_new_error!{EcKeyError,"code [{}] < 1", dercode.len()}
		}
		let code :u8 = dercode[0] & EC_CODE_MASK;
		let ybit :u8 = dercode[0] & EC_CODE_YBIT;
		let degr :i64 = grp.degree();
		let fieldsize :usize = ((degr + 7) >> 3) as usize;
		let x :BigInt;
		let y :BigInt;
		ecsimple_log_trace!("grp degree [0x{:x}] fieldsize 0x{:x}", degr,fieldsize);

		if code == EC_CODE_UNCOMPRESSED {
			if dercode.len() < (1 + 2 *fieldsize) {
				ecsimple_new_error!{EcKeyError,"len [{}] < 1 + {} * 2", dercode.len(), fieldsize}
			}
			x = BigInt::from_bytes_be(Sign::Plus,&dercode[1..(fieldsize+1)]);
			ecsimple_log_trace!("x 0x{:X}",x);
			y = BigInt::from_bytes_be(Sign::Plus,&dercode[(fieldsize+1)..(2*fieldsize+1)]);
		} else if code == EC_CODE_COMPRESSED {
			if dercode.len() < (1 + fieldsize) {
				ecsimple_new_error!{EcKeyError,"len [{}] < 1 + {} ", dercode.len(), fieldsize}	
			}
			x = BigInt::from_bytes_be(Sign::Plus,&dercode[1..(fieldsize+1)]);
			ecsimple_log_trace!("x 0x{:X}",x);
			y = ECPrimePubKey::uncompress_x_point(grp,&x,ybit)?;
		} else if code == EC_CODE_HYBRID {
			if dercode.len() < (1 + 2 * fieldsize) {
				ecsimple_new_error!{EcKeyError,"len [{}] < 1 + {} * 2", dercode.len(), fieldsize}	
			}
			x = BigInt::from_bytes_be(Sign::Plus,&dercode[1..(fieldsize+1)]);
			ecsimple_log_trace!("x 0x{:X}",x);
			y = BigInt::from_bytes_be(Sign::Plus,&dercode[(fieldsize+1)..(2*fieldsize+1)]);
			ecsimple_log_trace!("y 0x{:X}",y);
			if x == zero() && ybit != 0{
				ecsimple_new_error!{EcKeyError,"x == 0 and ybit set"}
			} else {
			}
		} else {
			ecsimple_new_error!{EcKeyError,"unsupport code [0x{:X}] for public point", dercode[0]}
		}
		let z :BigInt = one();
		let pubk = pubk.set_affine_coordinates(&x,&y,&z)?;
		let _ = pubk.check_on_curve()?;
		ecsimple_log_trace!("pubkey.x 0x{:X} pubkey.y 0x{:X} pubkey.z 0x{:X}",pubk.x(),pubk.y(),pubk.z());

		Ok(Self {
			base : b.clone(),
			pubk : pubk.clone(),
		})
	}


	#[allow(unused_variables)]
	pub fn verify_base(&self,sig :&ECSignature, hashnum :&BigInt) -> Result<bool,Box<dyn Error>> {
		let mut u2 :BigInt;
		let order :BigInt = self.base.group.order.clone();
		let x :BigInt;
		ecsimple_log_trace!("sig.r 0x{:X} sig.s 0x{:X}", sig.r,sig.s);
		if sig.r == zero() || sig.s == zero() {
			ecsimple_new_error!{EcKeyError,"sig.r 0x{:X} or sig.s 0x{:X} zero",sig.r,sig.s}
		}
	
		let e :BigInt = &order - 2;
		u2 = sig.s.modpow(&e,&order);
		ecsimple_log_trace!("s 0x{:X} u2 0x{:X}",sig.s,u2);
		let m :BigInt = format_bigint_as_order(hashnum,&order);
		ecsimple_log_trace!("dgst 0x{:X}",m);

		let mut u1 :BigInt = (&u2 * &m) % &order;
		ecsimple_log_trace!("u1 0x{:X} = m 0x{:X} * tmp 0x{:X} % order 0x{:X}", u1,m,u2,order);

		u2 = &(&u2 * &sig.r) % &order;
		ecsimple_log_trace!("u2 0x{:X} sig->r 0x{:X} order 0x{:X}", u2,sig.r,order);
		let vfypnt :ECPrimePoint;

		vfypnt = self.pubk.mulex_op(&u1,&u2)?;
		(x,_) = vfypnt.get_affine_points()?;
		if x != sig.r {
			u1 = x.clone() % self.base.group.order.clone();
			ecsimple_log_trace!("u1 0x{:X} = X 0x{:X} % order 0x{:X} sig->r 0x{:X}",u1,x,self.base.group.order,sig.r);
			if u1 != sig.r {
				ecsimple_log_error!("x 0x{:X} != sig.r 0x{:X}", u1,sig.r);
				return Ok(false);				
			}
		}
		Ok(true)
	}

}

impl std::fmt::Display for ECPrimePubKey {
	fn fmt(&self, f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f,"base {};\npoint {};\n",self.base,self.pubk)
	}
}



#[allow(dead_code)]
#[derive(Clone)]
pub struct ECPrimePrivateKey {
	base : ECPrimePoint,
	privnum :BigInt,
}

impl ECPrimePrivateKey {
	pub fn new(grp :&ECGroupPrime , privnum :&BigInt) -> ECPrimePrivateKey {
		let b :ECPrimePoint = ECPrimePoint::new(grp);
		ECPrimePrivateKey {
			base : b,
			privnum : privnum.clone(),
		}
	}


	#[allow(non_snake_case)]
	fn setup_sign(&self) -> Result<(BigInt,BigInt),Box<dyn Error>> {
		let mut r :BigInt;
		let mut tmppnt :ECPrimePoint = self.base.clone();
		let zv :BigInt = zero();
		let ov :BigInt = one();
		let mut X :BigInt;
		let e :BigInt;
		tmppnt.set_x(&zv);
		tmppnt.set_y(&zv);
		tmppnt.set_z(&zv);
		let mut k  :BigInt ;
		let blen = get_max_bits(&self.base.group.order);
		ecsimple_log_trace!("tmp.x 0x{:X} tmp.y 0x{:X}, tmp.z 0x{:X}", tmppnt.x(),tmppnt.y(),tmppnt.z());
		ecsimple_log_trace!("order 0x{:X}",self.base.group.order);
		k = ov.clone() << blen;
		loop {
			ecsimple_log_trace!("k 0x{:X}",k);
			k = ecsimple_rand_range(&self.base.group.order);
			ecsimple_log_trace!("k 0x{:X} order 0x{:X} dlen 0x{:x}", k, self.base.group.order,((blen + 7 ) >> 3) as i64);

			ecsimple_log_trace!("group.x 0x{:X} group.y 0x{:X} group.z 0x{:X}", self.base.group.generator.x,self.base.group.generator.y,self.base.group.generator.z);
			tmppnt = self.base.mul_op(&k,false);
			ecsimple_log_trace!("tmp.x 0x{:X} tmp.y 0x{:X} tmp.z 0x{:X}", tmppnt.x(),tmppnt.y(),tmppnt.z());
			X = tmppnt.x();
			r = nmod(&X,&self.base.group.order);
			ecsimple_log_trace!("X 0x{:X} r 0x{:X}", X,r);
			if r != zv {
				break;
			}
		}

		e = self.base.group.order.clone() - ov.clone() - ov.clone();
		ecsimple_log_trace!("k 0x{:X}",k);
		k = k.modpow(&e,&self.base.group.order);
		ecsimple_log_trace!("k 0x{:X} r 0x{:X}",k,r);


		Ok((k,r))
	}


	#[allow(unused_variables)]
	pub fn sign_base(&self,hashnum :&[u8]) -> Result<ECSignature,Box<dyn Error>> {
		let bn :BigInt = BigInt::from_bytes_be(Sign::Plus,hashnum);
		ecsimple_log_trace!("begin sign");
		let mut s :BigInt = zero();
		let mut r :BigInt = zero();
		let kinv :BigInt;
		ecsimple_log_trace!("r 0x{:X} s 0x{:X}",r,s);
		ecsimple_log_trace!("order 0x{:X}", self.base.group.order);

		let realhash = format_bigint_as_order(&bn,&self.base.group.order);
		ecsimple_log_trace!("dgst 0x{:X}", realhash);
		(kinv,r) = self.setup_sign()?;
		ecsimple_log_trace!("ckinv 0x{:X} r 0x{:X}",kinv,r);
		s = (&realhash + &self.privnum * &r) % &self.base.group.order;
		ecsimple_log_trace!("s 0x{:X}",s);
		s = (&s * &kinv) % &self.base.group.order;
		ecsimple_log_trace!("s 0x{:X}",s);
		ecsimple_log_trace!("r 0x{:X} s 0x{:X}",r,s);
		let retv :ECSignature = ECSignature::new(&r,&s);
		Ok(retv)
	}
}


