extern crate num_bigint_dig as num_bigint2;

use crate::*;
use num_bigint::{BigInt,Sign};
use num_traits::{zero,one};
use crate::logger::*;
use lazy_static::lazy_static;

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

/*
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

#[allow(non_snake_case)]
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
        ecsimple_new_error!{EccArithmeticError,"n [{}] < 3",n}
    }

    if (n % &tv) != ov {
        ecsimple_new_error!{EccArithmeticError,"n [{}] even",n}
    }
    let a :BigInt = mod_with_sign(&_a,n);
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
    let nd :BigInt = mod_with_sign(n,&a1);
    let curv :BigInt = jacobi_get(&nd,&a1)?;
    return Ok(s * &curv);
}

fn square_root_mod_prime2(ac :&BigInt,p :&BigInt) -> Result<BigInt,Box<dyn Error>> {
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
    let jac :BigInt = jacobi_get(&a,p)?;

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
        let cd = jacobi_get(&bb4a,p)?;
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
*/

fn bn_is_odd(a :&BigInt) -> bool {
    let zv :BigInt = zero();
    let ov :BigInt = one();
    let tv :BigInt = ov.clone() + ov.clone();

    if (a % &tv) == zv {
        return false;
    }
    return true;
}


fn get_bit_set(bn :&BigInt,i :i32) -> i32 {
    let mut retv :i32 = 0;
    let ov :BigInt = one();
    let zv :BigInt = zero();
    if (bn & (ov << i)) != zv {
        retv = 1;
    }
    return retv;
}

fn nmod(a :&BigInt,m :&BigInt) -> BigInt {
    let mut retv :BigInt;
    let zv :BigInt = zero();
    retv = a % m;
    if retv < zv {
        retv += m;
    }
    return retv;
}

fn verify_mod_sqrt(retv :&BigInt,ac :&BigInt,p :&BigInt) -> Result<BigInt,Box<dyn Error>> {
    let cv :BigInt;
    cv = retv.clone() * retv.clone() % p.clone();
    if cv != ac.clone() {
        ecsimple_new_error!{EccArithmeticError,"check 0x{:X} for mod_sqrt(0x{:X},0x{:X}) error",retv,ac,p}
    }
    return Ok(retv.clone());
}

lazy_static ! {
    static ref KRONTAB :Vec<i32> = {
        let  retv :Vec<i32> = vec![0,1,0,-1,0,-1,0,1];
        retv
    };
}

fn get_lsw(v :&BigInt, mask :u32) -> u32 {
    let vecs :Vec<u8>;
    let mut retv :u32=0;
    (_,vecs) = v.to_bytes_le();
    let mut idx :usize = 0;
    while idx < vecs.len() && idx < 4 {
        retv |= (vecs[idx] as u32) << (idx * 8);
        idx += 1;
    }
    return retv & mask;
}

