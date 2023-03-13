
use num_bigint::{BigInt};
use crate::arithmetics::{modulo,modular_multiplicative_inverse};
use num_traits::{zero};


#[derive(Debug, Clone)]
pub struct AffinePoint {
    pub x: BigInt,
    pub y: BigInt,
    pub fp: BigInt,
}


#[allow(dead_code)]
impl AffinePoint {

    fn double(&self) -> AffinePoint {
        let lambda = modulo(
            &(3 * BigInt::pow(&self.x, 2)
                * modular_multiplicative_inverse(&self.fp, 2 * &self.y, None, None)),
            &self.fp,
        );

        // this is the same as the two lines in the add function, except
        // that we're not referencing `&other.x`, using `&self.x` instead.
        let rx = modulo(&(BigInt::pow(&lambda, 2) - &self.x - &self.x), &self.fp);
        let ry = modulo(&(lambda * (&self.x - &rx) - &self.y), &self.fp);

        AffinePoint {
            x: rx,
            y: ry,
            fp: self.fp.clone(),
        }
    }

    fn add(self, other: &AffinePoint) -> AffinePoint {
        if self.x == other.x && self.y == (&other.y * -1) {
            // P = -Q, vertical line, i.e. P + Q = P + (-P) = 0
            AffinePoint::identity(&self)
        } else if self.x == other.x && self.y == other.y {
            // P = Q, tangent to the curve, use point doubling on P
            self.double()
        } else if self.x == zero() && self.y == zero() {
            // P is identity element, Q + 0 = 0 + Q = Q
            other.clone()
        } else if other.x == zero() && other.y == zero() {
            // Q is identity element, P + 0 = 0 + P = P
            self
        } else {
            let lambda = modulo(
                &((&other.y - &self.y)
                    * modular_multiplicative_inverse(&self.fp, &other.x - &self.x, None, None)),
                &self.fp,
            );

            let rx = modulo(&(BigInt::pow(&lambda, 2) - &self.x - &other.x), &self.fp);
            let ry = modulo(&(lambda * (self.x - &rx) - self.y), &self.fp);

            AffinePoint {
                x: rx,
                y: ry,
                fp: self.fp,
            }
        }
    }	

    fn identity(&self) -> AffinePoint {
        AffinePoint {
            x: zero(),
            y: zero(),
            fp: self.fp.clone(),
        }
    }	

    pub fn multiply(mut self, mut n: BigInt) -> AffinePoint {
        let mut q = AffinePoint::identity(&self);

        while n > zero() {
            if &n % 2 != zero() {
                q = q.add(&self);
            }

            self = self.double();

            n = &n >> 1;
        }

        q
    }

}