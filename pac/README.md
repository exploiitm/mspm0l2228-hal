# MSPM0L222X Peripheral Access Crate

PAC for TI MSPM0L2228 Mixed Signal Microcontroller - written for ECTF 2026 
by exploiitm.

This crate was generated from the SVD file found in the Arm Keil CMSIS Pack 
for the MSPM0L222X family using svd2rust. 

The provided SVD file has mistakes that were patched using svdtools.

## Building Crate

> [!NOTE]  
> This section is only relevant if you need to rebuild the crate.
> Generated code is already included in the repository.

```bash
cargo install svd2rust svdtools form
rustup component add --toolchain nightly rustfmt
make
```

Generate docs using 
```
cargo doc
```
_(Quality of Life)_



