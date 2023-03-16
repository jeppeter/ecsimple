
use num_bigint::{BigInt};
use num_traits::{zero,one};
use crate::arithmetics::*;

#[derive(Clone,Debug)]
pub struct CurveFp {
    pub p :BigInt,
    pub a :BigInt,
    pub b :BigInt,
    pub h :BigInt,
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
        let val :BigInt = (y * y - ((x * x + &self.a) * x  + &self.b)) % &self.p;
        return val == zero();
    }
}

#[derive(Clone,Debug)]
pub struct Point {
    infinity : bool,
    curve : Option<CurveFp>,
    x :Option<BigInt>,
    y :Option<BigInt>,
    order :Option<BigInt>,
}

#[allow(dead_code)]
impl Point {
    pub fn new(curve :Option<CurveFp>, x :Option<BigInt>,y :Option<BigInt>, order :Option<BigInt>) -> Self {
        let retv :Point;
        if curve.is_some()  && x.is_some() && y.is_some()  {
            let cv :CurveFp = curve.as_ref().unwrap().clone();
            let xv :BigInt = x.as_ref().unwrap().clone();
            let yv :BigInt = y.as_ref().unwrap().clone();
            assert!(cv.constains_point(&xv,&yv));
            retv = Point {
                infinity : false,
                curve :Some(curve.as_ref().unwrap().clone()),
                x : Some(x.as_ref().unwrap().clone()),
                y : Some(y.as_ref().unwrap().clone()),
                order :order,
            };
        } else {
            retv = Point {
                infinity : true,
                curve : None,
                x : None,
                y : None,
                order : None,
            };
        }

        return retv;
    }

    pub fn infinity() -> Point {
        let retv = Point {
            infinity : true,
            curve  : None,
            x : None,
            y : None,
            order : None,
        };
        return retv;
    }

    pub fn negative(&self) -> Point {
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
        return Point::new(ocurve,ox,oy,oo);
    }

    pub fn double(&self) -> Point {
        if self.infinity {
            return Point::infinity();
        }
        let ocurve :Option<CurveFp> = Some(self.curve.as_ref().unwrap().clone());
        let p :BigInt = self.curve.as_ref().unwrap().p();
        let a :BigInt = self.curve.as_ref().unwrap().a();
        let x :BigInt = self.x.as_ref().unwrap().clone();
        let y :BigInt = self.y.as_ref().unwrap().clone();

        let l :BigInt = ((&x * &x * 3 + &a ) * inverse_mod(&(&y * 2),&p) ) % (&p);

        let x3 :BigInt = (&l * &l - &x * 2) % (&p);
        let y3 :BigInt = (&l * (&x - &x3) - &y) % (&p);

        return Point::new(ocurve,Some(x3.clone()),Some(y3.clone()),None);
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
                return Point::infinity();
            } else {
                return self.double();
            }
        }

        let l :BigInt = ((oy - &sy ) * inverse_mod(&((&ox) - (&sx)),&sp)) % (&sp);

        let x3 :BigInt = (&l * &l - &sx - &ox ) % (&sp);
        let y3 :BigInt = ((&l) * (&sx - &x3) - &sy) % (&sp);
        return Point::new(Some(scurve.clone()),Some(x3.clone()),Some(y3.clone()), None);
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
            return Point::infinity();
        }
        if self.infinity {
            return Point::infinity();
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
        let negative_self :Point = Point::new(ocurve,ox,oy,oo);

        let mut ileft :BigInt = leftmost_bit(&e3) / 2;

        let mut result :Point = self.clone();


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

impl PartialEq for Point {
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

impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, other :Self) -> Self {
        return (&self).add_point(other);
    }
}

impl std::ops::Mul<BigInt> for Point {
    type Output = Self;
    fn mul(self, other :BigInt) -> Self {
        return (&self).multiply_int(&other);
    }
}


