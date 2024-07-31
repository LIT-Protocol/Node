mod deriver;
mod error;

pub use error::*;

use cait_sith::{CSCurve, KeygenOutput};
use std::fmt::{self, Debug, Display, Formatter, LowerHex, UpperHex};

use k256::elliptic_curve::group::cofactor::CofactorGroup;
use k256::elliptic_curve::hash2curve::FromOkm;
use k256::elliptic_curve::{
    group::Curve, hash2curve::GroupDigest, CurveArithmetic, Field, Group, PrimeField,
};
use p256::elliptic_curve::ScalarPrimitive;
use serde::{Deserialize, Serialize};

use deriver::*;
pub use deriver::{compute_rerandomizer, update_cait_sith_presig};

#[derive(Debug, Clone, Copy, Hash, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub struct HdKeyDeriver<C>(C::Scalar)
where
    C: GroupDigest,
    <C as CurveArithmetic>::ProjectivePoint: CofactorGroup,
    <C as CurveArithmetic>::Scalar: FromOkm;

impl<C> Display for HdKeyDeriver<C>
where
    C: GroupDigest,
    <C as CurveArithmetic>::ProjectivePoint: CofactorGroup,
    <C as CurveArithmetic>::Scalar: FromOkm,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self)
    }
}

impl<C> LowerHex for HdKeyDeriver<C>
where
    C: GroupDigest,
    <C as CurveArithmetic>::ProjectivePoint: CofactorGroup,
    <C as CurveArithmetic>::Scalar: FromOkm,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for b in self.0.to_repr().as_ref() {
            write!(f, "{:x}", b)?;
        }
        Ok(())
    }
}

impl<C> UpperHex for HdKeyDeriver<C>
where
    C: GroupDigest,
    <C as CurveArithmetic>::ProjectivePoint: CofactorGroup,
    <C as CurveArithmetic>::Scalar: FromOkm,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for b in self.0.to_repr().as_ref() {
            write!(f, "{:X}", b)?;
        }
        Ok(())
    }
}

impl<C> HdKeyDeriver<C>
where
    C: GroupDigest,
    <C as CurveArithmetic>::ProjectivePoint: CofactorGroup,
    <C as CurveArithmetic>::Scalar: FromOkm,
{
    pub fn new(id: &[u8], cxt: &[u8]) -> Result<Self, Error> {
        Ok(Self(hash_to_scalar::<C>(id, cxt)?))
    }

    pub fn new_from_scalar_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let sp = ScalarPrimitive::<C>::from_slice(bytes)?;
        Ok(Self(C::Scalar::from(sp)))
    }

    pub fn compute_secret_key_share_cait_sith<S: CSCurve>(
        &self,
        shares: &[KeygenOutput<S>],
    ) -> Result<KeygenOutput<S>, Error> {
        let secrets = shares
            .iter()
            .map(|s| {
                let s = s.private_share.to_repr();
                let mut repr = <C::Scalar as Field>::ONE.to_repr();
                repr.as_mut().copy_from_slice(s.as_ref());
                C::Scalar::from_repr(repr)
            })
            .collect::<Vec<_>>();
        if secrets.iter().any(|s| s.is_none().into()) {
            return Err(Error::CurveMismatchOrInvalidShare);
        }
        let mut secrets = secrets.into_iter().map(|s| s.unwrap()).collect::<Vec<_>>();
        let share = self.compute_secret_key(&secrets)?;
        // NOTE: it would be better to call zeroize on the secrets but that would require
        // updating Cait-Sith to specify Scalar implements the Zeroize trait.
        // TODO: check if this is optimized away by the compiler.
        secrets.iter_mut().for_each(|s| *s = C::Scalar::ZERO);

        let mut repr = <S::Scalar as Field>::ONE.to_repr();
        repr.as_mut().copy_from_slice(share.to_repr().as_ref());
        let private_share = Option::<S::Scalar>::from(S::Scalar::from_repr(repr))
            .ok_or(Error::CurveMismatchOrInvalidShare)?;
        let public_key = S::ProjectivePoint::generator() * private_share;
        Ok(KeygenOutput {
            private_share,
            public_key: public_key.to_affine(),
        })
    }

    pub fn compute_secret_key(&self, shares: &[C::Scalar]) -> Result<C::Scalar, Error> {
        let mut result = C::Scalar::ZERO;

        // Compute the polynomial value using Horner's Method
        for share in shares.iter().rev() {
            // b_{n-1} = a_{n-1} + b_n*x
            if self.0.is_zero().into() {
                return Err(Error::CurveMismatchOrInvalidShare);
            }

            result *= self.0;
            result += share;
        }

        if result.is_zero().into() {
            return Err(Error::CurveMismatchOrInvalidShare);
        };

        Ok(result)
    }

    pub fn compute_public_key(&self, public_keys: &[C::ProjectivePoint]) -> C::ProjectivePoint {
        let mut powers = vec![<C::Scalar as Field>::ONE; public_keys.len()];
        powers[1] = self.0;
        for i in 2..powers.len() {
            powers[i] = powers[i - 1] * self.0;
        }
        sum_of_products_pippenger::<C>(public_keys, &powers)
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        let mut out = [0u8; 32];
        out.copy_from_slice(self.0.to_repr().as_ref());
        out
    }

    pub fn from_bytes(bytes: &[u8; 32]) -> Result<Self, Error> {
        let mut repr = <C::Scalar as Field>::ONE.to_repr();
        repr.as_mut().copy_from_slice(bytes);
        let inner = Option::<C::Scalar>::from(C::Scalar::from_repr(repr))
            .ok_or(Error::CurveMismatchOrInvalidShare)?;
        Ok(HdKeyDeriver(inner))
    }

    pub fn to_inner(self) -> C::Scalar {
        self.0
    }

    pub fn from_inner(inner: C::Scalar) -> Self {
        Self(inner)
    }
}

