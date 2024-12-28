use risc0_zkvm::{guest::env, Journal};

fn main() {

    let image_id: [u8; 32] = env::read();
    let journals: Vec<Journal> = env::read();
    let length = journals.len();
    for j in journals {
        env::verify(image_id, &j.bytes).unwrap();
    }
    env::commit(&length);
}