#[allow(dead_code)]
#[derive(Clone,Debug)]
pub struct PointJacobi {
    infinity : bool,
    curve :CurveFp,
    coords : (BigInt,BigInt,BigInt),
    order :Option<BigInt>,
    generator :bool,
    precompute : Vec<(BigInt,BigInt)>,
}

#[allow(non_snake_case)]
#[allow(dead_code)]
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
        let S :BigInt = (((X1 + &YY) * (X1 + &YY) - &XX - &YYYY) * 2) % p;
        let M :BigInt = (&XX * 3 ) + a;
        let T :BigInt = (&M * &M - &S * 2) % p;
        let Y3 :BigInt = (&M * (&S - &T) - &YYYY * 8) % p;
        let Z3 :BigInt = (Y1 * 2) % p;
        return (T, Y3 ,Z3);
    }

    fn _double(&self,X1 :&BigInt, Y1 :&BigInt,Z1 :&BigInt,p :&BigInt, a :&BigInt) -> (BigInt,BigInt,BigInt) {
        if  one::<BigInt>().eq(Z1)  {
            return self._double_with_z_1(X1,Y1,p,a);
        }
        if zero::<BigInt>().eq(Y1) || zero::<BigInt>().eq(Z1) {
            return (zero(),zero(),one());
        }
        let XX :BigInt = (X1 * X1 ) % p;
        let YY :BigInt = (Y1 * Y1) % p;

        if YY == zero() {
            return (zero(),zero(),one());
        }
        let YYYY :BigInt = (&YY * &YY) % p;
        let ZZ :BigInt = (Z1 * Z1) % p;
        let S :BigInt = (((X1 + &YY) * (X1 + &YY) - &XX - &YYYY) * 2) % p;
        let M :BigInt = ((&XX * 3 ) + (a * &ZZ *&ZZ)) % p;
        let T :BigInt = (&M * &M - &S * 2) % p;
        let Y3 :BigInt = (&M * (&S - &T) - &YYYY * 8) % p;
        let Z3 :BigInt = ((Y1 + Z1) *(Y1 + Z1) - &YY - &ZZ ) % p;
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

        if Y3 == zero() || Z3 == zero() {
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
        let y1 :BigInt = ((&y) * &zz_inv) % (&p);
        let z1 :BigInt = one::<BigInt>();
        self.coords = (x1.clone(),y1.clone(),z1.clone());
        return self.clone();
    }

    pub fn order(&self) -> BigInt {
        if self.order.is_none() {
            return zero::<BigInt>();
        }
        return self.order.as_ref().unwrap().clone();
    }


    fn _maybe_precompute(&mut self) {
        if self.order.is_none() || self.precompute.len() == 0 {
            return;
        }
        let mut order :BigInt = self.order.as_ref().unwrap().clone();
        let mut precompute :Vec<(BigInt,BigInt)> = Vec::new();
        let mut i :BigInt = one::<BigInt>();
        order *= 2;
        let (x,y,z) = self.coords.clone();
        let mut doubler :PointJacobi = PointJacobi::new(&self.curve,&x,&y,&z,Some(order.clone()),false);
        order *= 2;
        precompute.push((doubler.x(),doubler.y()));
        while i < order {
            i *= 2;
            doubler = doubler.double().scale();
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
        let xres :BigInt = ((&x1) * (&zz2) - (&x2) * (&zz1)) % (&p);
        if !zero::<BigInt>().eq(&xres) {
            retval = false;
        }

        let yres :BigInt = ((&y1) * (&zz2) * (&z2) - (&y2) * (&zz1) * (&z1)) % (&p);
        if !zero::<BigInt>().eq(&yres) {
            retval = false;
        }
        return retval;        
    }

    pub fn eq_point(&self, other :&Point) -> bool {
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
        let xres :BigInt = ((&x1) * (&zz2) - (&x2) * (&zz1)) % (&p);
        if !zero::<BigInt>().eq(&xres) {
            retval = false;
        }

        let yres :BigInt = ((&y1) * (&zz2) * (&z2) - (&y2) * (&zz1) * (&z1)) % (&p);
        if !zero::<BigInt>().eq(&yres) {
            retval = false;
        }
        return retval;        
    }

    pub fn to_affine(&mut self) -> Point {
        let (_ , y ,z) = self.coords.clone();
        if zero::<BigInt>().eq(&y) || zero::<BigInt>().eq(&z) {
            return Point::infinity();
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
        return Point::new(ocurve,ox,oy,oo);
    }


    pub fn from_affine(pt :&Point,generator :bool) -> PointJacobi {
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
        let X3 :BigInt = ((&r) * (&r) - &J - (&V) * 2) % p;
        let Y3 :BigInt = ((&r) * (&V - &X3) - Y1 * (&J) * 2) % p;
        let Z3 :BigInt = ((&H) *2) % p;
        return (X3,Y3,Z3);
    }

    /*http://hyperelliptic.org/EFD/g1p/auto-shortw-jacobian.html#addition-zadd-2007-m*/
    fn _add_with_z_eq(&self,X1 :&BigInt, Y1 :&BigInt,Z1 :&BigInt,X2 :&BigInt,Y2 :&BigInt , p :&BigInt) -> (BigInt,BigInt,BigInt) {
        let A :BigInt = ((X2 - X1 ) * (X2 - X1)) % p;
        let B :BigInt = (X1 * (&A)) % p;
        let C :BigInt = X2 * (&A);
        let D :BigInt = ((Y2 - Y1) * (Y2 - Y1)) % p;
        if zero::<BigInt>().eq(&A) || zero::<BigInt>().eq(&D) {
            return self._double(X1,Y1,Z1,p,&(self.curve.a()));
        } 
        let X3 :BigInt = ((&D) - (&B) - (&C)) % p;
        let Y3 :BigInt = ((Y2 - Y1) * ((&B) - (&X3)) - Y1 * ((&C) - (&B))) % p;
        let Z3 :BigInt = (Z1 * (X2 - X1)) % p;
        return (X3, Y3,Z3);
    }

    fn _add_with_z2_1(&self,X1 :&BigInt, Y1 :&BigInt,Z1 :&BigInt,X2 :&BigInt,Y2 :&BigInt , p :&BigInt) -> (BigInt,BigInt,BigInt) {
        let Z1Z1 :BigInt = (Z1 * Z1) % p;
        let (U2,S2) :(BigInt,BigInt) = ((X2 * &(Z1Z1))% p , (Y2 * Z1 * (&Z1Z1)) % p);
        let H :BigInt = ((&U2) - X1) % p;
        let HH :BigInt = ((&H) * (&H)) % p;
        let I :BigInt = ((&HH) * 4) % p;
        let J :BigInt = (&H) * (&I);
        let r :BigInt = ((&S2) - Y1) % p;
        if zero::<BigInt>().eq(&r) || zero::<BigInt>().eq(&H) {
            return self._double_with_z_1(X2,Y2,p,&(self.curve.a()));
        }

        let V :BigInt = X1 * (&I);
        let X3 :BigInt = ((&r) * (&r) - &J - &V * 2) % p;
        let Y3 :BigInt = ((&r) * (&V - &X3) - Y1 * (&J) * 2) % p;
        let Z3 :BigInt = ((Z1 + (&H)) * (Z1 + (&H)) - &Z1Z1 - &HH) % p;
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
        let J :BigInt = ((&H) * (&I)) % p;
        let r :BigInt = (((&S2) - (&S1)) * 2) % p;
        if zero::<BigInt>().eq(&r) || zero::<BigInt>().eq(&H) {
            return self._double(X1,Y1,Z1,p,&(self.curve.a()));   
        }
        let V :BigInt = (&U1) * (&I);
        let X3 :BigInt = ((&r) * (&r) - (&J) - (&V) * 2) % p;
        let Y3 :BigInt = (((&r) * (&V - &X3)) - (&S1) * (&J) * 2) % p;
        let Z3 :BigInt = ((Z1 + Z2) * (Z1 + Z2) - &Z1Z1 - &Z2Z2) % p;
        return (X3,Y3,Z3);
    }

    fn _add(&self,X1 :&BigInt, Y1 :&BigInt,Z1 :&BigInt,X2 :&BigInt,Y2 :&BigInt ,Z2 :&BigInt , p :&BigInt) -> (BigInt,BigInt,BigInt) {
        let zv :BigInt = zero::<BigInt>();
        let ov :BigInt = one::<BigInt>();
        if zv.eq(Y1) || zv.eq(Z1) {
            return (X2.clone(),Y2.clone(),Z2.clone());
        }
        if zv.eq(Y2) || zv.eq(Z2) {
            return (X1.clone(),Y1.clone(),Z1.clone());
        }

        if Z1 == Z2 {
            if ov.eq(Z1) {
                return self._add_with_z_1(X1,Y1,X2,Y2,p);
            }
            return self._add_with_z_eq(X1,Y1,Z1,X2,Y2,p);
        }
        if ov.eq(Z1) {
            return self._add_with_z2_1(X2, Y2, Z2, X1, Y1, p);
        }

        if ov.eq(Z2) {
            return self._add_with_z2_1(X1, Y1, Z1, X2, Y2, p);
        }

        return self._add_with_z_ne(X1, Y1, Z1, X2, Y2, Z2, p);
    }

    pub fn add_jacobi(&self,other :&PointJacobi) -> PointJacobi {
        let ov :BigInt = zero::<BigInt>();
        if self.infinity {
            return other.clone();
        }
        if other.infinity {
            return self.clone();
        }

        assert!(self.curve.eq(&(other.curve)));
        let p :BigInt = self.curve.p();
        let (X1,Y1,Z1) = self.coords.clone();
        let (X2,Y2,Z2) = other.coords.clone();

        let (X3,Y3,Z3) = self._add(&X1, &Y1, &Z1, &X2, &Y2, &Z2, &p);
        if ov.eq(&Y3) || ov.eq(&Z3) {
            return PointJacobi::infinity();
        }
        let mut oo :Option<BigInt> = None;
        if self.order.is_some() {
            oo = Some(self.order.as_ref().unwrap().clone());
        }
        return PointJacobi::new(&(self.curve),&X3,&Y3,&Z3,oo,false);
    }

    pub fn add_point(&self,opoint :&Point) -> PointJacobi {
        let ov :BigInt = zero::<BigInt>();
        if self.infinity {
            return PointJacobi::from_affine(&opoint,false);
        }
        if opoint.isinfinity() {
            return self.clone();
        }

        let other :PointJacobi = PointJacobi::from_affine(&opoint,false);

        assert!(self.curve.eq(&(other.curve)));
        let p :BigInt = self.curve.p();
        let (X1,Y1,Z1) = self.coords.clone();
        let (X2,Y2,Z2) = other.coords.clone();

        let (X3,Y3,Z3) = self._add(&X1, &Y1, &Z1, &X2, &Y2, &Z2, &p);
        if ov.eq(&Y3) || ov.eq(&Z3) {
            return PointJacobi::infinity();
        }
        let mut oo :Option<BigInt> = None;
        if self.order.is_some() {
            oo = Some(self.order.as_ref().unwrap().clone());
        }
        return PointJacobi::new(&(self.curve),&X3,&Y3,&Z3,oo,false);
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


impl std::cmp::PartialEq<Point> for PointJacobi {
    fn eq(&self,other :&Point) -> bool {
        return self.eq_point(other);
    }

    fn ne(&self, other :&Point) -> bool {
        return  !self.eq(other);
    }
}