fn sum_of_products_pippenger<C: CurveArithmetic>(
    points: &[C::ProjectivePoint],
    scalars: &[C::Scalar],
) -> C::ProjectivePoint {
    const WINDOW: usize = 4;
    const NUM_BUCKETS: usize = 1 << WINDOW;
    const EDGE: usize = WINDOW - 1;
    const MASK: u64 = (NUM_BUCKETS - 1) as u64;

    let scalars = scalars
        .iter()
        .map(|s| {
            let mut out = [0u64; 4];
            let primitive: ScalarPrimitive<C> = (*s).into();
            out.copy_from_slice(
                primitive
                    .as_limbs()
                    .iter()
                    .map(|l| l.0)
                    .collect::<Vec<_>>()
                    .as_slice(),
            );
            out
        })
        .collect::<Vec<_>>();
    let num_components = std::cmp::min(points.len(), scalars.len());
    let mut buckets = [<C::ProjectivePoint as Group>::identity(); NUM_BUCKETS];
    let mut res = C::ProjectivePoint::identity();
    let mut num_doubles = 0;
    let mut bit_sequence_index = 255usize;

    loop {
        for _ in 0..num_doubles {
            res = res.double();
        }

        let mut max_bucket = 0;
        let word_index = bit_sequence_index >> 6;
        let bit_index = bit_sequence_index & 63;

        if bit_index < EDGE {
            // we are on the edge of a word; have to look at the previous word, if it exists
            if word_index == 0 {
                // there is no word before
                let smaller_mask = ((1 << (bit_index + 1)) - 1) as u64;
                for i in 0..num_components {
                    let bucket_index: usize = (scalars[i][word_index] & smaller_mask) as usize;
                    if bucket_index > 0 {
                        buckets[bucket_index] += points[i];
                        if bucket_index > max_bucket {
                            max_bucket = bucket_index;
                        }
                    }
                }
            } else {
                // there is a word before
                let high_order_mask = ((1 << (bit_index + 1)) - 1) as u64;
                let high_order_shift = EDGE - bit_index;
                let low_order_mask = ((1 << high_order_shift) - 1) as u64;
                let low_order_shift = 64 - high_order_shift;
                let prev_word_index = word_index - 1;
                for i in 0..num_components {
                    let mut bucket_index =
                        ((scalars[i][word_index] & high_order_mask) << high_order_shift) as usize;
                    bucket_index |= ((scalars[i][prev_word_index] >> low_order_shift)
                        & low_order_mask) as usize;
                    if bucket_index > 0 {
                        buckets[bucket_index] += points[i];
                        if bucket_index > max_bucket {
                            max_bucket = bucket_index;
                        }
                    }
                }
            }
        } else {
            let shift = bit_index - EDGE;
            for i in 0..num_components {
                let bucket_index: usize = ((scalars[i][word_index] >> shift) & MASK) as usize;
                if bucket_index > 0 {
                    buckets[bucket_index] += points[i];
                    if bucket_index > max_bucket {
                        max_bucket = bucket_index;
                    }
                }
            }
        }
        res += &buckets[max_bucket];
        for i in (1..max_bucket).rev() {
            buckets[i] += buckets[i + 1];
            res += buckets[i];
            buckets[i + 1] = C::ProjectivePoint::identity();
        }
        buckets[1] = C::ProjectivePoint::identity();
        if bit_sequence_index < WINDOW {
            break;
        }
        bit_sequence_index -= WINDOW;
        num_doubles = {
            if bit_sequence_index < EDGE {
                bit_sequence_index + 1
            } else {
                WINDOW
            }
        };
    }
    res
}

