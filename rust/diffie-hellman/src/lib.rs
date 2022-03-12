use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;
use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modpow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modpow(b_pub, a, p)
}

fn modpow(base: u64, exp: u64, modulo: u64) -> u64 {
    let base = BigUint::from(base);
    let exp = BigUint::from(exp);
    let modulo = BigUint::from(modulo);
    base.modpow(&exp, &modulo)
        .to_u64()
        .unwrap_or(0)
}
// fn modular_exponentiation(p: u64, b_pub: u64, a: u64) -> u64 {
//     // c = b_pub^a % p
//     // c =     b^e % m
//     // let mut c = 1;
//
//     if p == 1 {
//         return 0
//     }
//     let mut r = 1;
//     let mut b = b_pub % p;
//     let mut e = a;
//     let mut m = p;
//     while e > 0 {
//         if e % 2 == 1 {
//             r = r * b % m;
//         }
//         b = b * b % m;
//         e = e / 2;
//     }
//     r
// }