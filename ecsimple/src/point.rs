
use crate::bngf2m::*;
use crate::group::*;
use crate::utils::*;
use num_bigint::{BigInt};
use num_traits::{zero};

use crate::logger::*;
use crate::randop::{ecsimple_rand_bits};


#[derive(Clone)]
pub struct ECGf2mPoint {
	x :BnGf2m,
	y :BnGf2m,
	z :BnGf2m,
	pub group :ECGroupBnGf2m,
	infinity : bool,
}

impl std::fmt::Display for ECGf2mPoint {
	fn fmt(&self, f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f,"curve[{}] isinfinity {} x 0x{:x} y 0x{:x} z 0x{:x}", self.group,self.infinity,self.x,self.y,self.z)
	}
}

impl std::default::Default for ECGf2mPoint {
	fn default() -> Self {
		ECGf2mPoint {
			x : BnGf2m::default(),
			y :BnGf2m::default(),
			z :BnGf2m::default(),
			group : ECGroupBnGf2m::default(),
			infinity : true,
		}
	}
}


impl ECGf2mPoint {
	pub fn is_infinity(&self) -> bool {
		return self.infinity;
	}

	pub fn new(grp :&ECGroupBnGf2m) -> ECGf2mPoint {
		ECGf2mPoint {
			x : grp.generator.x.clone(),
			y : grp.generator.y.clone(),
			z : grp.generator.z.clone(),
			group : grp.clone(),
			infinity : false,
		}
	}


	pub fn new_point(x :&BnGf2m, y :&BnGf2m,z :&BnGf2m, grp :&ECGroupBnGf2m) -> Self {
		Self {
			x :x.clone(),
			y :y.clone(),
			z :z.clone(),
			group :grp.clone(),
			infinity : false,
		}
	}


	pub fn set_group(&mut self, grp :&ECGroupBnGf2m) {
		self.group = grp.clone();
	}

	pub fn set_x(&mut self, x :&BnGf2m) {
		self.x = x.clone();
	}

	pub fn set_y(&mut self, y :&BnGf2m) {
		self.y = y.clone();
	}

	pub fn set_z(&mut self, z :&BnGf2m) {
		self.z = z.clone();
	}

	pub fn x(&self) -> BnGf2m {
		return self.x.clone();
	}

	pub fn y(&self) -> BnGf2m {
		return self.y.clone();
	}

	pub fn z(&self) -> BnGf2m {
		return self.z.clone();
	}

	fn field_mul(&self,a :&BnGf2m, b :&BnGf2m) -> BnGf2m {
		let retv :BnGf2m ;
		retv = a * b;
		let ord :BnGf2m = BnGf2m::new_from_bigint(&self.group.p);
		//ecsimple_log_trace!("a 0x{:X} * b 0x{:X} % ord 0x{:X} = 0x{:X}",a,b,ord, retv.clone() % ord.clone());
		return retv % ord;
	}

	fn field_sqr(&self,a :&BnGf2m) -> BnGf2m {
		let retv :BnGf2m;
		retv = a * a;
		let ord :BnGf2m = BnGf2m::new_from_bigint(&self.group.p);
		//ecsimple_log_trace!("a 0x{:X} * a 0x{:X} % ord 0x{:X} = 0x{:X}",a,a, ord,retv.clone() % ord.clone());
		return retv % ord;		
	}

	fn ladder_pre(&self, r :&mut ECGf2mPoint, s :&mut ECGf2mPoint, p :&ECGf2mPoint, bits :u64) {
		let mut bs :Vec<u8>;
		bs = ecsimple_rand_bits(bits);
		s.z = BnGf2m::new_from_be(&bs);
		ecsimple_log_trace!("random s->Z 0x{:X}", s.z);

		s.x = self.field_mul(&(p.x),&(s.z));
		ecsimple_log_trace!("s->X 0x{:X}", s.x);


		bs = ecsimple_rand_bits(bits);
		r.y = BnGf2m::new_from_be(&bs);
		ecsimple_log_trace!("random r->Y 0x{:X}",r.y);
		r.z = self.field_sqr(&(p.x));
		r.x = self.field_sqr(&(r.z));
		r.x = &r.x + &self.group.b;
		r.z = self.field_mul(&(r.z),&(r.y));
		r.x = self.field_mul(&(r.x),&(r.y));

		ecsimple_log_trace!("r->X 0x{:X} r->Y 0x{:X} r->Z 0x{:X}", r.x,r.y,r.z);

		return;
	}

