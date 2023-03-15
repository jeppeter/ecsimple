
use num_bigint::{BigInt};
use num_traits::{zero,one};
use crate::arithmetics::*;

#[derive(Clone,Debug)]
pub struct CurveFp {
    p :BigInt,
    a :BigInt,
    b :BigInt,
    h :BigInt,
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
pub struct PointJacobi {
    curve :CurveFp,
    coords : (BigInt,BigInt,BigInt),
    order :Option<BigInt>,
    generator :bool,
    precompute : Vec<(BigInt,BigInt)>,
}

impl PointJacobi {
    pub fn new(curve :&CurveFp, x :&BigInt, y :&BigInt, z :&BigInt,order :Option<BigInt>,generator :bool) -> Self {
        if order.is_none() {
            PointJacobi {
                curve :curve.clone(),
                order : None,
                coords : (x.clone(),y.clone(),z.clone()),
                generator : generator,
                precompute : Vec::new(),
            }
        } else {
            PointJacobi {
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
        }
        return;
    }
}
