# Mass verification test

A tiny app to test mass verification of Risc0 (succinct) proofs via composition. The goal is to come up with a guest program that can verify up to 10k proofs in a reasonable time.

## Run 

`./sample-proofs` direcroty contains proofs(lifted segment aka SuccinctReceipt) of a sample guest proving. The directory shoudl be fed to test composition performance.

`cargo run -- -p sample-proofs/waldo/po2\=20`

