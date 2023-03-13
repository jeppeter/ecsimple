
use num_bigint::{BigInt};
use crate::arithmetics::{modulo,modular_multiplicative_inverse};
use crate::affine::{AffinePoint};
use num_traits::{zero,one};


#[derive(Debug, Clone)]
pub struct JacobianPoint {
    pub x: BigInt,
    pub y: BigInt,
    pub z: BigInt,
    pub fp: BigInt,
}

#[allow(dead_code)]
impl JacobianPoint {
    pub fn to_affine(&self) -> AffinePoint {
        let p = self.clone();
        let z_inverse = modular_multiplicative_inverse(&self.fp, p.z, None, None);

        let x = modulo(&(p.x * &BigInt::pow(&z_inverse, 2)), &self.fp);
        let y = modulo(&(p.y * &BigInt::pow(&z_inverse, 3)), &self.fp);

        AffinePoint {
            x: x,
            y: y,
            fp: p.fp,
        }
    }

    // https://hyperelliptic.org/EFD/g1p/auto-shortw-jacobian-0.html#addition-add-2007-bl
    fn add(self, other: &JacobianPoint) -> JacobianPoint {
        let x1 = &self.x;
        let y1 = &self.y;
        let z1 = &self.z;

        let x2 = &other.x;
        let y2 = &other.y;
        let z2 = &other.z;

        if x2 == &zero() && y2 == &zero() {
            return self;
        }

        if x1 == &zero() && y1 == &zero() {
            return JacobianPoint {
                x: other.x.clone(),
                y: other.y.clone(),
                z: other.z.clone(),
                fp: self.fp,
            };
        }

        // what about case when P === Q ?

        let z1z1 = BigInt::pow(&z1, 2);
        let z2z2 = BigInt::pow(&z2, 2);
        let u1 = x1 * &z2z2;
        let u2 = x2 * &z1z1;
        let s1 = y1 * z2.clone() * &z2z2;
        let s2 = y2 * z1.clone() * &z1z1;
        let h = u2 - &u1;
        let i = BigInt::pow(&(2 * &h), 2);
        let j = &h * &i;
        let r = 2 * (s2 - &s1);
        let v = u1 * i;

        let x3 = modulo(&(BigInt::pow(&r, 2) - &j - BigInt::from(2) * &v), &self.fp);
        let y3 = modulo(&(r * (v - &x3) - BigInt::from(2) * s1 * j), &self.fp);
        let z3 = modulo(&((BigInt::pow(&(z1 + z2), 2) - z1z1 - z2z2) * h), &self.fp);

        JacobianPoint {
            x: x3,
            y: y3,
            z: z3,
            fp: self.fp,
        }
    }

    // https://en.wikibooks.org/wiki/Cryptography/Prime_Curve/Jacobian_Coordinates
    // https://stackoverflow.com/questions/8389324/how-to-calculate-point-addition-using-jacobian-coordinate-system-over-elliptic-e
    // https://hyperelliptic.org/EFD/g1p/auto-shortw-jacobian-0.html#doubling-dbl-2009-l
    fn double(self) -> JacobianPoint {
        let x1 = self.x;
        let y1 = self.y;
        let z1 = self.z;

        let a = BigInt::pow(&x1, 2);
        let b = BigInt::pow(&y1, 2);
        let c = BigInt::pow(&b, 2);
        let d = BigInt::from(2) * (BigInt::pow(&(x1 + &b), 2) - &a - &c);
        let e = BigInt::from(3) * &a;
        let f = BigInt::pow(&e, 2);

        let x3 = modulo(&(f - BigInt::from(2) * &d), &self.fp);
        let y3 = modulo(&(e * (d - &x3) - BigInt::from(8) * c), &self.fp);
        let z3 = modulo(&(BigInt::from(2) * y1 * z1), &self.fp);

        JacobianPoint {
            x: x3,
            y: y3,
            z: z3,
            fp: self.fp,
        }
    }

    pub fn multiply(self, mut n: BigInt) -> AffinePoint {
        let mut q = JacobianPoint {
            x: zero(),
            y: zero(),
            z: one(),
            fp: self.fp.clone(),
        };

        let mut j = JacobianPoint {
            x: self.x,
            y: self.y,
            z: one(),
            fp: self.fp.clone(),
        };

        while n > zero() {
            if &n % BigInt::from(2) != zero() {
                q = q.add(&j);
            }

            j = j.double();

            n = &n >> 1;
        }

        q.to_affine()
    }
}
