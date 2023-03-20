extern crate num_bigint_dig as num_bigint2;

use num_bigint::{BigInt};
use num_traits::{zero,one};
use crate::arithmetics::*;
use crate::logger::*;

#[derive(Clone)]
pub struct CurveFp {
    p :BigInt,
    a :BigInt,
    b :BigInt,
    h :BigInt,
}

impl std::fmt::Debug for CurveFp {
    fn fmt(&self,f :&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s :String = "".to_string();
        s.push_str("CurveFp(");
        s.push_str(&format!("p=0x{:x};",self.p));
        s.push_str(&format!("a=0x{:x};",self.a));
        s.push_str(&format!("b=0x{:x};",self.b));
        s.push_str(&format!("h=0x{:x};",self.h));
        s.push_str(")");
        write!(f,"{}",&s)
    }
}

impl PartialEq  for CurveFp {
    fn eq(&self,other :&Self) -> bool {
        if self.p != other.p  || self.a != other.a || self.b != other.b || self.h != other.h {
            return false;
        }
        return true;
    }

    fn ne(&self, other :&Self) -> bool {
        return  !self.eq(other);
    }
}

#[allow(dead_code)]
impl CurveFp {
    pub fn new(p :&BigInt, a :&BigInt, b :&BigInt, h :&BigInt) -> Self {
        Self {
            p : p.clone(),
            a : a.clone(),
            b : b.clone(),
            h : h.clone(),
        }
    }

    pub fn p(&self) -> BigInt {
        return self.p.clone();
    }

    pub fn a(&self) -> BigInt {
        return self.a.clone();
    }

    pub fn b(&self) -> BigInt {
        return self.b.clone();
    }

    pub fn h(&self) -> BigInt {
        return self.h.clone();
    }

    pub fn cofactor(&self) -> BigInt {
        return self.h.clone();
    }

    pub fn constains_point(&self,x :&BigInt, y :&BigInt) -> bool {
        let _val :BigInt = (y * y - ((x * x + &self.a) * x  + &self.b)) % &self.p;
        let val :BigInt = mod_with_sign(&_val,&self.p);
        return val == zero();
    }
}

#[derive(Clone)]
pub struct ECCPoint {
    infinity : bool,
    curve : Option<CurveFp>,
    x :Option<BigInt>,
    y :Option<BigInt>,
    order :Option<BigInt>,
}

impl std::fmt::Debug for ECCPoint {
    fn fmt(&self,f :&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s :String = "".to_string();
        s.push_str("ECCPoint(");
        if self.infinity {
            s.push_str("infinity");
        } else {
            s.push_str(&(format!("curve : {:?};",self.curve.as_ref().unwrap().clone())));
            s.push_str(&(format!("x : 0x{:x};", self.x())));
            s.push_str(&(format!("y : 0x{:x};", self.y())));
            if self.order.is_none() {
                s.push_str(&(format!("order : none;")));
            } else {
                s.push_str(&(format!("order : 0x{:x};",self.order.as_ref().unwrap().clone())));
            }
        }
        s.push_str(")");
        write!(f,"{}",&s)
    }
}

impl ECCPoint {
    pub fn new(curve :Option<CurveFp>, x :Option<BigInt>,y :Option<BigInt>, order :Option<BigInt>) -> Self {
        let retv :ECCPoint;
        if curve.is_some()  && x.is_some() && y.is_some()  {
            let cv :CurveFp = curve.as_ref().unwrap().clone();
            let xv :BigInt = x.as_ref().unwrap().clone();
            let yv :BigInt = y.as_ref().unwrap().clone();
            assert!(cv.constains_point(&xv,&yv));
            retv = ECCPoint {
                infinity : false,
                curve :Some(curve.as_ref().unwrap().clone()),
                x : Some(x.as_ref().unwrap().clone()),
                y : Some(y.as_ref().unwrap().clone()),
                order :order,
            };
        } else {
            retv = ECCPoint {
                infinity : true,
                curve : None,
                x : None,
                y : None,
                order : None,
            };
        }

        return retv;
    }

    pub fn infinity() -> ECCPoint {
        let retv = ECCPoint {
            infinity : true,
            curve  : None,
            x : None,
            y : None,
            order : None,
        };
        return retv;
    }

    pub fn negative(&self) -> ECCPoint {
        let mut ocurve :Option<CurveFp> = None;
        let mut ox :Option<BigInt> = None;
        let mut oy :Option<BigInt> = None;
        let oo :Option<BigInt> = None;
        if self.curve.is_some() {
            ocurve = Some(self.curve.as_ref().unwrap().clone());
        }
        if self.x.is_some() {
            ox = Some(self.x.as_ref().unwrap().clone());
        }


        if self.y.is_some() && self.curve.is_some() {
            let yy :BigInt = self.y.as_ref().unwrap().clone();
            let cp :BigInt = self.curve.as_ref().unwrap().p();
            oy = Some(cp - &yy);
        }
        return ECCPoint::new(ocurve,ox,oy,oo);
    }

