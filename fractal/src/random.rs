//! Random module

use rand::{Rng, RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;

/// Internal RNG
pub struct RandomNumberGenerator {
    rng: XorShiftRng,
}

impl RandomNumberGenerator {
    /// Create a new RanfomNumberGenerator instance
    pub fn new() -> Self {
        Self {
            rng: SeedableRng::from_entropy(),
        }
    }

    /// Returns a random value in the specified range, of type specified at the call site.
    /// This is INCLUSIVE of the first parameter, and EXCLUSIVE of the second.
    /// So range(1,6) will give you numbers from 1 to 5.
    pub fn range<T>(&mut self, min: T, max: T) -> T
    where
        T: rand::distributions::uniform::SampleUniform,
    {
        self.rng.gen_range(min, max)
    }

    /// Rolls dice, using the classic 3d6 type of format: n is the number of dice, die_type is the size of the dice.
    pub fn roll_dice(&mut self, n: i32, die_type: i32) -> i32 {
        (0..n).map(|_| self.range(1, die_type + 1)).sum()
    }
}