	fn ladder_step(&self, r :&mut ECGf2mPoint, s :&mut ECGf2mPoint, p :&ECGf2mPoint) {
		r.y = self.field_mul(&(r.z),&(s.x));
		s.x = self.field_mul(&(r.x),&(s.z));
		s.y = self.field_sqr(&(r.z));

		r.z = self.field_sqr(&(r.x));
		s.z = &r.y + &s.x;
		s.z = self.field_sqr(&(s.z));
		s.x = self.field_mul(&(r.y),&(s.x));
		r.y = self.field_mul(&(s.z),&(p.x));
		s.x = &s.x + &r.y;

		r.y = self.field_sqr(&(r.z));
		r.z = self.field_mul(&(r.z),&(s.y));
		s.y = self.field_sqr(&(s.y));
		s.y = self.field_mul(&(s.y),&(self.group.b));
		r.x = &r.y + &s.y;

		return;
	}

	fn make_affine(&self, _r :&mut ECGf2mPoint) {
		return;
	}

	fn point_invert(&self, r :&mut ECGf2mPoint) {
		if r.is_infinity() || r.y.is_zero() {
			return;
		}

		self.make_affine(r);
		r.y = &r.x + &r.y;
		return;
	}

	fn field_inv(&self,a :&BnGf2m) -> BnGf2m {
		let pbn :BnGf2m = BnGf2m::new_from_bigint(&self.group.p);
		return a.inv_op(&pbn).unwrap();
	}

	fn ladder_post(&self,r :&mut ECGf2mPoint,s :&ECGf2mPoint,p :&ECGf2mPoint) {
		if r.z.is_zero() {
			r.infinity = true;
			return;
		}
		if s.z.is_zero() {
			*r = p.clone();
			self.point_invert(r);
			return;
		}
		let t0 :BnGf2m;
		let mut t1 :BnGf2m;
		let mut t2 :BnGf2m;

		t0 = self.field_mul(&(r.z),&(s.z));
		t1 = self.field_mul(&(p.x),&(r.z));
		t1 = &r.x + &t1;
		//ecsimple_log_trace!("t1 0x{:X}",t1);
		t2 = self.field_mul(&(p.x),&(s.z));
		r.z = self.field_mul(&(r.x),&t2);
		t2 = &t2 + &s.x;
		//ecsimple_log_trace!("t2 0x{:X}",t2);
		t1 = self.field_mul(&t1,&t2);
		t2 = self.field_sqr(&p.x);
		t2 = &p.y + &t2;
		//ecsimple_log_trace!("t2 0x{:X}",t2);
		t2 = self.field_mul(&t2,&t0);
		t1 = &t2 + &t1;
		//ecsimple_log_trace!("t1 0x{:X}",t1);
		t2 = self.field_mul(&p.x,&t0);
		t2 = self.field_inv(&t2);
		t1 = self.field_mul(&t1,&t2);
		r.x = self.field_mul(&r.z,&t2);
		t2 = &p.x + &r.x;
		//ecsimple_log_trace!("t2 0x{:X}",t2);
		t2 = self.field_mul(&t2,&t1);
		r.y = &p.y + &t2;
		//ecsimple_log_trace!("r->Y 0x{:X}",r.y);
		r.z = BnGf2m::one();
		//ecsimple_log_trace!("r->Z 0x{:X}",r.z);

		return;
	}


