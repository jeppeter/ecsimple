// https://github.com/rust-num/num-bigint
use num_bigint::BigInt;
use num_traits::ToPrimitive;
use num_traits::{one, zero};

pub mod curves;
pub mod ecdsa;
pub mod ecdsa_affine;
pub mod ecdsa_windowed;
pub mod ecdsa_wnaf;

#[derive(Debug)]
pub struct Curve {
    pub g: AffinePoint,
    pub p: BigInt,
    pub n: BigInt,
}

#[derive(Debug, Clone)]
pub struct AffinePoint {
    pub x: BigInt,
    pub y: BigInt,
    pub fp: BigInt,
}

#[derive(Debug, Clone)]
pub struct JacobianPoint {
    pub x: BigInt,
    pub y: BigInt,
    pub z: BigInt,
    pub fp: BigInt,
}

#[derive(Debug)]
pub struct Signature {
    pub r: BigInt,
    pub s: BigInt,
}

// TODO:  split into modules !!!!

// operator overloading....
// https://doc.rust-lang.org/std/ops/

impl AffinePoint {
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

    fn identity(&self) -> AffinePoint {
        AffinePoint {
            x: zero(),
            y: zero(),
            fp: self.fp.clone(),
        }
    }

    // Q ← 0
    // for i from m to 0 do
    //     Q ← point_double_repeat(Q, w)
    //     if di > 0 then
    //         Q ← point_add(Q, diP) # using pre-computed value of diP
    // return Q
    pub fn multiply_with_windowed_method(self, mut n: BigInt, p: &Vec<AffinePoint>) -> AffinePoint {
        let w = 4; // w.unwrap_or(4);
        let mut bitlength = 0;
        let mut temp = n.clone();
        let mut q = self.identity();

        while temp != zero() {
            bitlength = bitlength + 1;
            temp = temp >> 1;
        }

        let m = bitlength / w;

        // create a number with w 1s in binary form
        // https://stackoverflow.com/a/2798211/1786712
        let mask = BigInt::from((1 << w) - 1);

        let mut d = Vec::new();

        // get windowed representation of scalar
        while n > zero() {
            // get rightmost bits of scalar
            let chunk = &n & &mask;

            n = n >> w;

            // d is the index of the precompute added
            d.push(chunk.to_u16().unwrap());
        }

        let mut i: i32 = m;

        // core multiplication algorithm
        while i >= 0 {
            for _c in 0..w {
                q = q.double();
            }

            // TODO: code-smell: should this be necessary?
            if i as usize >= d.len() {
                i = i - 1;

                continue;
            }

            let index = &d[i as usize];

            if index > &0 {
                let point = p.get((index - 1) as usize).unwrap();
                q = q.add(point);
            }

            i = i - 1;
        }

        q
    }
}

pub fn get_window_precomputes(q: &AffinePoint, w: Option<u32>) -> Vec<AffinePoint> {
    let w = w.unwrap_or(4);
    let mut precomputes = Vec::new();
    let mut index = 0;
    let base: i32 = 2;

    let r = q.clone();
    let mut q = q.clone();

    while index < base.pow(w) {
        precomputes.push(q.clone());

        q = q.add(&r);

        index = index + 1;
    }

    precomputes
}

/// Performs a modulo operation of a under b.
///
/// # Example
///
/// In Rust, the % operator performs the remainder operation, not the modulus operation.
/// These operations have different results for negative numbers, e.g.:
///
/// -21 modulus 4 => 3
/// -21 remainder 4 => -1
///
/// println!("{}", -21 % 4); // -1
///
/// This helper performs a modulus operation, as opposed to a simple remainder operation.
///
/// ```
/// use num_bigint::BigInt;
/// let a = BigInt::from(218753);
/// let b = BigInt::from(96461);
/// let expected = BigInt::from(25831);
/// let actual = signer::modulo(&a, &b);
/// assert_eq!(actual, expected);
/// ```
pub fn modulo(a: &BigInt, b: &BigInt) -> BigInt {
    let result = a % b;

    if result >= zero() {
        result
    } else {
        b + result
    }
}

// https://extendedeuclideanalgorithm.com/multiplicative_inverse.php
pub fn modular_multiplicative_inverse(
    n: &BigInt,
    mut b: BigInt,
    t1: Option<BigInt>,
    t2: Option<BigInt>,
) -> BigInt {
    let t1 = t1.unwrap_or(zero());
    let t2 = t2.unwrap_or(one());

    if n == &zero() || b == zero() {
        return zero();
    }

    if b < zero() {
        b = modulo(&b, n);
    }

    let q = n / &b;
    let r = modulo(n, &b);

    let t3 = t1 - &q * &t2;

    if r == zero() && b != one() {
        return zero();
    }

    if r == zero() {
        t2
    } else {
        modular_multiplicative_inverse(&b, r, Some(t2), Some(t3))
    }
}

