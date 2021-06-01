Used for generating Rust crate from [miracl's core library](https://github.com/miracl/core) library. Any curve can be selected by compiling with that feature. eg. to build library for curves ed25519, bls12461 and bn254, use (from `rust_64` directory)
```
cargo build --features "ed25519 bls12461 bn254"
``` 
The default feature is curve bls12381 only.


## Updates from upstream

To fetch updates from upstream (miracl/core) and build crate (for 64-bit only), use

```
python gen_rust_64.py
```

This will pull upstream changes in `miracl_core` directory and build the crate in `rust_64` directory. The crate name is `miracl_core`.

When new curves are added in upstream, add the number(s) corresponding to the curve (most likely 1 more than the max number) in file `curve_nos.txt` and the corresponding curves should be added as features in `Cargo.toml` and `lib.rs` of crate in `rust_64`. This is tedious but it avoidance needs some refactoring in miracl/core's `rust/config64.py` file.

Increase version before publishing `rust_64` to crates.io.