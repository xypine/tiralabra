use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    hash::{DefaultHasher, Hash, Hasher},
};

use crate::{
    rules::RuleSet1D,
    tile::TileState,
    utils::space::s1d::{Direction1D, NEIGHBOUR_COUNT_1D},
};

use super::TileExtractor;

#[derive(Debug)]
pub struct OverlappingTextExtractorOptions {
    /// Extracted tokens are n characters long
    pub n: usize,
}

#[derive(Debug)]
pub struct OverlappingTextExtractor {
    ruleset: RuleSet1D,
}

impl TileExtractor<NEIGHBOUR_COUNT_1D, Direction1D> for OverlappingTextExtractor {
    fn get_rules(&self) -> &RuleSet1D {
        &self.ruleset
    }
}

impl OverlappingTextExtractor {
    pub fn new(source: String, options: OverlappingTextExtractorOptions) -> Self {
        let width = source.len();

        let buffer = source.chars().collect::<Vec<char>>();

        let (patterns, weights) = Self::extract_patterns(buffer, width, options.n);

        let mut repr = HashMap::new();
        let mut tilestate_to_pattern = HashMap::new();
        let mut tilestate_to_weight = HashMap::new();
        let tile_states: Vec<TileState> = patterns
            .iter()
            .enumerate()
            .map(|(i, pattern)| {
                let mut hasher = DefaultHasher::new();
                pattern.hash(&mut hasher);
                let hash = hasher.finish();
                tilestate_to_pattern.insert(hash, pattern.clone());
                repr.insert(hash, pattern.iter().collect());
                tilestate_to_weight.insert(hash, weights[i]);
                hash
            })
            .collect();

        let allowed = Self::build_adjacency_set(&patterns, &tile_states, options.n);

        Self {
            ruleset: RuleSet1D::new(
                BTreeSet::from_iter(tile_states),
                allowed,
                tilestate_to_weight,
                repr,
                BTreeMap::new(),
            ),
        }
    }

    fn build_adjacency_set(
        patterns: &[Vec<char>],
        hashes: &[TileState],
        n: usize,
    ) -> HashSet<(u64, Direction1D, u64)> {
        let mut adjacency = HashSet::new();

        for (i, p1) in patterns.iter().enumerate() {
            for (j, p2) in patterns.iter().enumerate() {
                for dir_index in 0..NEIGHBOUR_COUNT_1D {
                    let dir = Direction1D::try_from(dir_index).unwrap();
                    if edges_match(p1, p2, dir, n) {
                        adjacency.insert((hashes[i], dir, hashes[j]));
                    }
                }
            }
        }

        adjacency
    }

    fn extract_patterns(bitmap: Vec<char>, width: usize, n: usize) -> (Vec<Vec<char>>, Vec<usize>) {
        use std::collections::HashMap;

        let xmax = width - n + 1;

        let mut patterns: Vec<Vec<char>> = Vec::new();
        let mut weights: Vec<usize> = Vec::new();
        let mut pattern_indices: HashMap<u64, usize> = HashMap::new();

        for x in 0..xmax {
            // Base pattern
            let p = pattern(
                |dx| {
                    let sx = (x + dx) % width;
                    bitmap[sx]
                },
                n,
            );

            let h = hash(&p);
            if let Some(&index) = pattern_indices.get(&h) {
                weights[index] += 1;
            } else {
                let index = weights.len();
                pattern_indices.insert(h, index);
                patterns.push(p.clone());
                weights.push(1);
            }
        }

        (patterns, weights)
    }
}

pub fn pattern<F>(f: F, n: usize) -> Vec<char>
where
    F: Fn(usize) -> char,
{
    let mut result = vec![' '; n];
    for x in 0..n {
        result[x] = f(x);
    }
    result
}

pub fn hash(p: &[char]) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    for &val in p {
        val.hash(&mut hasher);
    }
    hasher.finish()
}

pub fn edges_match(p1: &[char], p2: &[char], direction: Direction1D, n: usize) -> bool {
    match direction {
        Direction1D::RIGHT => p1[p1.len() - n..] == p2[..n],
        Direction1D::LEFT => p1[..n] == p2[p2.len() - n..],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extractor_basic() {
        let source = "Let's go fishing tomorrow, I know a good spot the rascals haven't found yet. At least I hope so...".to_owned();

        let options = OverlappingTextExtractorOptions { n: 3 };

        let extractor = OverlappingTextExtractor::new(source, options);

        let ruleset = extractor.get_rules();
        let tile_count = ruleset.possible.len();

        assert!(tile_count > 0, "Expected at least one tile to be extracted");
        assert!(ruleset.state_representations.len() == tile_count);
        assert!(
            ruleset
                .allowed
                .iter()
                .all(|(a, _, b)| ruleset.possible.contains(a) && ruleset.possible.contains(b)),
            "Adjacency rules must refer to known tile states"
        );
    }
}