    pub fn double(&self) -> ECCPoint {
        if self.infinity {
            return ECCPoint::infinity();
        }
        let ocurve :Option<CurveFp> = Some(self.curve.as_ref().unwrap().clone());
        let p :BigInt = self.curve.as_ref().unwrap().p();
        let a :BigInt = self.curve.as_ref().unwrap().a();
        let x :BigInt = self.x.as_ref().unwrap().clone();
        let y :BigInt = self.y.as_ref().unwrap().clone();

        let l :BigInt = ((&x * &x * (3) + &a ) * inverse_mod(&(&y * (2)),&p) ) % (&p);

        let _x3 :BigInt = (&l * &l - &x * (2)) % (&p);
        let x3 :BigInt = mod_with_sign(&_x3,&p);
        let _y3 :BigInt = (&l * (&x - &x3) - &y) % (&p);
        let y3 :BigInt = mod_with_sign(&_y3,&p);

        return ECCPoint::new(ocurve,Some(x3.clone()),Some(y3.clone()),None);
    }

    pub fn x(&self) -> BigInt {
        if self.x.is_none() {
            return zero::<BigInt>();
        }
        return self.x.as_ref().unwrap().clone();
    }

    pub fn y(&self) -> BigInt {
        if self.y.is_none() {
            return zero::<BigInt>();
        }
        return self.y.as_ref().unwrap().clone();
    }

    pub fn curve(&self) -> CurveFp {
        if self.curve.is_none() {
            let zv :BigInt = zero::<BigInt>();
            return CurveFp::new(&zv,&zv,&zv,&zv);
        }
        return self.curve.as_ref().unwrap().clone();
    }

    pub fn order(&self) -> BigInt {
        if self.order.is_none() {
            return zero::<BigInt>();
        }
        return self.order.as_ref().unwrap().clone();
    }

    pub fn add_point(&self, other :Self) -> Self {
        if other.infinity {
            return self.clone();
        }
        if self.infinity {
            return other.clone();
        }
        let scurve :CurveFp = self.curve.as_ref().unwrap().clone();
        let ocurve :CurveFp = other.curve.as_ref().unwrap().clone();
        assert!(scurve == ocurve);
        let sx :BigInt = self.x.as_ref().unwrap().clone();
        let ox :BigInt = other.x.as_ref().unwrap().clone();
        let sy :BigInt = self.y.as_ref().unwrap().clone();
        let oy :BigInt = other.y.as_ref().unwrap().clone();
        let sp :BigInt = scurve.p();
        if sx == ox {
            if ((sy + &oy) % (&sp)) == zero() {
                return ECCPoint::infinity();
            } else {
                return self.double();
            }
        }

        let _l :BigInt = ((oy - &sy ) * inverse_mod(&((&ox) - (&sx)),&sp)) % (&sp);
        let l :BigInt = mod_with_sign(&_l,&sp);

        let _x3 :BigInt = (&l * &l - &sx - &ox ) % (&sp);
        let x3 :BigInt = mod_with_sign(&_x3,&sp);
        let _y3 :BigInt = ((&l) * (&sx - &x3) - &sy) % (&sp);
        let y3 :BigInt = mod_with_sign(&_y3,&sp);
        return ECCPoint::new(Some(scurve.clone()),Some(x3.clone()),Some(y3.clone()), None);
    }
    
    pub fn isinfinity(&self) -> bool {
    	return self.infinity;
    }

    pub fn multiply_int(&self, order :&BigInt) -> Self {
        let e :BigInt = order.clone();
        let mut corder :BigInt = zero();
        if self.order.is_some() {
            corder = self.order.as_ref().unwrap().clone();
        }
        if e == zero() || (self.order.is_some() &&  (&e % &corder) == zero() ) {
            return ECCPoint::infinity();
        }
        if self.infinity {
            return ECCPoint::infinity();
        }
        if e < zero() {
            let nege :BigInt = -e;
            return self.negative().multiply_int(&nege);
        }
        let (bplus,vecs) = e.to_bytes_be();
        let mut e3 :BigInt = BigInt::from_bytes_be(bplus,&vecs);
        e3 *= 3;
        let oy :Option<BigInt> = Some(- self.y.as_ref().unwrap().clone());
        let ocurve :Option<CurveFp> = Some(self.curve.as_ref().unwrap().clone());
        let ox :Option<BigInt> = Some(self.x.as_ref().unwrap().clone());
        let mut oo :Option<BigInt> = None;
        if self.order.is_some() {
            oo = Some(self.order.as_ref().unwrap().clone());
        }
        let negative_self :ECCPoint = ECCPoint::new(ocurve,ox,oy,oo);

        let mut ileft :BigInt = leftmost_bit(&e3) / 2;

        let mut result :ECCPoint = self.clone();


        while ileft > one() {
            let (bplus,vecs) = e3.to_bytes_be();
            let ce3 = BigInt::from_bytes_be(bplus,&vecs);
            let (bplus,vecs) = e.to_bytes_be();
            let ce = BigInt::from_bytes_be(bplus,&vecs);
            result = result.double();
            if (ce3 & (&ileft)) != zero() && (ce & (&ileft)) == zero() {
                result = result + self.clone();
            }

            let (bplus,vecs) = e3.to_bytes_be();
            let ce3 = BigInt::from_bytes_be(bplus,&vecs);
            let (bplus,vecs) = e.to_bytes_be();
            let ce = BigInt::from_bytes_be(bplus,&vecs);
            if (ce3 & (&ileft)) == zero() && (ce & (&ileft)) != zero() {
                result = result + negative_self.clone();
            }

            ileft = ileft / 2;
        }

        return result;
    }
}

