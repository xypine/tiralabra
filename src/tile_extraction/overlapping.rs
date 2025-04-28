use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    hash::{DefaultHasher, Hash, Hasher},
};

use image::DynamicImage;

use crate::{
    rules::RuleSet2D,
    tile::TileState,
    tile_extraction::helpers::{hash, pattern, reflect, rotate},
    utils::space::{Direction2D, NEIGHBOUR_COUNT_2D},
};

use super::{
    TileExtractor,
    helpers::{edges_match, img_to_css_bg, pattern_to_image},
};

#[derive(Debug)]
pub struct OverlappingBitmapExtractorOptions {
    /// Extracted tiles are n ✖ n pixels
    pub n: usize,
    pub symmetry: usize,
    pub periodic_input: bool,
}

#[derive(Debug)]
pub struct OverlappingBitmapExtractor {
    ruleset: RuleSet2D,
}

impl TileExtractor<NEIGHBOUR_COUNT_2D, Direction2D> for OverlappingBitmapExtractor {
    fn get_rules(&self) -> &RuleSet2D {
        &self.ruleset
    }
}

impl OverlappingBitmapExtractor {
    pub fn new(image: DynamicImage, options: OverlappingBitmapExtractorOptions) -> Self {
        let rgba_image = image.to_rgba8();
        let (width, height) = rgba_image.dimensions();
        let width = width as usize;
        let height = height as usize;

        let buffer = rgba_image
            .pixels()
            .map(|pixel| {
                let [r, g, b, a] = pixel.0;
                ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
            })
            .collect::<Vec<u32>>();

        let (patterns, weights) = Self::extract_patterns(
            buffer,
            width,
            height,
            options.n,
            options.symmetry,
            options.periodic_input,
        );

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
                let pattern_img = pattern_to_image(pattern, options.n);
                let b64 = img_to_css_bg(pattern_img);
                repr.insert(hash, b64);
                tilestate_to_weight.insert(hash, weights[i]);
                hash
            })
            .collect();

        let allowed = Self::build_adjacency_set(&patterns, &tile_states, options.n);

        Self {
            ruleset: RuleSet2D::new(
                BTreeSet::from_iter(tile_states),
                allowed,
                tilestate_to_weight,
                repr,
                BTreeMap::new(),
            ),
        }
    }

    fn build_adjacency_set(
        patterns: &[Vec<u32>],
        hashes: &[TileState],
        n: usize,
    ) -> HashSet<(u64, Direction2D, u64)> {
        let mut adjacency = HashSet::new();

        for (i, p1) in patterns.iter().enumerate() {
            for (j, p2) in patterns.iter().enumerate() {
                for dir_index in 0..NEIGHBOUR_COUNT_2D {
                    let dir = Direction2D::try_from(dir_index).unwrap();
                    if edges_match(p1, p2, dir, n) {
                        adjacency.insert((hashes[i], dir, hashes[j]));
                    }
                }
            }
        }

        adjacency
    }

    fn extract_patterns(
        bitmap: Vec<u32>,
        width: usize,
        height: usize,
        n: usize,
        symmetry: usize,
        periodic_input: bool,
    ) -> (Vec<Vec<u32>>, Vec<usize>) {
        use std::collections::HashMap;

        let xmax = if periodic_input { width } else { width - n + 1 };
        let ymax = if periodic_input {
            height
        } else {
            height - n + 1
        };

        let mut patterns: Vec<Vec<u32>> = Vec::new();
        let mut weights: Vec<usize> = Vec::new();
        let mut pattern_indices: HashMap<u64, usize> = HashMap::new();

        for y in 0..ymax {
            for x in 0..xmax {
                let mut ps: Vec<Vec<u32>> = vec![vec![]; 8];

                // Base pattern
                ps[0] = pattern(
                    |dx, dy| {
                        let sx = (x + dx) % width;
                        let sy = (y + dy) % height;
                        bitmap[sx + sy * width]
                    },
                    n,
                );

                // Generate symmetrical variants
                ps[1] = reflect(&ps[0], n);
                ps[2] = rotate(&ps[0], n);
                ps[3] = reflect(&ps[2], n);
                ps[4] = rotate(&ps[2], n);
                ps[5] = reflect(&ps[4], n);
                ps[6] = rotate(&ps[4], n);
                ps[7] = reflect(&ps[6], n);

                // Store unique patterns and weights
                (0..symmetry).for_each(|k| {
                    let p = &ps[k];
                    let h = hash(p);
                    if let Some(&index) = pattern_indices.get(&h) {
                        weights[index] += 1;
                    } else {
                        let index = weights.len();
                        pattern_indices.insert(h, index);
                        patterns.push(p.clone());
                        weights.push(1);
                    }
                });
            }
        }

        (patterns, weights)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, RgbaImage};

    fn simple_image(size: u32, color_fn: impl Fn(u32, u32) -> [u8; 4]) -> DynamicImage {
        let mut img = RgbaImage::new(size, size);
        for y in 0..size {
            for x in 0..size {
                img.put_pixel(x, y, image::Rgba(color_fn(x, y)));
            }
        }
        DynamicImage::ImageRgba8(img)
    }

    #[test]
    fn test_extractor_basic_2x2() {
        let img = simple_image(3, |x, y| {
            if (x + y) % 2 == 0 {
                [255, 0, 0, 255] // red
            } else {
                [0, 255, 0, 255] // green
            }
        });

        let options = OverlappingBitmapExtractorOptions {
            n: 2,
            symmetry: 1,
            periodic_input: false,
        };

        let extractor = OverlappingBitmapExtractor::new(img, options);

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

    #[test]
    fn test_extractor_symmetry_effect() {
        let img = simple_image(3, |x, y| [(x * 30) as u8, (y * 30) as u8, 0, 255]);

        let options_nosym = OverlappingBitmapExtractorOptions {
            n: 2,
            symmetry: 1,
            periodic_input: false,
        };

        let options_sym = OverlappingBitmapExtractorOptions {
            n: 2,
            symmetry: 8,
            periodic_input: false,
        };

        let extractor_no_sym = OverlappingBitmapExtractor::new(img.clone(), options_nosym);
        let extractor_sym = OverlappingBitmapExtractor::new(img, options_sym);

        let count_no_sym = extractor_no_sym.get_rules().possible.len();
        let count_sym = extractor_sym.get_rules().possible.len();

        assert!(
            count_sym >= count_no_sym,
            "Symmetric pattern extraction should find more or equal patterns"
        );
    }

    #[test]
    fn test_css_representation_format() {
        let img = simple_image(2, |x, y| {
            let val = (x + y) * 50;
            [val as u8, val as u8, val as u8, 255]
        });

        let options = OverlappingBitmapExtractorOptions {
            n: 2,
            symmetry: 1,
            periodic_input: false,
        };

        let extractor = OverlappingBitmapExtractor::new(img, options);
        let reprs = extractor.get_rules().state_representations.clone();

        for (_, css) in reprs.iter() {
            assert!(
                css.starts_with("url('data:image/png;base64,"),
                "CSS background should start with base64 header"
            );
            assert!(
                css.ends_with("')"),
                "CSS background should end with closing quotes and parentheses"
            );
        }
    }
}
