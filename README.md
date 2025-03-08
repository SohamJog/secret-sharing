# secret-sharing


Rust implementation of Shamir's secret sharing with Lambdaworks polynomials and FFT-friendly fields. Using the [Stark 252 prime field](https://github.com/lambdaclass/lambdaworks/blob/main/math/src/field/fields/fft_friendly/stark_252_prime_field.rs) from Lambdaworks. 

I wrote a [post](https://www.sohamjog.com/blog/shamir) about this optimization.

## Directory Structure


Currently, the project is still a WIP. I have 3 Reliable Broadcast protocols in the `consensus` folder. The `consensus/src/sss` folder contains the secret sharing protocol. The `consensus/src/sss_fft` file is where I am attempting to speed up the share creation step of secret sharing. 

The `scripts` folder contains scripts to test the protocols. The `testdata` folder contains the configurations for the tests.

---

## Benchmarks

You can find the benchmarks in `consensus/benches/`. To run the benchmarks, use the following command:

```bash
cd consensus
cargo bench
```

The following are the results after running the benchmarks on my machine:

### Benchmark Results for Shamir's Secret Sharing (n = 32, threshold = 16)

Comparing between SSS without Lambdaworks, SSS with lambdaworks without FFT, and SSS with Lambdaworks FFT.

| Task                                   | Time (µs)           | % Outliers |
|----------------------------------------|---------------------|------------|
| **Share Generation**                  | 146.98 - 149.08    | 7.00%      |
| **Share Generation using Lambdaworks**| 11.496 - 12.480    | 7.00%      |
| **Share Generation using FFT**        | 4.7853 - 4.8140    | 24.00%     |
| **Reconstructing Secret**              | 189.70 - 190.29    | 7.00%      |
| **Reconstructing Secret using LW**     | 146.34 - 147.32    | 5.00%      |
| **Reconstructing Secret using FFT**    | 144.74 - 145.88    | 4.00%      |
| **Fill Evaluation at all points**      | 268.38 - 270.27    | 7.00%      |
| **Fill Evaluation at all points using LW**  | 222.58 - 223.84    | 22.00%     |
| **Fill Evaluation at all points (FFT)**| 206.96 - 211.90    | 13.00%     |

### Observations:
- **Generating shares with FFT is significantly faster** 
- **Reconstruction using FFT does not provide a drastic improvement**, which is expected due to interpolation.
- **FFT-based fill evaluation is slightly faster**.

---

## Notes
1. Kill these processes
```bash
sudo lsof -ti :7000-7015,5000 | sudo xargs kill -9
```

2. Script to test with n number of nodes (in case you lose config by switching branches)
```bash
cd testdata
mkdir hyb_16
./target/release/genconfig --NumNodes 16 --delay 10 --blocksize 100 --client_base_port 7000 --target testdata/hyb_16/ --payload 100 --out_type json --base_port 9000 --client_run_port 4000 --local true
```

3. Script to test RBC with test_msgs.txt

```bash
./scripts/test.sh testdata/hyb_16/syncer Hi false testdata/test_msgs.txt
```

4. SSS Unit Tests

```bash
cargo test
```