impl PartialEq for ECCPoint {
    fn eq(&self, other :&Self) -> bool {
        if self.infinity && self.infinity == other.infinity {
            return true;
        } else if self.infinity != other.infinity {
            return false;
        }
        let sv :&CurveFp = self.curve.as_ref().unwrap();
        let ov :&CurveFp = other.curve.as_ref().unwrap();

        if !sv.eq(ov) { 
            return false;
        }

        let sx :&BigInt = self.x.as_ref().unwrap();
        let ox :&BigInt = other.x.as_ref().unwrap();
        if sx != ox {
            return false;
        }

        let sy :&BigInt = self.y.as_ref().unwrap();
        let oy :&BigInt = other.y.as_ref().unwrap();
        if sy != oy {
            return false;
        }

        let so :&BigInt = self.order.as_ref().unwrap();
        let oo :&BigInt = other.order.as_ref().unwrap();
        if so != oo {
            return false;
        }


        return true;
    }
}

impl std::ops::Add for ECCPoint {
    type Output = Self;
    fn add(self, other :Self) -> Self {
        return (&self).add_point(other);
    }
}

impl std::ops::Mul<BigInt> for ECCPoint {
    type Output = Self;
    fn mul(self, other :BigInt) -> Self {
        return (&self).multiply_int(&other);
    }
}


#[derive(Clone)]
pub struct PointJacobi {
    infinity : bool,
    curve :CurveFp,
    coords : (BigInt,BigInt,BigInt),
    order :Option<BigInt>,
    generator :bool,
    precompute : Vec<(BigInt,BigInt)>,
}

impl std::fmt::Debug for PointJacobi {
    fn fmt(&self,f :&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s :String = "".to_string();
        s.push_str("PointJacobi(");
        if self.infinity {
            s.push_str("infinity");
        } else {
            s.push_str(&(format!("curve : {:?};",self.curve)));
            s.push_str(&(format!("coords (0x{:x},0x{:x},0x{:x});",self.coords.0,self.coords.1,self.coords.2)));
            if self.order.is_none() {
                s.push_str("order : none;");
            } else {
                s.push_str(&(format!("order : 0x{:x};",self.order.as_ref().unwrap().clone())));
            }
            s.push_str(&(format!("generator : {:?}",self.generator)));
        }
        s.push_str(")");
        write!(f,"{}",&s)
    }
}

#[allow(non_snake_case)]
impl PointJacobi {
    pub fn infinity() -> Self {
        let zv :BigInt = zero();
        PointJacobi {
            infinity : true,
            curve : CurveFp::new(&zv,&zv,&zv,&zv),
            order: None,
            coords : (zero(),zero(),zero()),
            generator : false,
            precompute : Vec::new(),
        }
    }

    pub fn curve(&self) -> CurveFp {
        return self.curve.clone();
    }

    pub fn new(curve :&CurveFp, x :&BigInt, y :&BigInt, z :&BigInt,order :Option<BigInt>,generator :bool) -> Self {
        if order.is_none() {
            PointJacobi {
                infinity : false,
                curve :curve.clone(),
                order : None,
                coords : (x.clone(),y.clone(),z.clone()),
                generator : generator,
                precompute : Vec::new(),
            }
        } else {
            PointJacobi {
                infinity : false,
                curve : curve.clone(),
                order : Some(order.as_ref().unwrap().clone()),
                coords : (x.clone(),y.clone(),z.clone()),
                generator : generator,
                precompute : Vec::new(),
            }
        }
    }

    pub fn x(&self) -> BigInt {
        let (x,_ ,z) = self.coords.clone();
        if z == one::<BigInt>() {
            return x;
        }
        let p:BigInt = self.curve.p();
        let cz :BigInt = inverse_mod(&z,&p);
        return (x * (&cz * &cz)) % p;
    }

    pub fn y(&self) -> BigInt {
        let (_,y,z) = self.coords.clone();
        if z == one::<BigInt>() {
            return y;
        }
        let p :BigInt = self.curve.p();
        let cz :BigInt = inverse_mod(&z,&p);
        return (y * (&cz * &cz * &cz)) % p;
    }

    fn _double_with_z_1(&self, X1 :&BigInt, Y1 :&BigInt,p :&BigInt, a :&BigInt) -> (BigInt,BigInt,BigInt) {
        let XX :BigInt = (X1 * X1 ) % p;
        let YY :BigInt = (Y1 * Y1) % p;
        if YY == zero() {
            return (zero(),zero(),one());
        }
        let YYYY :BigInt = (&YY * &YY) % p;
        let _S :BigInt = (((X1 + &YY) * (X1 + &YY) - &XX - &YYYY) * 2) % p;
        let S :BigInt = mod_with_sign(&_S,p);
        let M :BigInt = (&XX * 3 ) + a;
        let _T :BigInt = (&M * &M - &S * 2) % p;
        let T :BigInt = mod_with_sign(&_T,p);
        let _Y3 :BigInt = (&M * (&S - &T) - &YYYY * 8) % p;
        let Y3 :BigInt = mod_with_sign(&_Y3,p);
        let Z3 :BigInt = (Y1 * 2) % p;
        return (T, Y3 ,Z3);
    }