#[allow(non_snake_case)]
fn kronecker_value(a :&BigInt,b :&BigInt) -> i32 {
    let mut ret :i32;
    let mut A :BigInt;
    let mut B :BigInt;
    let zv :BigInt = zero();
    let ov :BigInt = one();
    let mut i :i32;
    let mut tmp :BigInt;

    A = a.clone();
    B = b.clone();
    ecsimple_log_trace!("A 0x{:X} B 0x{:X}",A,B);

    /*
     * Kronecker symbol, implemented according to Henri Cohen,
     * "A Course in Computational Algebraic Number Theory"
     * (algorithm 1.4.10).
     */
     if B == zv {
        if A == ov {
            ret = 1;
        } else {
            ret = 0;
        }
        ecsimple_log_trace!("ret {}",ret);
        return ret;
    }

    if !bn_is_odd(&A) && !bn_is_odd(&B) {
        ret = 0;
        ecsimple_log_trace!("ret 0");
        return ret;
    }

    i = 0;
    while get_bit_set(&B,i) == 0 {
        i += 1;
    }
    ecsimple_log_trace!("B 0x{:X} i {}",B,i);
    B = B.clone() >> i;
    ecsimple_log_trace!("B 0x{:X}",B);

    if (i & 1) != 0 {
        ret = KRONTAB[get_lsw(&A,7) as usize];
        ecsimple_log_trace!("ret {} tab[BN_lsw(0x{:X})&7 = 0x{:x}]", ret, A,get_lsw(&A,7));
    } else {
        ret = 1;
        ecsimple_log_trace!("ret 1");
    }

    if B < zv {
        B = -B.clone();
        if A < zv {
            ret = -ret;
            ecsimple_log_trace!("A net ret {}",ret);
        }
    }

    loop {
        if A == zv {
            if B != ov {
                ret = 0;
            }
            ecsimple_log_trace!("B 0x{:X} ret {}",B,ret);
            return ret;
        }

        i = 0;
        while get_bit_set(&A,i) == 0{
            i += 1;
        }
        ecsimple_log_trace!("A 0x{:X} i {}",A,i);
        A = A.clone() >> i;
        ecsimple_log_trace!("A 0x{:X}",A);

        if (i & 1) != 0 {
            ecsimple_log_trace!("ret {} = ret {} * tab[BN_lsw(0x{:X})&7 = 0x{:x}]",ret * KRONTAB[get_lsw(&B,7) as usize], ret,B,get_lsw(&B,7));
            ret = ret * KRONTAB[get_lsw(&B,7) as usize];
        }

        let mut rv :u32;

        if A < zv {
            rv = get_lsw(&A,0xffffffff);
            rv = !rv;
        } else {
            rv = get_lsw(&A,0xffffffff) & get_lsw(&B,0xffffffff) & 0x2;
        }

        if rv != 0 {
            let mut negv :String=format!("0");
            if A < zv {
                negv = format!("1");
            }
            ecsimple_log_trace!("A->neg {} A 0x{:X} B 0x{:X}",negv,A,B);
            ret = -ret;
        }

        B = nmod(&B,&A);
        ecsimple_log_trace!("nnmod(B 0x{:X},B,A 0x{:X})",B,A);

        tmp = A.clone();
        A = B.clone();
        B = tmp.clone();
        ecsimple_log_trace!("A 0x{:X} B 0x{:X}",A,B);
        if B < zv {
            B = -B;
        }
    }
}


