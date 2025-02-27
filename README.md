# secret-sharing

Rust implementation of Shamir's secret sharing with Lambdaworks polynomials and FFT-friendly fields. Using the [Stark 252 prime field](https://github.com/lambdaclass/lambdaworks/blob/main/math/src/field/fields/fft_friendly/stark_252_prime_field.rs) from Lambdaworks. 

# To-Do List  

- [x] Create regular SSS scheme with tests  
- [ ] Create SSS scheme with roots of unity  
- [ ] Add FFT evaluation  
- [ ] Unit testing  
- [ ] Create benchmarks for both implementations and compare  
- [ ] Add documentation

## Directory Structure


Currently, the project is still a WIP. I have 3 Reliable Broadcast protocols in the `consensus` folder. The `consensus/src/shamir` folder contains the secret sharing protocol. The `scripts` folder contains scripts to test the protocols. The `testdata` folder contains the configurations for the tests.

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

3. Script to test with test_msgs.txt

```bash
./scripts/test.sh testdata/hyb_16/syncer Hi false testdata/test_msgs.txt
```