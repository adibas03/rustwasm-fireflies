// extern crate rand;

// use rand::prelude::*;
// use rand::random;

fn random_u8(available_values: &[u8]) -> u8 {
    // let mut rng = rand::thread_rng();
    // let n1: u8 = rng.gen::<u8>();
    let n1: u8 = rand::random::<u8>();
    return n1
}
