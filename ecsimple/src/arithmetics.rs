
use num_bigint::{BigInt};
use num_traits::{zero,one};

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
    return lm % m;
}

pub fn mod_with_sign(a :&BigInt, m :&BigInt) -> BigInt {
    let mut retv :BigInt;
    let ov :BigInt = zero::<BigInt>();
    retv = a % m;
    while retv < ov {
        retv += m;
    }
    return retv;

}

pub fn leftmost_bit(x :&BigInt) -> BigInt {
    let mut result :BigInt = one::<BigInt>();
    let tv :i32 = 2;
    while result <= x.clone() {
        result *= tv;
    }
    return result / tv;
}