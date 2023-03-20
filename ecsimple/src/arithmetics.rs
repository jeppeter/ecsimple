extern crate num_bigint_dig as num_bigint2;

use num_bigint::{BigInt};
use num_traits::{zero,one};

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