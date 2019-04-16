# zcrash

Steps to get vulnerable parameters from Zcash Sprout MPC transcript:
1) clone the MPC ceremony repository: https://github.com/zcash/mpc
2) Make sure you have libsnark library and install all dependencies.
3) Download Zcash transcript and r1cs (available in mpc readme) and ensure they are in mpc folder.
4) Add get_vuln.rs to the /mpc/ folder.
5) Replace libsnarkwrap.cpp in /mpc/src/ with the modified version.
6) In the libsnark library, replace /libsnark/src/algebra/curves/alt_bn128/alt_bn128_g1.cpp and .hpp with modified versions. These modifications provide the ability to write parameters to txt file.
7) To build and run: **cargo run --release --bin get_vuln**
8) Files pk_a.txt and pk_a_prime.txt, both 220 MB, will be output.