	pub fn mul_op(&self, bn :&BigInt) -> ECGf2mPoint {
		let zv :BigInt = zero();
		let p :ECGf2mPoint = ECGf2mPoint::new(&self.group);
		let mut s :ECGf2mPoint = ECGf2mPoint::new(&self.group);
		let mut r :ECGf2mPoint = ECGf2mPoint::new(&self.group);
		let cardinal :BigInt;
		let mut lamda :BigInt = zero();
		let mut k :BigInt = zero();
		if bn <= &zv {
			r = self.clone();
			r.infinity = true;
			return r;
		}

		if self.infinity {
			return self.clone();
		}

		if self.group.order == zv || self.group.cofactor == zv {
			panic!("group order 0x{:x} or group cofactor 0x{:x}", self.group.order, self.group.cofactor);
		}

		cardinal = &self.group.order * &self.group.cofactor;

		//ecsimple_log_trace!("field 0x{:X} p 0x{:X}", self.group.p,self.group.p);
		//ecsimple_log_trace!("group->a 0x{:X} a 0x{:X}", self.group.a,self.group.a);
		//ecsimple_log_trace!("group->b 0x{:X} b 0x{:X}", self.group.b,self.group.b);
		//ecsimple_log_trace!("cardinality 0x{:X} order 0x{:X} cofactor 0x{:X}",cardinal,self.group.order,self.group.cofactor);
		ecsimple_log_trace!("k 0x{:X} lambda 0x{:X}", k, lamda);
		//ecsimple_log_trace!("k 0x{:X} lambda 0x{:X}", k, lamda);

		k = bn.clone();
		lamda = &k + &cardinal;
		ecsimple_log_trace!("scalar 0x{:X} k 0x{:X}",k,k);
		ecsimple_log_trace!("lambda 0x{:X}",lamda);

		k = &lamda + &cardinal;
		//ecsimple_log_trace!("k 0x{:X} cardinality 0x{:X}",k,cardinal);

		let cardbits = get_max_bits(&cardinal);
		let mut i :i32;
		let mut pbit :i32 = 1;
		let mut kbit :i32;
		//ecsimple_log_trace!("k 0x{:X} lambda 0x{:X} cardinality_bits 0x{:x}",k,lamda,cardbits);

		s.x = BnGf2m::zero();
		s.y = BnGf2m::zero();
		s.z = BnGf2m::zero();

		r.x = BnGf2m::zero();
		r.y = BnGf2m::zero();
		r.z = BnGf2m::zero();

		//ecsimple_log_trace!("s.X 0x{:X} s.Y 0x{:X} s.Z 0x{:X}",s.x,s.y,s.z);
		//ecsimple_log_trace!("r.X 0x{:X} r.Y 0x{:X} r.Z 0x{:X}",r.x,r.y,r.z);
		//ecsimple_log_trace!("p.X 0x{:X} p.Y 0x{:X} p.Z 0x{:X}",p.x,p.y,p.z);



		//ecsimple_log_trace!("p.X 0x{:X} p.Y 0x{:X} p.Z 0x{:X}",p.x,p.y,p.z);

		self.ladder_pre(&mut r,&mut s, &p, (cardbits - 1) as u64);

		i = (cardbits - 1) as i32;
		while i >= 0 {
			kbit = get_bit_set(&k,i) ^ pbit;
			//ecsimple_log_trace!("s.X 0x{:X} s.Y 0x{:X} s.Z 0x{:X}",s.x,s.y,s.z);
			//ecsimple_log_trace!("r.X 0x{:X} r.Y 0x{:X} r.Z 0x{:X}",r.x,r.y,r.z);
			//ecsimple_log_trace!("[{}]kbit 0x{:x} pbit 0x{:x} [0x{:x}] bitset [0x{:x}]", i,kbit,pbit,i, get_bit_set(&k,i));

			if kbit != 0 {
				(r,s) = (s,r);
			}

			//ecsimple_log_trace!("s.X 0x{:X} s.Y 0x{:X} s.Z 0x{:X}",s.x,s.y,s.z);
			//ecsimple_log_trace!("r.X 0x{:X} r.Y 0x{:X} r.Z 0x{:X}",r.x,r.y,r.z);

			self.ladder_step(&mut r,&mut s,&p);

			//ecsimple_log_trace!("s.X 0x{:X} s.Y 0x{:X} s.Z 0x{:X}",s.x,s.y,s.z);
			//ecsimple_log_trace!("r.X 0x{:X} r.Y 0x{:X} r.Z 0x{:X}",r.x,r.y,r.z);
			//ecsimple_log_trace!("p.X 0x{:X} p.Y 0x{:X} p.Z 0x{:X}",p.x,p.y,p.z);

			pbit ^= kbit;
			i -= 1;
		}
		
		//ecsimple_log_trace!("s.X 0x{:X} s.Y 0x{:X} s.Z 0x{:X}",s.x,s.y,s.z);
		//ecsimple_log_trace!("r.X 0x{:X} r.Y 0x{:X} r.Z 0x{:X}",r.x,r.y,r.z);

		self.ladder_post(&mut r,&mut s,&p);
		//ecsimple_log_trace!("s.X 0x{:X} s.Y 0x{:X} s.Z 0x{:X}",s.x,s.y,s.z);
		//ecsimple_log_trace!("r.X 0x{:X} r.Y 0x{:X} r.Z 0x{:X}",r.x,r.y,r.z);
		//ecsimple_log_trace!("p.X 0x{:X} p.Y 0x{:X} p.Z 0x{:X}",p.x,p.y,p.z);

		return r;
	}

}