//! Module for utilities pertaining to random numbers

use crate::parsing::parse_dice_string;
use crate::parsing::DiceParseError;
use crate::parsing::DiceType;
use rand::{Rng, RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;

/// Struct for Random Number Generation
pub struct RandomNumberGenerator {
    rng: XorShiftRng,
}

impl RandomNumberGenerator {
    /// Creates a new RNG from a randomly generated seed
    #[allow(clippy::new_without_default)] // XorShiftRng doesn't have a Default, so we don't either
    pub fn new() -> RandomNumberGenerator {
        let rng: XorShiftRng = SeedableRng::from_entropy();
        RandomNumberGenerator { rng }
    }

    /// Creates a new RNG from a specific seed
    pub fn seeded(seed: u64) -> RandomNumberGenerator {
        let rng: XorShiftRng = SeedableRng::seed_from_u64(seed);
        RandomNumberGenerator { rng }
    }

    /// Returns a random value of whatever type you specify
    pub fn rand<T>(&mut self) -> T
    where
        rand::distributions::Standard: rand::distributions::Distribution<T>,
    {
        self.rng.gen::<T>()
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

    /// Returns the RNG's next unsigned-64 type
    pub fn next_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }

    /// Rolls dice based on a DiceType struct, including application of the bonus
    pub fn roll(&mut self, dice: DiceType) -> i32 {
        self.roll_dice(dice.n_dice, dice.die_type) + dice.bonus
    }

    /// Rolls dice based on passing in a string, such as roll_str("1d12")
    pub fn roll_str<S: ToString>(&mut self, dice: S) -> Result<i32, DiceParseError> {
        match parse_dice_string(&dice.to_string()) {
            Ok(dt) => Ok(self.roll(dt)),
            Err(e) => Err(e),
        }
    }

    /// Returns a random index into a slice
    pub fn random_slice_index<T>(&mut self, slice: &[T]) -> Option<usize> {
        if slice.is_empty() {
            None
        } else {
            let sz = slice.len();
            if sz == 1 {
                Some(0)
            } else {
                Some(self.roll_dice(1, sz as i32) as usize - 1)
            }
        }
    }

    /// Returns a random entry in a slice (or none if empty)
    pub fn random_slice_entry<'a, T>(&mut self, slice: &'a [T]) -> Option<&'a T> {
        if slice.is_empty() {
            None
        } else {
            let sz = slice.len();
            if sz == 1 {
                Some(&slice[0])
            } else {
                Some(&slice[self.roll_dice(1, sz as i32) as usize - 1])
            }
        }
    }
}
