
#[cfg(feature = "glam")]
use glam::{IVec2, IVec3, IVec4, UVec2, UVec3, UVec4};

/// A permutation RNG. Indexes into a table instead of hashing a state
/// to "mix" values. Primarily used in procedural texture generation to
/// generate random numbers from 2d, 3d, or 4d vector coordinates.
/// 
/// The lower 256 bytes of the permutation are the same as the upper
/// 256 bytes. This is so we can index the permutation with the sum
/// of a hash byte and a permutation byte without wrapping.
#[derive(Clone)]
pub struct Permutation([u8; 512]);

impl Permutation {
    const DEFAULT: [u8; 512] = {
        let mut result = [0; 512];
        let mut i = 0;
        while i < 512 {
            result[i] = (i & 255) as u8;
            i += 1;
        }
        result
    };

    pub fn new() -> Self {
        Self::with_local_seed()
    }

    /// Construct the permutation with a seed from the thread-local rng state.
    pub fn with_local_seed() -> Self {
        Self::with_seed(crate::seed::from_local())
    }

    /// Construct the permutation with a seed from system state.
    pub fn with_system_seed() -> Self {
        Self::with_seed(crate::seed::from_system())
    }

    /// Construct a new permutation by shuffling the default 
    /// permutation with the provided seed using WyRand. 
    pub fn with_seed(seed: u64) -> Self {
        let mut result = Self::DEFAULT;
        // shuffle lower 256 
        crate::wyrand::WyRand::with_seed(seed)
            .shuffle(&mut result[..256]);
        // copy lower 256 to upper 256
        result[..].copy_within(..256, 256);
        Self(result)
    }

    /// Hash a value, returning a u8 in the range [0,256).
    pub fn mix(&self, v: impl PermMix) -> u8 {
        v.perm_mix(&self.0)
    }

    /// Get a reference to the permutation bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0[..256]
    }

    /// Construct the permutation from 256 bytes.
    pub fn from_bytes(bytes: [u8; 256]) -> Self {
        let mut result = [0; 512];
        result[..256].copy_from_slice(&bytes);
        result[256..].copy_from_slice(&bytes);
        Self(result)
    }

    /// Get a reference to the inner permutation bytes, with the padding.
    pub fn as_bytes_padded(&self) -> &[u8; 512] {
        &self.0
    }

    /// Construct the Permutation from bytes, with the padding.
    pub fn from_bytes_padded(bytes: [u8; 512]) -> Self {
        Self(bytes)
    }
}

/// Mix behavior for a value in the permutation.
pub trait PermMix {
    fn perm_mix(self, perm: &[u8; 512]) -> u8;
}

#[cfg(feature = "glam")]
impl PermMix for IVec2 {
    fn perm_mix(self, perm: &[u8; 512]) -> u8 {
        perm[(self.x & 255) as usize + perm[(self.y & 255) as usize] as usize]
    }
}

#[cfg(feature = "glam")]
impl PermMix for UVec2 {
    fn perm_mix(self, perm: &[u8; 512]) -> u8 {
        perm[(self.x & 255) as usize + perm[(self.y & 255) as usize] as usize]
    }
}

#[cfg(feature = "glam")]
impl PermMix for IVec3 {
    fn perm_mix(self, perm: &[u8; 512]) -> u8 {
        perm[
            (self.x & 255) as usize + perm[
                (self.y & 255) as usize + perm[
                    (self.z & 255) as usize
                ] as usize
            ] as usize
        ]
    }
}

#[cfg(feature = "glam")]
impl PermMix for UVec3 {
    fn perm_mix(self, perm: &[u8; 512]) -> u8 {
        perm[
            (self.x & 255) as usize + perm[
                (self.y & 255) as usize + perm[
                    (self.z & 255) as usize
                ] as usize
            ] as usize
        ]
    }
}

#[cfg(feature = "glam")]
impl PermMix for IVec4 {
    fn perm_mix(self, perm: &[u8; 512]) -> u8 {
        perm[
            (self.x & 255) as usize + perm[
                (self.y & 255) as usize + perm[
                    (self.z & 255) as usize + perm[
                        (self.w & 255) as usize
                    ] as usize
                ] as usize
            ] as usize
        ]
    }
}

#[cfg(feature = "glam")]
impl PermMix for UVec4 {
    fn perm_mix(self, perm: &[u8; 512]) -> u8 {
        perm[
            (self.x & 255) as usize + perm[
                (self.y & 255) as usize + perm[
                    (self.z & 255) as usize + perm[
                        (self.w & 255) as usize
                    ] as usize
                ] as usize
            ] as usize
        ]
    }
}

impl PermMix for u64 {
    fn perm_mix(self, perm: &[u8; 512]) -> u8 {
        u32::perm_mix(self as u32, perm)
    }
}

impl PermMix for i64 {
    fn perm_mix(self, perm: &[u8; 512]) -> u8 {
        u32::perm_mix(self as u32, perm)
    }
}

impl PermMix for usize {
    fn perm_mix(self, perm: &[u8; 512]) -> u8 {
        u32::perm_mix(self as u32, perm)
    }
}

impl PermMix for isize {
    fn perm_mix(self, perm: &[u8; 512]) -> u8 {
        u32::perm_mix(self as u32, perm)
    }
}

impl PermMix for u32 {
    fn perm_mix(self, perm: &[u8; 512]) -> u8 {
        let u = self as usize;
        let a = u & 0xFF;
        let b = (u >> 8) & 0xFF;
        let c = (u >> 16) & 0xFF;
        perm[a + perm[b + perm[c] as usize] as usize]
    }
}

impl PermMix for i32 {
    fn perm_mix(self, perm: &[u8; 512]) -> u8 {
        u32::perm_mix(self as u32, perm)
    }
}

impl PermMix for u16 {
    fn perm_mix(self, perm: &[u8; 512]) -> u8 {
        let [a, b] = self.to_le_bytes();
        perm[b as usize + perm[a as usize] as usize]
    }
}

impl PermMix for i16 {
    fn perm_mix(self, perm: &[u8; 512]) -> u8 {
        let [a , b] = self.to_le_bytes();
        perm[b as usize + perm[a as usize] as usize]
    }
}

impl PermMix for u8 {
    fn perm_mix(self, perm: &[u8; 512]) -> u8 {
        perm[(self & 255) as usize]
    }
}

impl PermMix for i8 {
    fn perm_mix(self, perm: &[u8; 512]) -> u8 {
        perm[self as usize & 255]
    }
}