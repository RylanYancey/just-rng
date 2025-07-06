
use std::ops::Range;
#[cfg(feature = "glam")]
use glam::{IVec2, IVec3, IVec4, UVec2, UVec3, UVec4};
use crate::primes::*;

/// A small, highly efficient WyRand implementation.
#[derive(Copy, Clone)]
pub struct WyRand {
    /// The current value of the RNG.
    state: u64,
}

impl WyRand {
    /// Construct a new WyRand instance.
    pub fn new() -> Self {
        Self::with_local_seed()
    }

    /// Construct a new WyRand instance with your own seed.
    pub fn with_seed(seed: u64) -> Self {
        Self { state: seed }
    }

    /// Construct the WyRand instance with a seed generated from the
    /// thread-local WyRand seed generator, which is seeded from system
    /// source.
    pub fn with_local_seed() -> Self {
        Self::with_seed(crate::seed::from_local())
    }

    /// Construct a WyRand instance from system source 
    /// when on x86 and web_time::SystemTime when on wasm.
    /// 
    /// This IS a system call on x86 - shouldn't be used frequently.
    pub fn with_system_seed() -> Self {
        Self::with_seed(crate::seed::from_system())
    }

    /// Generate a value by updating and hashing the state.
    pub fn next<T: FromRng>(&mut self) -> T {
        self.state = self.state.wrapping_add(P0);
        let r = u128::from(self.state).wrapping_mul(u128::from(self.state ^ P1));
        T::from_rng((r.wrapping_shr(64) ^ r) as u64)
    }

    /// Generate a value by updating and hashing the state, then wrapping to the range. 
    pub fn next_in_range<T: RangeRng>(&mut self, range: Range<T>) -> T {
        T::from_range(self.next(), range)
    }

    /// Shuffle a slice 
    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        for i in 0..slice.len() {
            slice.swap(i, self.next_in_range(0..slice.len()))
        }
    }
}

pub trait FromRng {
    fn from_rng(v: u64) -> Self;
}

impl FromRng for u64 {
    fn from_rng(v: u64) -> Self {
        v
    }
}

impl FromRng for i64 {
    fn from_rng(v: u64) -> Self {
        v as i64
    }
}

impl FromRng for usize {
    fn from_rng(v: u64) -> Self {
        v as usize
    }
}

impl FromRng for isize {
    fn from_rng(v: u64) -> Self {
        v as isize
    }
}

impl FromRng for u32 {
    fn from_rng(v: u64) -> Self {
        (v >> 32) as u32
    }
}

impl FromRng for i32 {
    fn from_rng(v: u64) -> Self {
        (v >> 32) as i32
    }
}

impl FromRng for u16 {
    fn from_rng(v: u64) -> Self {
        (v & 0xFFFF) as u16
    }
}

impl FromRng for i16 {
    fn from_rng(v: u64) -> Self {
        (v & 0xFFFF) as i16
    }
}

impl FromRng for u8 {
    fn from_rng(v: u64) -> Self {
        (v & 0xFF) as u8
    }
}

impl FromRng for i8 {
    fn from_rng(v: u64) -> Self {
        (v & 0xFF) as i8
    }
}

impl FromRng for f64 {
    fn from_rng(v: u64) -> Self {
        v as f64 / u64::MAX as f64
    }
}

impl FromRng for f32 {
    fn from_rng(v: u64) -> Self {
        (v as f64 / u64::MAX as f64) as f32
    }
}

#[cfg(feature = "glam")]
impl FromRng for IVec2 {
    fn from_rng(v: u64) -> Self {
        IVec2 {
            x: (v & 0xFFFFFFFF) as i32,
            y: (v >> 32) as i32,
        }
    }
}

#[cfg(feature = "glam")]
impl FromRng for UVec2 {
    fn from_rng(v: u64) -> Self {
        UVec2 {
            x: (v & 0xFFFFFFFF) as u32,
            y: (v >> 32) as u32,
        }
    }
}

#[cfg(feature = "glam")]
impl FromRng for IVec3 {
    fn from_rng(v: u64) -> Self {
        // 21 bits per component
        IVec3 {
            x: (v & 0x1FFFFF) as i32,
            y: ((v >> 21) & 0x1FFFFF) as i32,
            z: ((v >> 42) & 0x1FFFFF) as i32,
        }
    }
}

#[cfg(feature = "glam")]
impl FromRng for UVec3 {
    fn from_rng(v: u64) -> Self {
        // 21 bits per component
        UVec3 {
            x: (v & 0x1FFFFF) as u32,
            y: ((v >> 21) & 0x1FFFFF) as u32,
            z: ((v >> 42) & 0x1FFFFF) as u32,
        }
    }
}


#[cfg(feature = "glam")]
impl FromRng for IVec4 {
    fn from_rng(v: u64) -> Self {
        // 16 bits per component
        IVec4 {
            x: (v & 0xFFFF) as i32,
            y: ((v >> 16) & 0xFFFF) as i32,
            z: ((v >> 32) & 0xFFFF) as i32,
            w: ((v >> 48) & 0xFFFF) as i32
        }
    }
}

#[cfg(feature = "glam")]
impl FromRng for UVec4 {
    fn from_rng(v: u64) -> Self {
        // 16 bits per component
        UVec4 {
            x: (v & 0xFFFF) as u32,
            y: ((v >> 16) & 0xFFFF) as u32,
            z: ((v >> 32) & 0xFFFF) as u32,
            w: ((v >> 48) & 0xFFFF) as u32
        }
    }
}

