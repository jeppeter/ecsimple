extern crate num_bigint_dig as num_bigint2;

use crate::*;
use num_bigint::{BigInt};
use num_traits::{zero,one};

use std::error::Error;

ecsimple_error_class!{EccArithmeticError}



pub fn mod_with_sign(a :&BigInt, m :&BigInt) -> BigInt {
    let mut retv :BigInt;
    let ov :BigInt = zero::<BigInt>();
    retv = a % m;
    while retv < ov {
        retv += m;
    }
    return retv;
}


pub fn inverse_mod(a :&BigInt,m :&BigInt) -> BigInt {
    if  zero::<BigInt>().eq(a) {
        return zero::<BigInt>();
    }
    let mut lm :BigInt;
    let mut hm :BigInt;
    let mut low :BigInt;
    let mut high :BigInt;
    lm = one();
    hm = zero();
    low  = a %m ;
    high = m.clone();
    while low > one() {
        let r :BigInt = &high / &low;
        let tmplm :BigInt = lm.clone();
        let tmplow :BigInt = low.clone();
        lm = &hm - &lm * &r;
        low = &high - &low * &r;
        hm = tmplm.clone();
        high = tmplow.clone();
    }
    return mod_with_sign(&lm,&m);
}


pub fn leftmost_bit(x :&BigInt) -> BigInt {
    let mut result :BigInt = one::<BigInt>();
    let tv :i32 = 2;
    while result <= x.clone() {
        result *= tv;
    }
    return result / tv;
}

pub (crate) fn bit_length(num :&BigInt) -> usize {
    let mut retv : usize = 0;
    let (_ , vecs) = num.to_bytes_be();
    if vecs.len() > 0 {
        let mut uv = vecs[0];
        retv = (vecs.len() - 1) * 8;
        while uv > 0 {
            retv += 1;
            uv >>= 1;
        }
    }
    return retv;
}

fn jacobi_get(a :&BigInt,p :&BigInt) -> BigInt {
    return one();
}

fn polynomial_exp_mod(cf :&[BigInt],pb :&BigInt,f :&[BigInt],p :&BigInt) -> Vec<BigInt> {
    return Vec::new();
}

pub (crate) fn square_root_mod_prime(ac :&BigInt,p :&BigInt) -> Result<BigInt,Box<dyn Error>> {
    let a :BigInt = mod_with_sign(ac,p);
    let zv :BigInt = zero();
    let ov :BigInt = one();
    let tv :BigInt = &ov + &ov;
    let nov :BigInt = - ov.clone();
    let threev :BigInt = &tv + &ov;
    let fourv :BigInt = &tv + &tv;
    let eightv :BigInt = &fourv + &fourv;
    let fivev :BigInt = &fourv + &ov;
    if a == zv {
        return Ok(zv);
    }
    if p.eq(&tv) {
        return Ok(a.clone());
    }
    let jac :BigInt = jacobi_get(&a,p);

    if jac == nov {
        ecsimple_new_error!{EccArithmeticError,"a [{}:0x{:x}] p [{}:0x{:x}] not square modulo", ac,ac,p,p}
    }

    if (p % &fourv) == threev {
        let p4 :BigInt = (p + &ov ) / &fourv;
        return Ok(a.modpow(&p4,p));
    }

    if (p % &eightv) == fivev {
        let p14 :BigInt = (p - &ov) / &fourv;
        let d :BigInt = a.modpow(&p14,p);
        if d == ov {
            let p34 :BigInt = (p + &threev) / &eightv;
            return Ok(a.modpow(&p34,p));
        }
        assert!(d == (p - &ov));
        let p58 :BigInt = (p - &fivev) / &eightv;
        let a4 :BigInt = a.clone() * 4;
        return Ok( (&a * a4.modpow(&p58,p) * 2 ) % p);
    }

    let mut b :BigInt ;
    let basep :BigInt = p.clone();
    b = tv.clone();
    while b < basep {
        let bb4a :BigInt = &b * &b - &a * 4;
        let cd = jacobi_get(&bb4a,p);
        if cd == nov {
            let f :Vec<BigInt> = vec![a.clone(),- b.clone(),ov.clone()];
            let cf :Vec<BigInt> = vec![zv.clone(),ov.clone()];
            let p12 :BigInt = (p + &ov) / &tv;
            let ff :Vec<BigInt> = polynomial_exp_mod(&cf,&p12,&f,p);
            if ff.len() < 2 {
                ecsimple_new_error!{EccArithmeticError,"ff len [{}] < 2", ff.len()}
            }
            if ff[1] != zv {
                ecsimple_new_error!{EccArithmeticError,"ff[1] [0x{:x}]", ff[1]}
            }
            return Ok(ff[0].clone());
        }
        b += &ov;
    }

    ecsimple_new_error!{EccArithmeticError,"no square root for [{}:0x{:x}] [{}:0x{:x}]", ac,ac,p,p}
}
