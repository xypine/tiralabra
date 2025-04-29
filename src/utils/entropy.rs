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