// #[test]
// fn pippinger_k256_known() {
//     let points = [k256::ProjectivePoint::GENERATOR; 3];
//     let scalars = [
//         k256::Scalar::from(1u64),
//         k256::Scalar::from(2u64),
//         k256::Scalar::from(3u64),
//     ];
//     let expected = points[0] * scalars[0] + points[1] * scalars[1] + points[2] * scalars[2];

//     let actual = sum_of_products_pippenger::<k256::Secp256k1>(&points, &scalars);

//     assert_eq!(expected, actual);
// }

// #[test]
// fn pippinger_schnorr_proof() {
//     let mut rng = rand::thread_rng();

//     for _ in 0..25 {
//         let h0 = k256::ProjectivePoint::random(&mut rng);
//         let s = k256::Scalar::random(&mut rng);
//         let s_tilde = k256::Scalar::random(&mut rng);
//         let c = k256::Scalar::random(&mut rng);

//         assert_eq!(
//             h0 * s,
//             sum_of_products_pippenger::<k256::Secp256k1>(&[h0], &[s])
//         );

//         assert_eq!(
//             h0 * s_tilde,
//             sum_of_products_pippenger::<k256::Secp256k1>(&[h0], &[s_tilde])
//         );

//         let u = h0 * s;
//         let u_tilde = h0 * s_tilde;
//         let s_hat = s_tilde - c * s;
//         assert_eq!(u_tilde, u * c + h0 * s_hat);
//         assert_eq!(
//             u_tilde,
//             sum_of_products_pippenger::<k256::Secp256k1>(&[u, h0], &[c, s_hat])
//         )
//     }
// }

// #[test]
// fn pippinger_p256_known() {
//     let points = [p256::ProjectivePoint::generator(); 3];
//     let scalars = [
//         p256::Scalar::from(1u64),
//         p256::Scalar::from(2u64),
//         p256::Scalar::from(3u64),
//     ];
//     let expected = points[0] * scalars[0] + points[1] * scalars[1] + points[2] * scalars[2];

//     let actual = sum_of_products_pippenger::<p256::NistP256>(&points, &scalars);

//     assert_eq!(expected, actual);
// }

// #[test]
// fn compute_secret_key() {
//     let mut rng = rand::thread_rng();
//     let d0 = k256::Scalar::random(&mut rng);
//     let d1 = k256::Scalar::random(&mut rng);

//     let d0_shares = vsss_rs::shamir::split_secret::<k256::Scalar, u8, Vec<u8>>(2, 3, d0, &mut rng)
//         .unwrap()
//         .iter()
//         .map(|s| <Vec<u8> as vsss_rs::Share>::as_field_element::<k256::Scalar>(s).unwrap())
//         .collect::<Vec<_>>();
//     let d1_shares = vsss_rs::shamir::split_secret::<k256::Scalar, u8, Vec<u8>>(2, 3, d1, &mut rng)
//         .unwrap()
//         .iter()
//         .map(|s| <Vec<u8> as vsss_rs::Share>::as_field_element::<k256::Scalar>(s).unwrap())
//         .collect::<Vec<_>>();

//     let deriver =
//         HdKeyDeriver::<k256::Secp256k1>::new(b"id", b"LIT_HD_KEY_ID_secp256k1_XMD:SHA-256_SSWU_RO_NUL_")
//             .unwrap();
//     let p0 = deriver.compute_secret_key(&[d0_shares[0], d1_shares[0]]);
//     let p1 = deriver.compute_secret_key(&[d0_shares[1], d1_shares[1]]);

//     let shares = [p0, p1]
//         .iter()
//         .enumerate()
//         .map(|(i, p)| <Vec<u8> as vsss_rs::Share>::from_field_element((i + 1) as u8, *p).unwrap())
//         .collect::<Vec<_>>();
//     let p = vsss_rs::combine_shares::<k256::Scalar, u8, Vec<u8>>(&shares).unwrap();

//     assert_eq!(p, d0 + d1 * deriver.to_inner());
// }
