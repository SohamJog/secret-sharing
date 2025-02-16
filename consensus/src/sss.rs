//use lambdaworks_math::fft::{evaluate_offset_fft, interpolate_fft};
// use lambdaworks_math::fft::polynomial;

use lambdaworks_math::field::element::FieldElement;
use lambdaworks_math::field::fields::fft_friendly::stark_252_prime_field::Stark252PrimeField;
use lambdaworks_math::polynomial::Polynomial;
use lambdaworks_math::unsigned_integer::element::UnsignedInteger;

pub use num_bigint;

use rand;

use rand::thread_rng;

use rand::random;

type StarkField = FieldElement<Stark252PrimeField>;

// Basic version cloned from https://github.com/bitrocks/verifiable-secret-sharing/blob/master/src/simple_sss.rs

/// The `ShamirSecretSharing` stores threshold, share_amount and the prime of finite field.

#[derive(Clone, Debug)]
pub struct ShamirSecretSharing {
    /// the threshold of shares to recover the secret.
    pub threshold: usize,
    /// the total number of shares to generate from the secret.
    pub share_amount: usize,
    /// (not necessary) the prime number of the finite field.
    pub prime: StarkField,
}

impl ShamirSecretSharing {
    /// Split a secret according to the config.
    pub fn split(&self, secret: StarkField) -> Vec<(usize, StarkField)> {
        let polynomial = Self::sample_polynomial(secret, self.threshold);

        // Evaluate polynomial over a power-of-2 domain using FFT
        let blowup_factor = 1;
        let domain_size = Some(self.share_amount.next_power_of_two());
        let offset = StarkField::one();

        let shares =
            Polynomial::evaluate_offset_fft(&polynomial, blowup_factor, domain_size, &offset)
                .unwrap();

        // Convert to (index, value) pairs
        (1..=self.share_amount).zip(shares).collect()
    }

    pub fn rand_field_elements(order: usize) -> Vec<StarkField> {
        let mut result = Vec::with_capacity(order);
        for _ in 0..result.capacity() {
            let rand_big = UnsignedInteger { limbs: random() };
            result.push(StarkField::new(rand_big));
        }
        result
    }

    fn sample_polynomial(secret: StarkField, threshold: usize) -> Polynomial<StarkField> {
        let mut coefficients: Vec<StarkField> = Self::rand_field_elements(threshold - 1);
        // edit first element to secret
        coefficients[0] = secret;

        Polynomial::new(&coefficients[..])
    }

    /// Recover the secret by the shares.

    pub fn recover(&self, shares: &[(usize, StarkField)]) -> StarkField {
        let (xs, ys): (Vec<usize>, Vec<StarkField>) = shares.iter().cloned().unzip();

        // Interpolate polynomial from shares
        let poly = Polynomial::interpolate_fft::<Stark252PrimeField>(&ys).unwrap();

        // Evaluate the polynomial at x = 0 (constant term is the secret)
        poly.evaluate(&StarkField::zero())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_wikipedia_example() {
        let sss = ShamirSecretSharing {
            threshold: 3,
            share_amount: 6,
            prime: BigInt::from(1613),
        };
        let shares = sss.evaluate_polynomial(vec![
            BigInt::from(1234),
            BigInt::from(166),
            BigInt::from(94),
        ]);
        assert_eq!(
            shares,
            [
                (1, BigInt::from(1494)),
                (2, BigInt::from(329)),
                (3, BigInt::from(965)),
                (4, BigInt::from(176)),
                (5, BigInt::from(1188)),
                (6, BigInt::from(775))
            ]
        );
        assert_eq!(
            sss.recover(&[
                (1, BigInt::from(1494)),
                (2, BigInt::from(329)),
                (3, BigInt::from(965))
            ]),
            BigInt::from(1234)
        );
    }
    #[test]
    fn test_large_prime() {
        let sss = ShamirSecretSharing {
            threshold: 3,
            share_amount: 5,
            // prime: BigInt::from(6999213259363483493573619703 as i128),
            prime: BigInt::parse_bytes(
                b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f",
                16,
            )
            .unwrap(),
        };
        let secret = BigInt::parse_bytes(b"ffffffffffffffffffffffffffffffffffffff", 16).unwrap();
        let shares = sss.split(secret.clone());
        assert_eq!(secret, sss.recover(&shares[0..sss.threshold as usize]));
    }
}
