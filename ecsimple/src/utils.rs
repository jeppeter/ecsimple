use num_bigint::{BigInt,Sign};
use num_traits::{zero,one};
use crate::*;
use std::error::Error;

ecsimple_error_class!{ECUtilsError}

pub (crate) fn get_max_bits(bn :&BigInt) -> i64 {
	let mut retv : i64 = -1;
	let mut idx : i64 = 0 ;
	let zv :BigInt = zero();
	let mut cv : BigInt = one();

	while bn >= &cv {
		if (&cv & bn) != zv {
			/*for expand*/
			retv = idx + 1;
		}
		idx += 1;
		cv <<= 1;
	}
	return retv;
}

pub (crate) fn format_bigint_as_order(bn :&BigInt, order :&BigInt) -> BigInt {
	let obits :i64 = get_max_bits(order);
	let mut bs :Vec<u8>;
	(_,bs) = bn.to_bytes_be();
	if (8 * bs.len()) > (obits as usize) {
		bs = bs[0..(((obits as usize) +7) >> 3)].to_vec();
	}

	let mut retv :BigInt = BigInt::from_bytes_be(Sign::Plus,&bs);
	if bs.len() * 8 > (obits as usize) {
		retv = retv >> (8 - obits & 0x7);
	}
	return retv;
}

pub (crate) fn get_bit_set(bn :&BigInt,i :i32) -> i32 {
	let mut retv :i32 = 0;
	let ov :BigInt = one();
	let zv :BigInt = zero();
	if (bn & (ov << i)) != zv {
		retv = 1;
	}
	return retv;
}

pub (crate) fn nmod(a :&BigInt,m :&BigInt) -> BigInt {
	let mut retv :BigInt;
	let zv :BigInt = zero();
	retv = a % m;
	if retv < zv {
		retv += m;
	}
	return retv;
}

fn polynomial_reduce_mod(_poly :&[BigInt],polymod :&[BigInt],p :&BigInt) -> Vec<BigInt> {
    let ov :BigInt = one();
    let zv :BigInt = zero();
    let mut poly :Vec<BigInt> = _poly.to_vec().clone();
    let mut lasti :usize;
    let mut lastj :usize;
    let mut lastk :usize;
    if polymod.len() > 0 {
        lasti = polymod.len() - 1;
        assert!(polymod[lasti] == (- ov));
    }
    assert!(polymod.len() > 1);

    while poly.len() >= polymod.len() {
        lasti = poly.len() - 1;
        if poly[lasti] != zv {
            for i in 2..(polymod.len() + 1) {
                lasti = poly.len() - i;
                lastj = polymod.len() - i;
                lastk = poly.len() - 1;
                poly[lasti] = (&poly[lasti] - &poly[lastk] * &polymod[lastj]) % p;
            }
        }
        lasti = poly.len() - 1;
        poly = poly[0..lasti].to_vec().clone();
    }
    return poly;


}

fn polynomial_multiply_mod(m1 :&[BigInt],m2 :&[BigInt],polymod :&[BigInt],p :&BigInt) -> Vec<BigInt> {
    let zv :BigInt = zero();
    let mut prod :Vec<BigInt> = Vec::new();
    let mut idx :usize = 0;
    while idx < (m1.len() + m2.len() - 1) {
        prod.push(zv.clone());
        idx += 1;
    }

    for i in 0..m1.len() {
        for j in 0..m2.len() {
            prod[i + j] = (&prod[i+j] + &(m1[i]) * &(m2[j])) % p;
        }
    }

    return polynomial_reduce_mod(&prod,polymod,p);
}



fn polynomial_exp_mod(base :&[BigInt],exponent :&BigInt,polymod :&[BigInt],p :&BigInt) -> Vec<BigInt> {
    let zv :BigInt = zero();
    let ov :BigInt = one();
    let tv :BigInt = &ov + &ov;
    let mut G :Vec<BigInt>;
    let mut k :BigInt;
    let mut s :Vec<BigInt>;
    assert!(exponent < p);
    if exponent.eq(&zv) {
        s = vec![ov.clone()];
        return s;
    }

    G = base.to_vec().clone();
    k = exponent.clone();

    if ((&k) % &tv) == ov {
        s = G.clone();
    } else {
        s = vec![ov.clone()];
    }

    while k > ov {
        k /= &tv;
        G = polynomial_multiply_mod(&G,&G,polymod,p);
        if ((&k) % &tv) == ov {
            s = polynomial_multiply_mod(&G,&s,polymod,p);
        }
    }
    return s;

}


fn jacobi_get(_a :&BigInt,n :&BigInt) -> Result<BigInt,Box<dyn Error>> {
    let ov :BigInt = one();
    let zv :BigInt = zero();
    let tv :BigInt = &ov + &ov;
    let threev :BigInt = &tv + &ov;
    let fourv :BigInt = &tv + &tv;
    let eightv :BigInt = &fourv + &fourv;
    let sevenv :BigInt = &eightv - &ov;
    if !(n >= &threev) {
        ecsimple_new_error!{ECUtilsError,"n [{}] < 3",n}
    }

    if (n % &tv) != ov {
        ecsimple_new_error!{ECUtilsError,"n [{}] even",n}
    }
    let a :BigInt = nmod(&_a,n);
    if a == zv {
        return Ok(zv.clone());
    }
    if a == ov {
        return Ok(ov.clone());
    }
    let mut a1 :BigInt = a.clone();
    let mut e :BigInt = zv.clone();
    while (&a1 % &tv) == zv {
        a1 = a1 / &tv;
        e += &ov;
    }

    let mut s :BigInt;

    if (e % &tv) == zv  || (n % &eightv) == ov || ( n % &eightv)  == sevenv {
        s = ov.clone();
    } else{
        s = - ov.clone();
    }

    if a1 == ov {
        return Ok(s.clone());
    }

    if (n % &fourv) == threev && ( (&a1) % &fourv) == threev {
        s = - s.clone();
    }
    let nd :BigInt = nmod(n,&a1);
    let curv :BigInt = jacobi_get(&nd,&a1)?;
    return Ok(s * &curv);
}


pub (crate) fn mod_sqrt(ac :&BigInt,p :&BigInt) -> Result<BigInt,Box<dyn Error>> {
    let a :BigInt = nmod(ac,p);
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
    let jac :BigInt = jacobi_get(&a,p)?;

    if jac == nov {
        ecsimple_new_error!{ECUtilsError,"a [{}:0x{:x}] p [{}:0x{:x}] not square modulo", ac,ac,p,p}
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
        let cd = jacobi_get(&bb4a,p)?;
        if cd == nov {
            let f :Vec<BigInt> = vec![a.clone(),- b.clone(),ov.clone()];
            let cf :Vec<BigInt> = vec![zv.clone(),ov.clone()];
            let p12 :BigInt = (p + &ov) / &tv;
            let ff :Vec<BigInt> = polynomial_exp_mod(&cf,&p12,&f,p);
            if ff.len() < 2 {
                ecsimple_new_error!{ECUtilsError,"ff len [{}] < 2", ff.len()}
            }
            if ff[1] != zv {
                ecsimple_new_error!{ECUtilsError,"ff[1] [0x{:x}]", ff[1]}
            }
            return Ok(ff[0].clone());
        }
        b += &ov;
    }

    ecsimple_new_error!{ECUtilsError,"no square root for [{}:0x{:x}] [{}:0x{:x}]", ac,ac,p,p}
}