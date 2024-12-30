use risc0_zkvm::{guest::env, ReceiptClaim};

fn main() {
    let claims: Vec<ReceiptClaim> = env::read();
    for c in &claims {
        env::verify_integrity(c).unwrap();
    }
    env::commit(&claims.len());
}