#[allow(non_snake_case)]
pub fn square_root_mod_prime(ac :&BigInt,p :&BigInt) -> Result<BigInt,Box<dyn Error>> {
    let mut retv :BigInt = zero();
    let ov :BigInt = one();
    let zv :BigInt = zero();
    let tv :BigInt = ov.clone() + ov.clone();
    let A :BigInt;
    let mut b :BigInt;
    let mut q :BigInt;
    let mut t :BigInt;
    let mut x :BigInt = zv.clone();
    let mut y :BigInt = zv.clone();
    let mut e :i32;
    let mut vecs :Vec<u8>;
    let mut i :i32;
    let mut r :i32;
    if (p % tv.clone()) == zv || p == &ov {
        if p == &tv {
            if get_bit_set(ac,0) == 0 {
                retv = zv.clone();
            } else {
                retv = ov.clone();
            }
            return Ok(retv);
        }
        ecsimple_new_error!{EccArithmeticError,"p 0x{:X} not valid prime",p}
    }

    if ac == &zv || ac == &ov {
        retv = ac.clone();
        ecsimple_log_trace!("ret 0x{:X}",retv);
        return Ok(retv);
    }

    A = nmod(ac,p);
    ecsimple_log_trace!("BN_nnmod(A 0x{:X},a 0x{:X},p 0x{:X})",A,ac,p);
    e = 1;
    while get_bit_set(p,e) == 0 {
        e += 1;
    }
    ecsimple_log_trace!("e 0x{:x}",e);

    if e == 1 {
        q = p.clone() >> 2;
        ecsimple_log_trace!("BN_rshift(q 0x{:X},p 0x{:X},2)",q,p);
        /*to set plus*/
        (_,vecs) = q.to_bytes_le();
        q = BigInt::from_bytes_le(Sign::Plus,&vecs);
        q += ov.clone();
        ecsimple_log_trace!("BN_add_word(q 0x{:X},1)",q);
        retv = A.modpow(&q,p);
        ecsimple_log_trace!("BN_mod_exp(ret 0x{:X},A 0x{:X},q 0x{:X},p 0x{:X})",retv,A,q,p);
        return verify_mod_sqrt(&retv,&A,p);
    }
    if e == 2 {
        /*-
         * |p| == 5  (mod 8)
         *
         * In this case  2  is always a non-square since
         * Legendre(2,p) = (-1)^((p^2-1)/8)  for any odd prime.
         * So if  a  really is a square, then  2*a  is a non-square.
         * Thus for
         *      b := (2*a)^((|p|-5)/8),
         *      i := (2*a)*b^2
         * we have
         *     i^2 = (2*a)^((1 + (|p|-5)/4)*2)
         *         = (2*a)^((p-1)/2)
         *         = -1;
         * so if we set
         *      x := a*b*(i-1),
         * then
         *     x^2 = a^2 * b^2 * (i^2 - 2*i + 1)
         *         = a^2 * b^2 * (-2*i)
         *         = a*(-i)*(2*a*b^2)
         *         = a*(-i)*i
         *         = a.
         *
         * (This is due to A.O.L. Atkin,
         * Subject: Square Roots and Cognate Matters modulo p=8n+5.
         * URL: https://listserv.nodak.edu/cgi-bin/wa.exe?A2=ind9211&L=NMBRTHRY&P=4026
         * November 1992.)
         */


         /* t := 2*a */
         t = (A.clone() << 1) % p.clone();
         ecsimple_log_trace!("BN_mod_lshift1_quick(t 0x{:X},A 0x{:X},p 0x{:X})",t,A,p);
         /* b := (2*a)^((|p|-5)/8) */
         q = p.clone() >> 3;
         ecsimple_log_trace!("BN_rshift(q 0x{:X},p 0x{:X},0x3)",q,p);
         /*to set plus*/
         (_,vecs) = q.to_bytes_le();
         q = BigInt::from_bytes_le(Sign::Plus,&vecs);

         b = t.modpow(&q,p);
         ecsimple_log_trace!("BN_mod_exp(b 0x{:X},t 0x{:X},q 0x{:X},p 0x{:X})",b,t,q,p);

         /* y := b^2 */
         y = b.modpow(&tv,p);
         ecsimple_log_trace!("BN_mod_sqr(y 0x{:X},b 0x{:X},p 0x{:X})",y,b,p);

         /* t := (2*a)*b^2 - 1 */
         t = t.clone() * y.clone() % p.clone();
         ecsimple_log_trace!("BN_mod_mul(t 0x{:X},t,y 0x{:X},p 0x{:X})",t,y,p);
         t -= ov.clone();
         ecsimple_log_trace!("BN_sub_word(t 0x{:X},1)",t);

         /* x = a*b*t */
         x = A.clone() * b.clone() % p.clone();
         ecsimple_log_trace!("BN_mod_mul(x 0x{:X},A 0x{:X},b 0x{:X},p 0x{:X})",x,A,b,p);
         x = x.clone() * t.clone() % p.clone();
         ecsimple_log_trace!("BN_mod_mul(x 0x{:X},x,t 0x{:X},p 0x{:X})",x,t,p);
         retv = x.clone();
         ecsimple_log_trace!("BN_copy(ret 0x{:X},x 0x{:X})",retv,x);
         return verify_mod_sqrt(&retv,&A,p);
     }

     q = p.clone();
     ecsimple_log_trace!("BN_copy(q 0x{:X},p 0x{:X})",q,p);
     (_,vecs) = q.to_bytes_le();
     q = BigInt::from_bytes_le(Sign::Plus,&vecs);

     i = 2;
     loop {
        if i < 83 {
            vecs = Vec::new();
            vecs.push(i as u8);
            y = BigInt::from_bytes_le(Sign::Plus,&vecs);
        }

        ecsimple_log_trace!("before BN_kronecker(y 0x{:X},q 0x{:X})",y,q);
        r = kronecker_value(&y,&q);
        ecsimple_log_trace!("r ({})=BN_kronecker(y 0x{:X},q 0x{:X})",r,y,q);

        if r < -1 {
            ecsimple_new_error!{EccArithmeticError,"error kronecker_value {}",r}
        } else if r == 0 {
            ecsimple_new_error!{EccArithmeticError,"not prime for p 0x{:X}",p}
        }

        i += 1;
        if r != 1 || i >= 82{
            break;
        }
    }
    ecsimple_log_trace!("r = {}",r);

    if r != -1 {
        ecsimple_new_error!{EccArithmeticError,"too many iterators for mod_sqrt"}
    }

    q = q.clone() >> e;
    ecsimple_log_trace!("BN_rshift(q 0x{:X},q,e 0x{:x})",q,e);

    y = y.clone().modpow(&q,p);
    ecsimple_log_trace!("BN_mod_exp(y 0x{:X},y,q 0x{:X},p 0x{:X})",y,q,p);
    if y == ov {
        ecsimple_new_error!{EccArithmeticError,"p 0x{:X} is not prime",p}
    }

    /*-
     * Now we know that (if  p  is indeed prime) there is an integer
     * k,  0 <= k < 2^e,  such that
     *
     *      a^q * y^k == 1   (mod p).
     *
     * As  a^q  is a square and  y  is not,  k  must be even.
     * q+1  is even, too, so there is an element
     *
     *     X := a^((q+1)/2) * y^(k/2),
     *
     * and it satisfies
     *
     *     X^2 = a^q * a     * y^k
     *         = a,
     *
     * so it is the square root that we are looking for.
     */

    /* t := (q-1)/2  (note that  q  is odd) */
    t = q.clone() >> 1;
    ecsimple_log_trace!("BN_rshift1(t 0x{:X},q 0x{:X})",t,q);

    /* x := a^((q-1)/2) */
    if t == zv {
        t = nmod(&A,p);
        ecsimple_log_trace!("BN_nnmod(t 0x{:X},A 0x{:X},p 0x{:X})",t,A,p);
        if t == zv {
            retv = zv.clone();
            return Ok(retv);
        } else if x == ov {
            return Ok(retv);
        }
    } else {
        x = A.clone().modpow(&t,p);
        ecsimple_log_trace!("BN_mod_exp(x 0x{:X},A 0x{:X},t 0x{:X},p 0x{:X})",x,A,t,p);
        if x == zv {
            retv = zv.clone();
            return Ok(retv);
        }
    }

    /* b := a*x^2  (= a^q) */
    b = (x.clone() * x.clone()) % p.clone();
    ecsimple_log_trace!("BN_mod_sqr(b 0x{:X},x 0x{:X},p 0x{:X})",b,x,p);
    b = (b.clone() * A.clone()) % p.clone();
    ecsimple_log_trace!("BN_mod_mul(b 0x{:X},b,A 0x{:X},p 0x{:X})",b,A,p);
    x = (x.clone() * A.clone()) % p.clone();
    ecsimple_log_trace!("BN_mod_mul(x 0x{:X},x,A 0x{:X},p 0x{:X})",x,A,p);

    loop {
        /*-
         * Now  b  is  a^q * y^k  for some even  k  (0 <= k < 2^E
         * where  E  refers to the original value of  e,  which we
         * don't keep in a variable),  and  x  is  a^((q+1)/2) * y^(k/2).
         *
         * We have  a*b = x^2,
         *    y^2^(e-1) = -1,
         *    b^2^(e-1) = 1.
         */
        if b == ov {
            retv = x.clone();
            ecsimple_log_trace!("BN_copy(ret 0x{:X},x 0x{:X})",retv,x);
            return verify_mod_sqrt(&retv,&A,p);
        }

        i = 1;
        while i < e {
            if i == 1 {
                t = b.clone() * b.clone() % p.clone();
                ecsimple_log_trace!("BN_mod_sqr(t 0x{:X},b 0x{:X},p 0x{:X})",t,b,p);
            } else {
                t = t.clone() * t.clone() % p.clone();
                ecsimple_log_trace!("BN_mod_mul(t 0x{:X},t,t,p 0x{:X})",t,p);
            }
            if t == ov {
                break;
            }
            i +=1;
        }

        if i >= e {
            ecsimple_new_error!{EccArithmeticError,"no sqrt for a [0x{:X}] for p [0x{:X}]",ac,p}
        }

        /* t := y^2^(e - i - 1) */
        t = y.clone();
        let mut j :i32;
        j = e - i - 1;
        while j > 0 {
            t = t.clone() * t.clone() % p.clone();
            ecsimple_log_trace!("BN_mod_sqr(t 0x{:X},t,p 0x{:X})",t,p);
            j -= 1;
        }
        y = t.clone() * t.clone() % p.clone();
        ecsimple_log_trace!("BN_mod_mul(y 0x{:X},t 0x{:X},t,p 0x{:X})",y,t,p);
        x = x.clone() * t.clone() % p.clone();
        ecsimple_log_trace!("BN_mod_mul(x 0x{:X},x,t 0x{:X},p 0x{:X})",x,t,p);
        b = b.clone() * y.clone() % p.clone();
        ecsimple_log_trace!("BN_mod_mul(b 0x{:X},b,y 0x{:X},p 0x{:X})",b,y,p);
        e = i;
    }

}