    fn _double(&self,X1 :&BigInt, Y1 :&BigInt,Z1 :&BigInt,p :&BigInt, a :&BigInt) -> (BigInt,BigInt,BigInt) {
        if  one::<BigInt>().eq(Z1)  {
            ecsimple_log_trace!(" ");
            return self._double_with_z_1(X1,Y1,p,a);
        }
        if zero::<BigInt>().eq(Y1) || zero::<BigInt>().eq(Z1) {
            ecsimple_log_trace!(" ");
            return (zero(),zero(),one());
        }
        let XX :BigInt = (X1 * X1 ) % p;
        let YY :BigInt = (Y1 * Y1) % p;

        if YY == zero() {
            ecsimple_log_trace!(" ");
            return (zero(),zero(),one());
        }
        let YYYY :BigInt = (&YY * &YY) % p;
        let ZZ :BigInt = (Z1 * Z1) % p;
        let _S :BigInt = (((X1 + &YY) * (X1 + &YY) - &XX - &YYYY) * 2) % p;
        let S :BigInt = mod_with_sign(&_S,p);
        let _M :BigInt = ((&XX * 3 ) + (a * &ZZ *&ZZ)) % p;
        let M :BigInt = mod_with_sign(&_M,p);
        let _T :BigInt = (&M * &M - &S * 2) % p;
        let T :BigInt = mod_with_sign(&_T,p);
        let _Y3 :BigInt = (&M * (&S - &T) - &YYYY * 8) % p;
        let Y3 :BigInt = mod_with_sign(&_Y3,p);
        let _Z3 :BigInt = ((Y1 + Z1) *(Y1 + Z1) - &YY - &ZZ ) % p;
        let Z3 :BigInt = mod_with_sign(&_Z3,p);
        ecsimple_log_trace!(" ");
        return (T, Y3 ,Z3);
    }

    pub fn double(&self) -> PointJacobi {
        let (X1,Y1,Z1) = self.coords.clone();
        if Y1 == zero() {
            return PointJacobi::infinity();
        }
        let p :BigInt = self.curve.p();
        let a :BigInt = self.curve.a();
        let (X3,Y3,Z3) = self._double(&X1,&Y1,&Z1,&p,&a);
        ecsimple_log_trace!("X3 0x{:x} Y3 0x{:x} Z3 0x{:x}",X3,Y3,Z3);

        if Y3 == zero() || Z3 == zero() {
            ecsimple_log_trace!("INFINITY");
            return PointJacobi::infinity();
        }

        let mut oo :Option<BigInt> = None;
        if self.order.is_some() {
            oo = Some(self.order.as_ref().unwrap().clone());
        }

        return PointJacobi::new(&self.curve,&X3,&Y3,&Z3,oo,false);
    }

    pub fn scale(&mut self) -> PointJacobi {
        let (x,y,z) = self.coords.clone();
        if one::<BigInt>().eq(&z) {
            return self.clone();
        }
        let p :BigInt = self.curve.p();
        let z_inv :BigInt = inverse_mod(&z,&p);
        let zz_inv :BigInt = (&z_inv * &z_inv) % (&p);
        let x1 :BigInt = ((&x) * &zz_inv) % (&p);
        let y1 :BigInt = ((&y) * &zz_inv * &z_inv) % (&p);
        let z1 :BigInt = one::<BigInt>();
        self.coords = (x1.clone(),y1.clone(),z1.clone());
        ecsimple_log_trace!("z_inv 0x{:x} zz_inv 0x{:x} x 0x{:x} y 0x{:x} p 0x{:x}",z_inv,zz_inv,x1,y1,p);
        return self.clone();
    }

    pub fn order(&self) -> BigInt {
        if self.order.is_none() {
            return zero::<BigInt>();
        }
        return self.order.as_ref().unwrap().clone();
    }


    fn _maybe_precompute(&mut self) {
        if !self.generator || self.precompute.len() > 0 {
            return;
        }
        assert!(self.order.is_some());
        let mut order :BigInt = self.order.as_ref().unwrap().clone();
        let mut precompute :Vec<(BigInt,BigInt)> = Vec::new();
        let mut i :BigInt = one::<BigInt>();
        order *= 2;
        let (x,y,z) = self.coords.clone();
        let mut doubler :PointJacobi = PointJacobi::new(&self.curve,&x,&y,&z,Some(order.clone()),false);
        order *= 2;
        ecsimple_log_trace!("[{}] x 0x{:x} y 0x{:x}",precompute.len(), doubler.x(),doubler.y());
        precompute.push((doubler.x(),doubler.y()));
        while i < order {
            i *= 2;
            doubler = doubler.double().scale();
            ecsimple_log_trace!("[{}] x 0x{:x} y 0x{:x}",precompute.len(), doubler.x(),doubler.y());
            precompute.push((doubler.x(),doubler.y()));
        }
        self.precompute = precompute;
        return;
    }