pub trait RangeRng: Sized {
    fn from_range(v: u64, range: Range<Self>) -> Self;
}

impl RangeRng for u64 {
    fn from_range(v: u64, range: Range<Self>) -> Self {
        range.start + (v % (range.end - range.start))
    }
}

impl RangeRng for i64 {
    fn from_range(v: u64, range: Range<Self>) -> Self {
        range.start + (v % (range.end - range.start) as u64) as i64
    }
}

impl RangeRng for usize {
    fn from_range(v: u64, range: Range<Self>) -> Self {
        range.start + (v % (range.end as u64 - range.start as u64)) as usize
    }
}

impl RangeRng for isize {
    fn from_range(v: u64, range: Range<Self>) -> Self {
        range.start + (v % (range.end - range.start) as u64) as isize
    }
}

impl RangeRng for u32 {
    fn from_range(v: u64, range: Range<Self>) -> Self {
        range.start + (v % (range.end as u64 - range.start as u64)) as u32
    }
}

impl RangeRng for i32 {
    fn from_range(v: u64, range: Range<Self>) -> Self {
        range.start + (v % (range.end - range.start) as u64) as i32
    }
}

impl RangeRng for u16 {
    fn from_range(v: u64, range: Range<Self>) -> Self {
        range.start + (v % (range.end as u64 - range.start as u64)) as u16
    }
}

impl RangeRng for i16 {
    fn from_range(v: u64, range: Range<Self>) -> Self {
        range.start + (v % (range.end - range.start) as u64) as i16
    }
}

impl RangeRng for u8 {
    fn from_range(v: u64, range: Range<Self>) -> Self {
        range.start + (v % (range.end as u64 - range.start as u64)) as u8
    }
}

impl RangeRng for i8 {
    fn from_range(v: u64, range: Range<Self>) -> Self {
        range.start + (v % (range.end - range.start) as u64) as i8
    }
}

impl RangeRng for f64 {
    fn from_range(v: u64, range: Range<Self>) -> Self {
        range.start + (v as f64 / u64::MAX as f64) * (range.end - range.start)
    }
}

impl RangeRng for f32 {
    fn from_range(v: u64, range: Range<Self>) -> Self {
        range.start + (v as f64 / u64::MAX as f64) as f32 * (range.end - range.start)
    }
}

#[cfg(feature = "glam")]
impl RangeRng for IVec2 {
    fn from_range(v: u64, range: Range<Self>) -> Self {
        IVec2 {
            x: ((v & 0xFFFFFFFF) % (range.end.x - range.start.x) as u64) as i32,
            y: ((v >> 32) % (range.end.y - range.start.y) as u64) as i32
        }
    }
}

#[cfg(feature = "glam")]
impl RangeRng for UVec2 {
    fn from_range(v: u64, range: Range<Self>) -> Self {
        UVec2 {
            x: ((v & 0xFFFFFFFF) % (range.end.x - range.start.x) as u64) as u32,
            y: ((v >> 32) % (range.end.y - range.start.y) as u64) as u32
        }
    }
}

#[cfg(feature = "glam")]
impl RangeRng for IVec3 {
    fn from_range(v: u64, range: Range<Self>) -> Self {
        // 21 bits per component
        IVec3 {
            x: ((v & 0x1FFFFF) % (range.end.x - range.start.x) as u64) as i32,
            y: (((v >> 21) & 0x1FFFFF) % (range.end.y - range.start.y) as u64) as i32,
            z: (((v >> 42) & 0x1FFFFF) % (range.end.z - range.start.z) as u64) as i32,
        }
    }
}

#[cfg(feature = "glam")]
impl RangeRng for UVec3 {
    fn from_range(v: u64, range: Range<Self>) -> Self {
        // 21 bits per component
        UVec3 {
            x: ((v & 0x1FFFFF) % (range.end.x - range.start.x) as u64) as u32,
            y: (((v >> 21) & 0x1FFFFF) % (range.end.y - range.start.y) as u64) as u32,
            z: (((v >> 42) & 0x1FFFFF) % (range.end.z - range.start.z) as u64) as u32,
        }
    }
}

#[cfg(feature = "glam")]
impl RangeRng for IVec4 {
    fn from_range(v: u64, range: Range<Self>) -> Self {
        // 16 bits per component
        IVec4 {
            x: ((v & 0xFFFF) % (range.end.x - range.start.x) as u64) as i32,
            y: (((v >> 16) & 0xFFFF) % (range.end.y - range.start.y) as u64) as i32,
            z: (((v >> 32) & 0xFFFF) % (range.end.z - range.start.z) as u64) as i32,
            w: (((v >> 48) & 0xFFFF) % (range.end.w - range.start.w) as u64) as i32
        }
    }
}

#[cfg(feature = "glam")]
impl RangeRng for UVec4 {
    fn from_range(v: u64, range: Range<Self>) -> Self {
        // 16 bits per component
        UVec4 {
            x: ((v & 0xFFFF) % (range.end.x - range.start.x) as u64) as u32,
            y: (((v >> 16) & 0xFFFF) % (range.end.y - range.start.y) as u64) as u32,
            z: (((v >> 32) & 0xFFFF) % (range.end.z - range.start.z) as u64) as u32,
            w: (((v >> 48) & 0xFFFF) % (range.end.w - range.start.w) as u64) as u32
        }
    }
}
