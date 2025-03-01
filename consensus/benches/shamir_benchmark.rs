use consensus::{LargeField, ShamirSecretSharing, ShamirSecretSharingFFT};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkGroup, Criterion};
use lambdaworks_math::field::element::FieldElement;
use lambdaworks_math::field::fields::fft_friendly::stark_252_prime_field::Stark252PrimeField;
use lambdaworks_math::unsigned_integer::element::UnsignedInteger;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::time::Duration;

fn bench_shamir_secret_sharing(c: &mut Criterion) {
    type LargeField = FieldElement<Stark252PrimeField>; // Alias for LargeField
    let secret = LargeField::new(UnsignedInteger::from(1234u64));

    let sss = ShamirSecretSharing {
        share_amount: 32,
        threshold: 16,
    };

    c.bench_function("Secret Generation", |b| {
        b.iter(|| {
            let shares = sss.split(secret);
        })
    });
}

fn bench_shamir_secret_sharing_reconstruct(c: &mut Criterion) {
    type LargeField = FieldElement<Stark252PrimeField>; // Alias for LargeField
    let secret = LargeField::new(UnsignedInteger::from(1234u64));
    let sss = ShamirSecretSharing {
        share_amount: 32,
        threshold: 16,
    };
    let shares = sss.split(secret);

    // let shares to use x be all even numbers from 2 to 32
    let shares_to_use_x: Vec<LargeField> = (2..=32)
        .filter(|x| x % 2 == 0)
        .map(|x| LargeField::new(UnsignedInteger::from(x as u64)))
        .collect();
    let shares_to_use_y: Vec<LargeField> = shares
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 1)
        .map(|(_, y)| *y)
        .collect();
    c.bench_function("Reconstructing Secret", |b| {
        b.iter(|| {
            let poly_2 = sss.reconstructing(&shares_to_use_x, &shares_to_use_y);
            let secret_recovered = sss.recover(&poly_2);
            assert_eq!(secret, secret_recovered);
        })
    });
}

fn bench_shamir_secret_sharing_fft(c: &mut Criterion) {
    type LargeField = FieldElement<Stark252PrimeField>; // Alias for LargeField
    let secret = LargeField::new(UnsignedInteger::from(1234u64));

    let sss = ShamirSecretSharingFFT {
        share_amount: 32,
        threshold: 16,
        roots_of_unity: ShamirSecretSharingFFT::gen_roots_of_unity(32),
    };

    c.bench_function("Secret Generation using FFT", |b| {
        b.iter(|| {
            let shares = sss.split(secret);
        })
    });
}

fn bench_shamir_secret_sharing_reconstruct_fft(c: &mut Criterion) {
    type LargeField = FieldElement<Stark252PrimeField>; // Alias for LargeField
    let secret = LargeField::new(UnsignedInteger::from(1234u64));
    let sss = ShamirSecretSharingFFT {
        share_amount: 32,
        threshold: 16,
        roots_of_unity: ShamirSecretSharingFFT::gen_roots_of_unity(32),
    };
    let shares = sss.split(secret);

    // let shares to use x be all even numbers from 2 to 32
    let shares_to_use_x: Vec<u64> = (2..=32)
        .filter(|x| x % 2 == 0)
        .map(|x| (x as u64))
        .collect();
    let shares_to_use_y: Vec<LargeField> = shares
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 1)
        .map(|(_, y)| *y)
        .collect();
    c.bench_function("Reconstructing Secret Using FFT", |b| {
        b.iter(|| {
            let poly_2 = sss.reconstructing(&shares_to_use_x, &shares_to_use_y);
            let secret_recovered = sss.recover(&poly_2);
            assert_eq!(secret, secret_recovered);
        })
    });
}

fn bench_fill_evaluation_at_all_points_fft(c: &mut Criterion) {
    type LargeField = FieldElement<Stark252PrimeField>; // Alias for LargeField
    let secret = LargeField::new(UnsignedInteger::from(1234u64));

    let sss = ShamirSecretSharingFFT {
        share_amount: 32,
        threshold: 16,
        roots_of_unity: ShamirSecretSharingFFT::gen_roots_of_unity(32),
    };

    // generate polynomial, generate shares, then create a new vector with the first t+1 shares and the secret, and then verify that its equal to the shares polynomial after fill evals at all points

    c.bench_function("Fill Evaluation at all points using FFT", |b| {
        b.iter(|| {
            let polynomial = sss.sample_polynomial(secret);
            let shares = sss.generating_shares(&polynomial);
            let mut shares_to_use = Vec::new();
            shares_to_use.push(secret);
            shares_to_use.extend(shares[0..sss.threshold + 1].to_vec());

            sss.fill_evaluation_at_all_points(&mut shares_to_use);
            // assert first element of shares_to_use is equal to secret
            assert_eq!(shares_to_use[0], secret);
            // remove shares_to_use[0]
            shares_to_use.remove(0);
            // assert shares_to_use is equal to shares
            assert_eq!(shares_to_use, shares);
        })
    });
}

fn bench_fill_evaluation_at_all_points(c: &mut Criterion) {
    type LargeField = FieldElement<Stark252PrimeField>; // Alias for LargeField
    let secret = LargeField::new(UnsignedInteger::from(1234u64));

    let sss = ShamirSecretSharing {
        share_amount: 32,
        threshold: 16,
    };

    // generate polynomial, generate shares, then create a new vector with the first t+1 shares and the secret, and then verify that its equal to the shares polynomial after fill evals at all points
    c.bench_function("Fill Evaluation at all points", |b| {
        b.iter(|| {
            let polynomial = sss.sample_polynomial(secret);
            let shares = sss.generating_shares(&polynomial);
            let mut shares_to_use = Vec::new();
            shares_to_use.push(secret);
            shares_to_use.extend(shares[0..sss.threshold + 1].to_vec());

            sss.fill_evaluation_at_all_points(&mut shares_to_use);
            // remove shares_to_use[0]
            shares_to_use.remove(0);
            // assert shares_to_use is equal to shares
            assert_eq!(shares_to_use, shares);
        })
    });
}

criterion_group!(
    benches,
    bench_shamir_secret_sharing,
    bench_shamir_secret_sharing_fft,
    bench_shamir_secret_sharing_reconstruct,
    bench_shamir_secret_sharing_reconstruct_fft,
    bench_fill_evaluation_at_all_points,
    bench_fill_evaluation_at_all_points_fft
);
criterion_main!(benches);
