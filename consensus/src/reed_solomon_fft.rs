use crate::{sss, LargeField, ShamirSecretSharingFFT};
use reed_solomon_erasure::{galois_8::ReedSolomon, Error};
use lambdaworks_math::traits::ByteConversion;
/*
 *  Steps to use FFT SSS in Reed solomon:
 * ***ENCODE***
 * Split data into k parts -> [data]
 * Generate k StarkField Elements from bytes be from [data] -> [elements]
 * interpolate the polynomial from [elements] -> [polynomial_coefficients]
 * Evaluate the polynomial at n points using FFT -> [shares_lw]
 * Convert the Starkfield elements to bytes -> [shares]
 *
 * ***DECODE***
 * Convert the bytes to Starkfield elements -> [shares]
 * Convert evaluation points to Roots of Unity -> [eval_points]
 * Interpolate the polynomial from [shares, eval_points] -> [polynomial_coefficients]
 * Evaluate the polynomial at N points using FFT -> [shares_lw]
 * Return the first t shares, convert to bytes -> [data]
 *

*/

pub fn get_shards(data: Vec<u8>, shards: usize, parity_shards: usize) -> Vec<Vec<u8>> {
    //  * ***ENCODE***
    //  * Split data into k(shards) parts -> [data]
    let original_size = data.len();
    let k = shards;
    let n = k + parity_shards;

    let size = if original_size % k == 0 {
        original_size
    } else {
        original_size + (k - (original_size % k))
    };
    let block_size = size / k;

    let mut output_shards =  Vec::new();
    for i in 0..k {
        let mut shard = vec![0; block_size];
        shard.copy_from_slice(&data[i * block_size..(i + 1) * block_size]);
        output_shards.push(shard);
    }
    assert!(output_shards.len() == k);
    //  * Generate k StarkField Elements from bytes be from [data] -> [elements]
    let mut elements: Vec<LargeField> = Vec::new();
    for i in 0..k {
        elements.push(LargeField::from_bytes_be(&output_shards[i]).unwrap());
    }
    //  * interpolate the polynomial from [elements] -> [polynomial_coefficients]
    let sss = ShamirSecretSharingFFT::new(k, n);
    sss.fill_evaluation_at_all_points(&mut elements);

    //  * Evaluate the polynomial at n points using FFT -> [shares_lw]
    //  * Convert the Starkfield elements to bytes -> [shares
    output_shards.clear();
    for i in 0..n {
        let mut shard = vec![0; block_size];
        shard.copy_from_slice(&elements[i].to_bytes_be().to_vec());
        output_shards.push(shard);
    }
    //  *
    output_shards
}

// The shards are reconstructed inline with the variable data
pub fn reconstruct_data(
    data: &mut Vec<Option<Vec<u8>>>,
    shards: usize,
    parity_shards: usize,
) -> Result<(), Error> {
    let k = shards;
    let n = k + parity_shards;
    let mut elements: Vec<LargeField> = Vec::new();
    let mut eval_points: Vec<u64> = Vec::new();

    for i in 0..n {
        if let Some(share) = &data[i] {
            elements.push(LargeField::from_bytes_be(&share).unwrap());
            eval_points.push(i as u64);
        }
    }

    let sss = ShamirSecretSharingFFT::new(k, n);
    let polynomial_coeffs = sss.reconstructing(&eval_points, &elements);

    let shares = sss.generating_shares(&polynomial_coeffs);

    for i in 0..data.len() {
        if let Some(share) = &data[i] {
            assert!(share == &shares[i].to_bytes_be().to_vec());
        }
        data[i] = Some(shares[i].to_bytes_be().to_vec());
    }


    Ok(())
}


// test cases to compare this implementation with the original one