// For Bitcoin compressed public keys a single 0✕02 or 0✕03 byte is prepended on the ✕ values.
// Which of these two single bytes is used depends on the Y value.
// Prepend 0✕02 if the Y value is even and 0✕03 if the Y value is odd.
pub fn compress_point(point: &AffinePoint) -> String {
    let mut prefix: String;

    if &point.y % 2 != zero() {
        prefix = String::from("03");
    } else {
        prefix = String::from("02");
    }

    let hex_point: String = format!("{:x}", point.x);

    if hex_point.len() < 64 {
        prefix.push_str("0");
    }

    prefix.push_str(&hex_point);

    prefix
}

// Q is the point we are multiplying
// w is the window width
pub fn precompute_points(mut q: JacobianPoint, w: u32) -> std::vec::Vec<JacobianPoint> {
    let mut p = vec![q.clone()];

    q = q.double();

    let base: u32 = 2;

    for j in 1..base.pow(w - 1) {
        let mut buffer = q.clone();
        buffer = buffer.add(&p[(j - 1) as usize]);
        p.push(buffer);
    }

    p
}

// type of vector is wrong, each item in vector is eith a 0 or 1
pub fn convert_to_binary(mut n: BigInt) -> std::vec::Vec<u8> {
    let mut a: Vec<u8> = [].to_vec();

    while n >= one() {
        let r = modulo(&n, &BigInt::from(2));
        a.push(r.to_u8().unwrap());
        n = n / 2;
    }

    for i in 0..(a.len() / 2) {
        let j = a[i];
        a[i] = a[a.len() - i - 1];

        let len = a.len();
        a[len - i - 1] = j;
    }

    a
}

/// Calculates wNAF represenation of scalar.
///
/// https://www.researchgate.net/publication/280599362_A_Mathematical_Analysis_of_Elliptic_Curve_Point_Multiplication
///
/// Efficient Arithmetic on Koblitz Curves, JEROME A. SOLINAS - page 201 / 131 (page 7 on pdf)
/// A Mathematical Analysis of Elliptic Curve Point Multiplication, Ravi Kishore Kodali - page 4
/// Contemporary Cryptology - Dario Catalano, Ronald Cramer, Ivan Damgard, Giovanni Di Crescenzo, David Pointcheval, Tsuyoshi Takagi
/// Exponentiation, Chapter 9, Christophe Doche - page 154
pub fn calculate_wnaf(w: u32, mut n: BigInt) -> std::vec::Vec<i8> {
    let bits = convert_to_binary(n.clone());

    let _array_size = bits.len() + 1;

    let mut wnaf: Vec<i8> = Vec::new();

    let base: i8 = 2;

    let modulus = BigInt::from(base.pow(w)); // or 1 << w

    let mut i = 0;

    while n >= one() {
        if modulo(&n, &BigInt::from(2)) != zero() {
            // if n is odd
            let remainder = modulo(&n, &modulus);

            // is this conditional branching necessary?
            if remainder > BigInt::from(base.pow(w - 1) - 1) {
                wnaf.push((remainder - &modulus).to_i8().unwrap());
            } else {
                wnaf.push(remainder.to_i8().unwrap());
            }

            n = n - wnaf[i]
        } else {
            wnaf.push(0);
        }

        n = n / BigInt::from(2); // or n >> 1

        i = i + 1;
    }

    wnaf
}

// ====================================================================
// JACOBIAN POINTS
// ====================================================================

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

    pub fn new(x: BigInt, y: BigInt, z: BigInt, fp: BigInt) -> JacobianPoint {
        JacobianPoint {
            x: x,
            y: y,
            z: z,
            fp: fp,
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
            return other.clone();
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

        return JacobianPoint::new(x3, y3, z3, self.fp.clone());
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

        return JacobianPoint::new(x3, y3, z3, self.fp.clone());
    }

    pub fn multiply(self, mut n: BigInt) -> AffinePoint {
        let mut q = JacobianPoint::new(zero(), zero(), one(), self.fp.clone());

        let mut j = JacobianPoint::new(self.x, self.y, one(), self.fp.clone());

        while n > zero() {
            if &n % BigInt::from(2) != zero() {
                q = q.add(&j);
            }

            j = j.double();

            n = &n >> 1;
        }

        q.to_affine()
    }

    pub fn multiply_with_non_adjacent_form(
        self,
        n: BigInt,
        width: u32,
        pre_comp: &std::vec::Vec<JacobianPoint>,
    ) -> JacobianPoint {
        let wnaf = calculate_wnaf(width, n);

        let mut q = JacobianPoint::new(zero(), zero(), one(), self.fp.clone());

        let mut i = (wnaf.len() as i32) - 1;

        while i > -1 {
            q = q.double();

            let n = i as usize;

            if wnaf[n] > 0 {
                let d = (wnaf[n] - 1) / 2;

                q = q.add(&pre_comp[d as usize]);
            } else if wnaf[n] < 0 {
                let d = (-wnaf[n] - 1) / 2;

                let z = JacobianPoint {
                    x: pre_comp[d as usize].x.clone(),
                    y: pre_comp[d as usize].y.clone() * -1,
                    z: pre_comp[d as usize].z.clone(),
                    fp: self.fp.clone(),
                };

                q = q.add(&z);
            }

            i = i - 1;
        }

        q
    }
}
