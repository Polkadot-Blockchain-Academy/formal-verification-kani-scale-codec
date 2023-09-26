# Formal Verification for Substrate

In this exercise, we will use [Kani](https://github.com/model-checking/kani) to verify simple properties of the [SCALE Codec](https://github.com/paritytech/parity-scale-codec) in Rust.

#### Pre-requisites

Make sure you have cloned the [parity-scale-codec](https://github.com/paritytech/parity-scale-codec) repo, and have installed kani via cargo following the instructions [here](https://github.com/model-checking/kani#installation)

```sh
# Install and setup Kani
cargo install --locked kani-verifier
cargo kani setup
```

#### Tasks

- Familiarize yourself with what the codec is doing for integer types.
  Specifically, inspect the encode and decode _roundtrip_ `(decode(encode(x))==x)` tests for `u8`, `u16`,..., `u256`.
  Check sample tests [here](https://github.com/paritytech/parity-scale-codec/blob/e960eb9be27059ab0458cef68b80f9ed0db6e757/src/codec.rs#L1645) and try to come up with similar concrete value tests for single integers.
- Further, convert the previous concrete tests to Kani Proofs and verify the same property.
  Inject simple mistakes in the code logic and check the verification results.
- Try similarly for `f32` and observe results.
  If verification fails, conclude if either logic is wrong or there are more assumptions to be added in your proof harness.
- Try the same workflow for [compact](https://github.com/paritytech/parity-scale-codec/blob/master/src/compact.rs) encoding for integers.
  Use the tutorial [here](https://model-checking.github.io/kani/tutorial-loop-unwinding.html) to know how to verify code with loops.
- Write proof harnesses for more functionalities like DecodeLength (`DecodeLength(x) == Decode(x).length()`) and `EncodeAppend` (`EncodeAppend(vec,item) == Encode(vec.append(item))`).
