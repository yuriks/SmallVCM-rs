use std::rand::{Rng, SeedableRng};

/// This is the XorShift128+ generator.
/// Based on the C code provided at http://xorshift.di.unimi.it/

pub struct XorShift128Plus {
    s0: u64, s1: u64
}

impl XorShift128Plus {
    fn new_unseeded() -> XorShift128Plus {
        XorShift128Plus {
            // Numbers generated by random.org
            s0: 0x4587ba0ead01370f,
            s1: 0xdd817882dc98c4aa,
        }
    }
}

impl Rng for XorShift128Plus {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let mut s1 = self.s0;
        let s0 = self.s1;
        self.s0 = s0;
        s1 ^= s1 << 23;
        self.s1 = s1 ^ s0 ^ (s1 >> 17) ^ (s0 >> 26);
        return self.s1 + s0
    }
}

impl SeedableRng<[u64, ..2]> for XorShift128Plus {
    #[inline]
    fn reseed(&mut self, seed: [u64, ..2]) {
        self.s0 = seed[0];
        self.s1 = seed[1];
    }

    #[inline]
    fn from_seed(seed: [u64, ..2]) -> XorShift128Plus {
        XorShift128Plus {
            s0: seed[0],
            s1: seed[1],
        }
    }
}
