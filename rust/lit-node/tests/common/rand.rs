// This file is largely copied inspired by rand::rngs::adapter::ThreadRng

use std::{cell::UnsafeCell, rc::Rc};

use rand::{rngs::StdRng, Rng};
use rand_core::{RngCore, SeedableRng};

thread_local!(
    static THREAD_RNG_KEY: Rc<UnsafeCell<StdRng>> = {
        // first use StdRng to generate a random seed
        let mut rng = StdRng::from_entropy();
        let seed = rng.gen_range(0..10000);

        // make it known what the random seed is!
        println!("seed: {:?}", seed);

        // then use seed to create an instance of StdRng
        let rng = StdRng::from_seed([seed as u8; 32]);
        Rc::new(UnsafeCell::new(rng))
    }
);

pub struct ThreadRng {
    rng: Rc<UnsafeCell<StdRng>>,
}

pub fn thread_rng() -> ThreadRng {
    ThreadRng {
        rng: THREAD_RNG_KEY.with(|rng| rng.clone()),
    }
}

impl RngCore for ThreadRng {
    fn next_u32(&mut self) -> u32 {
        unsafe { (*self.rng.get()).next_u32() }
    }
    fn next_u64(&mut self) -> u64 {
        unsafe { (*self.rng.get()).next_u64() }
    }
    fn fill_bytes(&mut self, slice: &mut [u8]) {
        unsafe { (*self.rng.get()).fill_bytes(slice) }
    }
    fn try_fill_bytes(&mut self, slice: &mut [u8]) -> std::result::Result<(), rand::Error> {
        unsafe { (*self.rng.get()).try_fill_bytes(slice) }
    }
}
