
use num_bigint::{BigInt};
use num_traits::{zero,one};

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