    pub fn eq_jacobipoint(&self, other :&PointJacobi) -> bool {
        let (x1,y1,z1) = self.coords.clone();
        if other.infinity {
            let retval :bool = zero::<BigInt>().eq(&z1) || zero::<BigInt>().eq(&y1);
            return retval;
        }
        let (x2,y2,z2) = other.coords.clone();
        if self.curve != other.curve {
            return false;
        }
        let p :BigInt = self.curve.p();
        let zz1 :BigInt = ((&z1) * (&z1)) % (&p);
        let zz2 :BigInt = ((&z2) * (&z2)) % (&p);

        let mut retval = true;
        let _xres :BigInt = ((&x1) * (&zz2) - (&x2) * (&zz1)) % (&p);
        let xres :BigInt = mod_with_sign(&_xres,&p);
        if !zero::<BigInt>().eq(&xres) {
            retval = false;
        }

        let _yres :BigInt = ((&y1) * (&zz2) * (&z2) - (&y2) * (&zz1) * (&z1)) % (&p);
        let yres :BigInt = mod_with_sign(&_yres,&p);
        if !zero::<BigInt>().eq(&yres) {
            retval = false;
        }
        return retval;        
    }

    pub fn eq_point(&self, other :&ECCPoint) -> bool {
        let (x1,y1,z1) = self.coords.clone();
        if other.infinity {
            let retval :bool = zero::<BigInt>().eq(&z1) || zero::<BigInt>().eq(&y1);
            return retval;
        }
        let x2 :BigInt = other.x();
        let y2 :BigInt = other.y();
        let z2 :BigInt = one::<BigInt>();
        if self.curve != other.curve() {
            return false;
        }
        let p :BigInt = self.curve.p();
        let zz1 :BigInt = ((&z1) * (&z1)) % (&p);
        let zz2 :BigInt = ((&z2) * (&z2)) % (&p);

        let mut retval = true;
        let _xres :BigInt = ((&x1) * (&zz2) - (&x2) * (&zz1)) % (&p);
        let xres :BigInt = mod_with_sign(&_xres,&p);
        if !zero::<BigInt>().eq(&xres) {
            retval = false;
        }

        let _yres :BigInt = ((&y1) * (&zz2) * (&z2) - (&y2) * (&zz1) * (&z1)) % (&p);
        let yres :BigInt = mod_with_sign(&_yres,&p);
        if !zero::<BigInt>().eq(&yres) {
            retval = false;
        }
        return retval;        
    }

    pub fn to_affine(&mut self) -> ECCPoint {
        let (_ , y ,z) = self.coords.clone();
        if zero::<BigInt>().eq(&y) || zero::<BigInt>().eq(&z) {
            return ECCPoint::infinity();
        }
        let _ = self.scale();
        let (x,y,_) = self.coords.clone();
        let ocurve :Option<CurveFp> = Some(self.curve.clone());
        let ox :Option<BigInt> = Some(x);
        let oy :Option<BigInt> = Some(y);
        let mut oo :Option<BigInt> = None;
        if self.order.is_some() {
            oo = Some(self.order());
        }
        return ECCPoint::new(ocurve,ox,oy,oo);
    }


    pub fn from_affine(pt :&ECCPoint,generator :bool) -> PointJacobi {
        let z :BigInt = one::<BigInt>();
        let oo :Option<BigInt> = Some(pt.order());
        return PointJacobi::new(&(pt.curve()),&(pt.x()),&(pt.y()),&z,oo,generator);
    }

    /*http://hyperelliptic.org/EFD/g1p/auto-shortw-jacobian.html#addition-mmadd-2007-bl*/
    fn _add_with_z_1(&self,X1 :&BigInt, Y1 :&BigInt,X2 :&BigInt, Y2 :&BigInt,p :&BigInt) -> (BigInt,BigInt,BigInt) {
        let H :BigInt = X2 - X1;
        let HH :BigInt = (&H) * (&H);
        let I :BigInt = (&HH * 4) % p;
        let J :BigInt = (&H) * (&I);
        let r :BigInt = (Y2 - Y1) * 2;
        if zero::<BigInt>().eq(&H) || zero::<BigInt>().eq(&r) {
            return self._double_with_z_1(X1,Y1,p,&(self.curve.a()));
        }
        let V :BigInt = X1 * (&I);
        let _X3 :BigInt = ((&r) * (&r) - &J - (&V) * 2) % p;
        let X3 :BigInt = mod_with_sign(&_X3,p);
        let _Y3 :BigInt = ((&r) * (&V - &X3) - Y1 * (&J) * 2) % p;
        let Y3 :BigInt = mod_with_sign(&_Y3,p);
        let Z3 :BigInt = ((&H) *2) % p;
        return (X3,Y3,Z3);
    }

