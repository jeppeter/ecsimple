
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

pub fn leftmost_bit(x :&BigInt) -> BigInt {
    let mut result :BigInt = one::<BigInt>();
    while result <= x.clone() {
        result *= 2;
    }
    return result / 2;
}