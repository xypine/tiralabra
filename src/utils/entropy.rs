//! Utilities for handling tile "entropy"

use std::cmp::Ordering;

use serde::{Deserialize, Serialize};
use tsify_next::Tsify;

use crate::utils::space::s2d::Location2D;

use super::space::s1d::Location1D;

// comparisons can fail for floating point numbers if one of the entropies is "NaN"
// praise the IEEC
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Entropy(pub f64);

impl Eq for Entropy {} // Safe because we guarantee consistent Ord

impl Ord for Entropy {
    fn cmp(&self, other: &Self) -> Ordering {
        other.partial_cmp(self).unwrap_or(Ordering::Greater) // Treat NaN as the highest value
    }
}

#[derive(Debug, Eq, Clone, Copy, PartialEq, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct EntropyHeapEntry {
    pub location: Location2D,
    pub entropy: Entropy,
    pub version: usize,
}

impl PartialOrd for EntropyHeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.entropy.cmp(&other.entropy))
    }
}

impl Ord for EntropyHeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.entropy.cmp(&other.entropy)
    }
}

#[derive(Debug, Eq, Clone, Copy, PartialEq, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct EntropyHeapEntry1D {
    pub location: Location1D,
    pub entropy: Entropy,
    pub version: usize,
}

impl PartialOrd for EntropyHeapEntry1D {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.entropy.cmp(&other.entropy))
    }
}

impl Ord for EntropyHeapEntry1D {
    fn cmp(&self, other: &Self) -> Ordering {
        self.entropy.cmp(&other.entropy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entropy_cmp_normal_values() {
        let a = Entropy(1.0);
        let b = Entropy(2.0);

        // Note: entropy order is reversed — lower entropy is "greater"
        assert_eq!(a.cmp(&b), std::cmp::Ordering::Greater);
        assert_eq!(b.cmp(&a), std::cmp::Ordering::Less);
        assert_eq!(a.cmp(&a), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_entropy_cmp_with_nan() {
        let a = Entropy(1.0);
        let nan = Entropy(f64::NAN);

        assert_eq!(nan.cmp(&a), std::cmp::Ordering::Greater);
        assert_eq!(a.cmp(&nan), std::cmp::Ordering::Greater);
    }

    #[test]
    fn test_entropy_heap_entry_cmp() {
        let a = EntropyHeapEntry {
            location: Location2D { x: 0, y: 0 },
            entropy: Entropy(1.0),
            version: 1,
        };
        let b = EntropyHeapEntry {
            location: Location2D { x: 1, y: 1 },
            entropy: Entropy(2.0),
            version: 2,
        };

        assert_eq!(a.cmp(&b), std::cmp::Ordering::Greater);
        assert_eq!(b.cmp(&a), std::cmp::Ordering::Less);
    }

    #[test]
    fn test_entropy_heap_entry1d_cmp() {
        let a = EntropyHeapEntry1D {
            location: Location1D { x: 0 },
            entropy: Entropy(1.0),
            version: 1,
        };
        let b = EntropyHeapEntry1D {
            location: Location1D { x: 1 },
            entropy: Entropy(2.0),
            version: 2,
        };

        assert_eq!(a.cmp(&b), std::cmp::Ordering::Greater);
        assert_eq!(b.cmp(&a), std::cmp::Ordering::Less);
    }
}