    /*http://hyperelliptic.org/EFD/g1p/auto-shortw-jacobian.html#addition-zadd-2007-m*/
    fn _add_with_z_eq(&self,X1 :&BigInt, Y1 :&BigInt,Z1 :&BigInt,X2 :&BigInt,Y2 :&BigInt , p :&BigInt) -> (BigInt,BigInt,BigInt) {
        let _A :BigInt = ((X2 - X1 ) * (X2 - X1)) % p;
        let A :BigInt = mod_with_sign(&_A,p);
        let B :BigInt = (X1 * (&A)) % p;
        let C :BigInt = X2 * (&A);
        let _D :BigInt = ((Y2 - Y1) * (Y2 - Y1)) % p;
        let D :BigInt = mod_with_sign(&_D,p);
        if zero::<BigInt>().eq(&A) || zero::<BigInt>().eq(&D) {
            return self._double(X1,Y1,Z1,p,&(self.curve.a()));
        } 
        let _X3 :BigInt = ((&D) - (&B) - (&C)) % p;
        let X3 :BigInt = mod_with_sign(&_X3,p);
        let _Y3 :BigInt = ((Y2 - Y1) * ((&B) - (&X3)) - Y1 * ((&C) - (&B))) % p;
        let Y3 :BigInt = mod_with_sign(&_Y3,p);
        let _Z3 :BigInt = (Z1 * (X2 - X1)) % p;
        let Z3 :BigInt = mod_with_sign(&_Z3,p);
        return (X3, Y3,Z3);
    }

    fn _add_with_z2_1(&self,X1 :&BigInt, Y1 :&BigInt,Z1 :&BigInt,X2 :&BigInt,Y2 :&BigInt , p :&BigInt) -> (BigInt,BigInt,BigInt) {
        let Z1Z1 :BigInt = (Z1 * Z1) % p;
        let (U2,S2) :(BigInt,BigInt) = ((X2 * &(Z1Z1))% p , (Y2 * Z1 * (&Z1Z1)) % p);
        let _H :BigInt = ((&U2) - X1) % p;
        let H :BigInt = mod_with_sign(&_H,p);
        let HH :BigInt = ((&H) * (&H)) % p;
        let I :BigInt = ((&HH) * 4) % p;
        let J :BigInt = (&H) * (&I);
        let _r :BigInt = (((&S2) - Y1)*2) % p;
        let r :BigInt = mod_with_sign(&_r,p);
        if zero::<BigInt>().eq(&r) || zero::<BigInt>().eq(&H) {
            ecsimple_log_trace!("r or H == 0");
            return self._double_with_z_1(X2,Y2,p,&(self.curve.a()));
        }

        let V :BigInt = X1 * (&I);
        let _X3 :BigInt = ((&r) * (&r) - &J - &V * 2) % p;
        let X3 :BigInt = mod_with_sign(&_X3,p);
        let _Y3 :BigInt = ((&r) * (&V - &X3) - Y1 * (&J) * 2) % p;
        let Y3 :BigInt = mod_with_sign(&_Y3,p);
        let _Z3 :BigInt = ((Z1 + (&H)) * (Z1 + (&H)) - &Z1Z1 - &HH) % p;
        let Z3 :BigInt = mod_with_sign(&_Z3,p);
        ecsimple_log_trace!("H 0x{:x} HH 0x{:x} I 0x{:x} J 0x{:x} r 0x{:x}",H,HH,I,J,r);
        ecsimple_log_trace!("V 0x{:x} X3 0x{:x} Y3 0x{:x} Z3 0x{:x}",V,X3,Y3,Z3);
        return (X3,Y3,Z3);
    }

    fn _add_with_z_ne(&self,X1 :&BigInt, Y1 :&BigInt,Z1 :&BigInt,X2 :&BigInt,Y2 :&BigInt ,Z2 :&BigInt , p :&BigInt) -> (BigInt,BigInt,BigInt) {
        let Z1Z1 :BigInt = (Z1 * Z1) % p;
        let Z2Z2 :BigInt = (Z2 * Z2) % p;
        let U1 :BigInt = (X1 * (&Z2Z2)) % p;
        let U2 :BigInt = (X2 * (&Z1Z1)) % p;
        let S1 :BigInt = (Y1 * Z2 * (&Z2Z2)) % p;
        let S2 :BigInt = (Y2 * Z1 * (&Z1Z1)) % p;
        let H :BigInt = (&U2) - (&U1);
        let I :BigInt = ((&H) * (&H) * 4) % p;
        let _J :BigInt = ((&H) * (&I)) % p;
        let J :BigInt = mod_with_sign(&_J,p);
        let _r :BigInt = (((&S2) - (&S1)) * 2) % p;
        let r :BigInt = mod_with_sign(&_r,p);
        if zero::<BigInt>().eq(&r) || zero::<BigInt>().eq(&H) {
            ecsimple_log_trace!("not H r");
            return self._double(X1,Y1,Z1,p,&(self.curve.a()));   
        }
        let V :BigInt = (&U1) * (&I);
        let _X3 :BigInt = ((&r) * (&r) - (&J) - (&V) * 2) % p;
        let X3 :BigInt = mod_with_sign(&_X3,p);
        let _Y3 :BigInt = (((&r) * (&V - &X3)) - (&S1) * (&J) * 2) % p;
        let Y3 :BigInt = mod_with_sign(&_Y3,p);
        let _Z3 :BigInt = (((Z1 + Z2) * (Z1 + Z2) - &Z1Z1 - &Z2Z2) * &H) % p;
        let Z3 :BigInt = mod_with_sign(&_Z3,p);
        ecsimple_log_trace!("Z1Z1 0x{:x} Z2Z2 0x{:x} U1 0x{:x} U2 0x{:x}",Z1Z1,Z2Z2,U1,U2);
        ecsimple_log_trace!("S1 0x{:x} S2 0x{:x} H 0x{:x} I 0x{:x}",S1,S2,H,I);
        ecsimple_log_trace!("J 0x{:x} r 0x{:x} V 0x{:x}",J,r,V);
        ecsimple_log_trace!("X3 0x{:x} Y3 0x{:x} Z3 0x{:x}",X3,Y3,Z3);
        return (X3,Y3,Z3);
    }

