
//! Non-cryptographic RNG for people who just want to generate random numbers
//! for applications or procedural generation. 
//! 
//! ## Seed generation
//! 
//! This crate uses `web_time` to seed the rng when compiling for wasm, and 
//! `getrandom` on other platforms. It will only do this once per thread, and
//! will instantiate future seeds by updating a thread-local state. 
//! 
//! Seeds can be generated manually with the `seed` module.
//! ```
//! fn main() {
//!     let s1 = justrng::seed::from_local();
//!     let s2 = justrng::seed::from_system();
//! }
//! ```
//! 
//! ## Vector support
//! 
//! Permutation and WyRand support generating and mixing 
//! vectors when the `glam` feature is enabled.
//! 
//! ## WyRand
//! 
//! The main RNG exported by this module is WyRand, complete with 
//! hash generation for all primitives, generating in a range, and 
//! shuffling slices. 
//! 
//! ```
//! fn main() {
//!     // instantiate your RNG with thread-local seed
//!     let mut rng = justrng::WyRand::new();
//! 
//!     // generate random numbers
//!     let mut n1 = rng.next::<u32>();
//!     let mut r1 = rng.next_in_range::<i64>(0..256);
//!     let mut f1 = rng.next_in_range::<f32>(-16.0..32.0);
//! 
//!     // shuffle slices
//!     let mut slice: Vec<i64> = vec![0, 1, 2, 3, 4, 5];
//!     rng.shuffle(&mut slice);
//! }
//! ```
//! 
//! ## Permutation
//! 
//! An index-based rng that is lower quality than WyRand, but
//! is very fast. Permutations are primarily used in procedural 
//! generation to hash vector coordinates. 
//! 
//! Unlike standard RNGs, Permutations do not update any state 
//! when they are used. It will produce the same value every time 
//! if the mix input (and seed) is the same. 
//! 
//! ```
//! use glam::IVec3;
//! 
//! fn main() {
//!     // instantiate the Permutation with thread-local seed
//!     let mut rng = justrng::Permutation::new();
//! 
//!     // mix vector coordinates
//!     let m1 = rng.mix(IVec3::new(-1, 245, 3));
//!     let m2 = rng.mix(IVec3::new(3, 99, 21));
//!     let m3 = rng.mix(IVec3::new(94, -21, 33));
//! 
//!     // mix the same vector twice, produces the same result.
//!     let vec = IVec3::new(27, -9, 41);
//!     let v1 = rng.mix(vec);
//!     let v2 = rng.mix(vec);
//!     assert_eq!(v1, v2);
//! }
//! ```

pub mod seed;
pub mod perm;
pub mod wyrand;
pub mod primes;

pub use wyrand::WyRand;
pub use perm::Permutation;

use wyrand::{FromRng, RangeRng};
use std::ops::Range;

/// Generate a random number
pub fn next<T: FromRng>() -> T {
    T::from_rng(crate::seed::from_local())
}

/// Generate a random number within a range.
pub fn next_in_range<T: RangeRng>(range: Range<T>) -> T {
    T::from_range(crate::seed::from_local(), range)
}

/// Get an RNG seeded from system source.
pub fn rng() -> WyRand {
    WyRand::with_local_seed()
}


