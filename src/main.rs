#![allow(clippy::many_single_char_names)]

use num::{BigInt, Integer, One};

fn main() {
    println!("Hello World!");

    let p = BigInt::from(17_u8);
    let q = BigInt::from(19_u8);

    let n = &p * &q;
    let phi = (p - BigInt::one()) * (q - BigInt::one());
    let e = BigInt::from(151);

    let d = modinverse(&e, &phi).unwrap();
    println!("Public key: ({}, {})", &n, &e);
    println!("Private key: ({}, {})", &n, &d);

    let c = BigInt::from('h' as u32);
    println!("{}", c.modpow(&e, &n));
}

pub fn egcd<T: Integer + Clone>(a: &T, b: &T) -> (T, T, T) {
    if a.is_zero() {
        (b.clone(), T::zero(), T::one())
    } else {
        let (quotient, remainder) = b.div_mod_floor(a);
        let (g, x, y) = egcd(&remainder, a);
        (g, y - (quotient * x.clone()), x)
    }
}

pub fn modinverse<T: Integer + Clone>(a: &T, m: &T) -> Option<T> {
    let (g, x, _) = egcd(a, m);
    if g != T::one() {
        None
    } else {
        Some((x.mod_floor(m) + m.clone()).mod_floor(m))
    }
}