    fn _add(&self,X1 :&BigInt, Y1 :&BigInt,Z1 :&BigInt,X2 :&BigInt,Y2 :&BigInt ,Z2 :&BigInt , p :&BigInt) -> (BigInt,BigInt,BigInt) {
        let zv :BigInt = zero::<BigInt>();
        let ov :BigInt = one::<BigInt>();
        if zv.eq(Y1) || zv.eq(Z1) {
            ecsimple_log_trace!(" ");
            return (X2.clone(),Y2.clone(),Z2.clone());
        }
        if zv.eq(Y2) || zv.eq(Z2) {
            ecsimple_log_trace!(" ");
            return (X1.clone(),Y1.clone(),Z1.clone());
        }

        if Z1 == Z2 {
            if ov.eq(Z1) {
                ecsimple_log_trace!(" ");
                return self._add_with_z_1(X1,Y1,X2,Y2,p);
            }
            ecsimple_log_trace!(" ");
            return self._add_with_z_eq(X1,Y1,Z1,X2,Y2,p);
        }
        if ov.eq(Z1) {
            ecsimple_log_trace!(" ");
            return self._add_with_z2_1(X2, Y2, Z2, X1, Y1, p);
        }

        if ov.eq(Z2) {
            ecsimple_log_trace!(" ");
            return self._add_with_z2_1(X1, Y1, Z1, X2, Y2, p);
        }

        ecsimple_log_trace!(" ");
        return self._add_with_z_ne(X1, Y1, Z1, X2, Y2, Z2, p);
    }

    fn _add_with_other(&self,other :&PointJacobi) -> PointJacobi {
        let ov :BigInt = zero::<BigInt>();
        assert!(self.curve.eq(&(other.curve)));
        let p :BigInt = self.curve.p();
        let (X1,Y1,Z1) = self.coords.clone();
        let (X2,Y2,Z2) = other.coords.clone();
        ecsimple_log_trace!("X1 0x{:x} Y1 0x{:x} Z1 0x{:x} X2 0x{:x} Y2 0x{:x} Z2 0x{:x} p 0x{:x}",X1,Y1,Z1,X2,Y2,Z2,p);

        let (X3,Y3,Z3) = self._add(&X1, &Y1, &Z1, &X2, &Y2, &Z2, &p);
        ecsimple_log_trace!("X3 0x{:x} Y3 0x{:x} Z3 0x{:x}",X3,Y3,Z3);
        if ov.eq(&Y3) || ov.eq(&Z3) {
            return PointJacobi::infinity();
        }
        let mut oo :Option<BigInt> = None;
        if self.order.is_some() {
            oo = Some(self.order.as_ref().unwrap().clone());
        }
        return PointJacobi::new(&(self.curve),&X3,&Y3,&Z3,oo,false);        
    }

    pub fn add_jacobi(&self,other :&PointJacobi) -> PointJacobi {
        if self.infinity {
            return other.clone();
        }
        if other.infinity {
            return self.clone();
        }
        return self._add_with_other(other);
    }

    pub fn add_point(&self,opoint :&ECCPoint) -> PointJacobi {
        if self.infinity {
            return PointJacobi::from_affine(&opoint,false);
        }
        if opoint.isinfinity() {
            return self.clone();
        }

        let other :PointJacobi = PointJacobi::from_affine(&opoint,false);
        return self._add_with_other(&other);
    }

    fn _mul_precompute(&self,o3 :&BigInt) -> PointJacobi {
        let zv :BigInt = zero::<BigInt>();
        let ov :BigInt = one::<BigInt>();
        let tv :BigInt = (&ov) + (&ov);
        let mut other :BigInt = o3.clone();
        let (mut X3,mut Y3,mut Z3) : (BigInt,BigInt,BigInt) = (zv.clone(),zv.clone(),ov.clone());
        let p :BigInt = self.curve.p();
        for (X2,Y2) in self.precompute.iter() {
            let negY2 :BigInt = - Y2;
            if zv.ne(&((&other) % 2)) {
                if ((&other) % 4 ) >= tv {
                    other = (&other + 1) / 2;
                    (X3,Y3,Z3) = self._add(&X3,&Y3,&Z3,X2,&negY2,&ov,&p);
                } else {
                    other = (&other - 1) / 2;
                    (X3,Y3,Z3) = self._add(&X3,&Y3,&Z3,X2,Y2,&ov,&p);
                }
            } else {
                other /= 2;
            }
        }

        if zv.eq(&Y3) || zv.eq(&Z3) {
            return PointJacobi::infinity();
        }

        let mut oo :Option<BigInt> = None;
        if self.order.is_some() {
            oo = Some(self.order.as_ref().unwrap().clone());
        }

        return PointJacobi::new(&self.curve,&X3,&Y3,&Z3,oo,false);
    }

