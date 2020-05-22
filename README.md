# Rust File Hash -> AccountId Map Contract

This contract provides a generic Vec<u8> map to accountId.

The intention is to demonstrate file provenance, ownership, etc... based on a file hash. By paying to add your file to the map you achieve 2 objectives:
1. prove some ownership, provenance, knowledge of the file
2. prove (1) at a point in time, inclusion in the Near blockchain

## Reqquirements

Latest rust. Recommended nightly.

## Tests

`./test.sh` to see full output or `cargo test`