    fn _naf(&self,m3 :&BigInt) -> Vec<i32> {
        let mut retv :Vec<i32> = Vec::new();
        let zv :BigInt = zero::<BigInt>();
        let ov :BigInt = one::<BigInt>();
        let fv :BigInt = (&ov) + (&ov) + (&ov) + (&ov);
        let mut mult :BigInt = m3.clone();

        while !zv.eq(&mult) {
            if !zv.eq(&((&mult) % 2)) {
                let ndb :BigInt = (&mult) % (&fv);
                let (_, vecs) = ndb.to_bytes_be();
                let vl :usize = vecs.len();
                let mut nd :i32 = vecs[vl - 1] as i32;
                if nd >= 2 {
                    nd -= 4;
                }
                retv.push(nd);
                mult -= nd;
            } else {
                retv.push(0);
            }
            mult /= 2;
        }
        return retv;
    }

    pub fn mul_int(&mut self,o3 :&BigInt) -> PointJacobi {
        let zv :BigInt = zero::<BigInt>();
        let ov :BigInt = one::<BigInt>();
        let mut ordv :BigInt = zv.clone();
        let mut other :BigInt = o3.clone();
        if self.infinity || zv.eq(&other) {
            return PointJacobi::infinity();
        }
        if ov.eq(&other) {
            return self.clone();
        }

        if self.order.is_some() {
            ordv = self.order.as_ref().unwrap().clone();
        }

        if !zv.eq(&ordv) {
            other = (&other) % ((&ordv) * 2);
        }

        let _ = self._maybe_precompute();
        if self.precompute.len() > 0 {
            return self._mul_precompute(&other);
        }
        let _ = self.scale();
        let (X2,Y2,_)  = self.coords.clone();
        ecsimple_log_trace!("X2 0x{:x} Y2 0x{:x}", X2,Y2);
        let (mut X3,mut Y3,mut Z3) = (zv.clone(),zv.clone(),ov.clone());
        let (p,a) = (self.curve.p(),self.curve.a());
        let mut nafvecs :Vec<i32> = self._naf(&other);
        let negY2 :BigInt = - (&Y2);
        let mut idx :i32 = 0;
        nafvecs.reverse();
        for i in nafvecs {
            ecsimple_log_trace!("[{}][{}] (X3 :0x{:x} , Y3 : 0x{:x}, Z3 : 0x{:x}) ",idx, i, X3,Y3,Z3);
            (X3,Y3,Z3) = self._double(&X3,&Y3,&Z3,&p,&a);
            ecsimple_log_trace!("after[{}][{}] (X3 :0x{:x} , Y3 : 0x{:x}, Z3 : 0x{:x}) ",idx, i, X3,Y3,Z3);
            if i < 0 {
                (X3,Y3,Z3) = self._add(&X3, &Y3, &Z3, &X2, &negY2, &ov, &p);
            } else if i > 0 {
                (X3,Y3,Z3) = self._add(&X3, &Y3, &Z3, &X2, &Y2, &ov, &p);
            }
            ecsimple_log_trace!("last[{}][{}] (X3 :0x{:x} , Y3 : 0x{:x}, Z3 : 0x{:x}) ", idx,i, X3,Y3,Z3);
            idx += 1;
        }

        if zv.eq(&Y3) || zv.eq(&Z3) {
            return PointJacobi::infinity();
        }

        let mut oo :Option<BigInt>= None;
        if self.order.is_some() {
            oo = Some(self.order.as_ref().unwrap().clone());
        }

        return PointJacobi::new(&self.curve,&X3,&Y3,&Z3,oo,false);
    }

}

impl std::cmp::PartialEq<PointJacobi> for PointJacobi {
    fn eq(&self,other :&Self) -> bool {
        return self.eq_jacobipoint(other);
    }

    fn ne(&self, other :&Self) -> bool {
        return  !self.eq(other);
    }
}


impl std::cmp::PartialEq<ECCPoint> for PointJacobi {
    fn eq(&self,other :&ECCPoint) -> bool {
        return self.eq_point(other);
    }

    fn ne(&self, other :&ECCPoint) -> bool {
        return  !self.eq(other);
    }
}

impl std::ops::Add for PointJacobi {
    type Output = Self;
    fn add(self, other :Self) -> Self {
        return (&self).add_jacobi(&other);
    }
}


impl std::ops::Add<ECCPoint> for PointJacobi {
    type Output = Self;
    fn add(self, other :ECCPoint) -> Self {
        return (&self).add_point(&other);
    }
}
