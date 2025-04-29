use std::hash::{Hash, Hasher};

use image::{DynamicImage, ImageFormat, Rgba, RgbaImage};

use crate::utils::space::s2d::{Delta2D, Direction2D};

pub fn pattern<F>(f: F, n: usize) -> Vec<u32>
where
    F: Fn(usize, usize) -> u32,
{
    let mut result = vec![0; n * n];
    for y in 0..n {
        for x in 0..n {
            result[x + y * n] = f(x, y);
        }
    }
    result
}
pub fn edges_match(p1: &[u32], p2: &[u32], direction: Direction2D, n: usize) -> bool {
    let Delta2D { x: dx, y: dy } = Delta2D::from(direction);

    let (x_start, x_end) = if dx > 0 {
        (dx as usize, n)
    } else {
        (0, (n as isize + dx) as usize)
    };
    let (y_start, y_end) = if dy > 0 {
        (dy as usize, n)
    } else {
        (0, (n as isize + dy) as usize)
    };

    for y in y_start..y_end {
        for x in x_start..x_end {
            let i1 = x + y * n;
            let i2 = (x as isize - dx) as usize + (y as isize - dy) as usize * n;
            if p1[i1] != p2[i2] {
                return false;
            }
        }
    }

    true
}

pub fn rotate(p: &[u32], n: usize) -> Vec<u32> {
    pattern(|x, y| p[n - 1 - y + x * n], n)
}

pub fn reflect(p: &[u32], n: usize) -> Vec<u32> {
    pattern(|x, y| p[n - 1 - x + y * n], n)
}

pub fn hash(p: &[u32]) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    for &val in p {
        val.hash(&mut hasher);
    }
    hasher.finish()
}

pub fn img_to_repr(image: DynamicImage, n: usize) -> u32 {
    // Convert to a concrete RGBA8 buffer
    let rgba = image.to_rgba8();
    let width = rgba.width() as usize;
    let height = rgba.height() as usize;

    // sanity check
    assert!(
        width == n && height == n,
        "img_to_css_bg: expected a {}×{} image, got {}×{}",
        n,
        n,
        width,
        height
    );

    // pick the centre pixel
    // for odd n: this is the exact middle; for even n: this is the lower-right of the 4 central pixels
    let cx = n / 2;
    let cy = n / 2;
    let pixel = rgba.get_pixel(cx as u32, cy as u32).0;
    let (r, g, b, a) = (pixel[0], pixel[1], pixel[2], pixel[3]);

    let repr: u32 = ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
    return repr;
}

pub fn pattern_to_image(pattern: &[u32], n: usize) -> DynamicImage {
    let mut img = RgbaImage::new(n as u32, n as u32);

    for y in 0..n {
        for x in 0..n {
            let color = pattern[x + y * n];
            let rgba = Rgba([
                ((color >> 16) & 0xFF) as u8, // Red
                ((color >> 8) & 0xFF) as u8,  // Green
                (color & 0xFF) as u8,         // Blue
                ((color >> 24) & 0xFF) as u8, // Alpha
            ]);
            img.put_pixel(x as u32, y as u32, rgba);
        }
    }

    DynamicImage::ImageRgba8(img)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_pattern(n: usize) -> Vec<u32> {
        pattern(|x, y| ((x + y * n) as u32) | 0xFF000000, n)
    }

    #[test]
    fn pattern_sanity() {
        let result = pattern(|x, y| (x + y * 3) as u32, 3);
        let expected = vec![
            0, 1, 2, //
            3, 4, 5, //
            6, 7, 8, //
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn rotate_sanity() {
        let original = sample_pattern(3);
        let rotated = rotate(&original, 3);
        let expected = vec![
            0xFF000002, 0xFF000005, 0xFF000008, //
            0xFF000001, 0xFF000004, 0xFF000007, //
            0xFF000000, 0xFF000003, 0xFF000006, //
        ];
        assert_eq!(rotated, expected);
    }

    #[test]
    fn reflect_sanity() {
        let original = sample_pattern(3);
        let reflected = reflect(&original, 3);
        let expected = vec![
            0xFF000002, 0xFF000001, 0xFF000000, //
            0xFF000005, 0xFF000004, 0xFF000003, //
            0xFF000008, 0xFF000007, 0xFF000006, //
        ];
        assert_eq!(reflected, expected);
    }

    #[test]
    fn edges_match_horizontal() {
        let n = 3;

        // Make two patterns where the right edge of p1 matches the left edge of p2
        // p1: right edge = 3, 6, 9
        let p1 = pattern(
            |x, y| {
                if x == n - 1 {
                    (y as u32 + 3) | 0xFF000000
                } else {
                    0
                }
            },
            n,
        );
        let p2 = pattern(
            |x, y| {
                if x == 0 {
                    (y as u32 + 3) | 0xFF000000
                } else {
                    0
                }
            },
            n,
        );

        assert!(edges_match(&p1, &p2, Direction2D::RIGHT, n));
        assert!(edges_match(&p2, &p1, Direction2D::LEFT, n));

        // Mismatch case
        let p3 = pattern(
            |x, y| {
                if x == 0 {
                    (y as u32 + 99) | 0xFF000000
                } else {
                    0
                }
            },
            n,
        );
        assert!(!edges_match(&p1, &p3, Direction2D::RIGHT, n));
    }

    #[test]
    fn edges_match_vertical() {
        let n = 3;

        // Make two patterns where the bottom edge of p1 matches the top edge of p2
        // bottom edge of p1 = 10, 11, 12
        // top edge of p2    = 10, 11, 12
        let p1 = pattern(
            |x, y| {
                if y == n - 1 {
                    (x as u32 + 10) | 0xFF000000
                } else {
                    0
                }
            },
            n,
        );

        let p2 = pattern(
            |x, y| {
                if y == 0 {
                    (x as u32 + 10) | 0xFF000000
                } else {
                    0
                }
            },
            n,
        );

        assert!(edges_match(&p1, &p2, Direction2D::DOWN, n));
        assert!(edges_match(&p2, &p1, Direction2D::UP, n));

        // Mismatch case: p3's top row doesn't match p1's bottom row
        let p3 = pattern(
            |x, y| {
                if y == 0 {
                    (x as u32 + 99) | 0xFF000000
                } else {
                    0
                }
            },
            n,
        );
        assert!(!edges_match(&p1, &p3, Direction2D::DOWN, n));
    }

    #[test]
    fn hash_consistency() {
        let p = sample_pattern(3);
        let h1 = hash(&p);
        let h2 = hash(&p);
        assert_eq!(h1, h2);
    }

    #[test]
    fn pattern_to_image_and_back() {
        let n = 3;
        let pat = sample_pattern(n);
        let img = pattern_to_image(&pat, n);

        // Test that dimensions match
        assert_eq!(img.width(), n as u32);
        assert_eq!(img.height(), n as u32);

        // Reverse convert and check pixel values
        let img_buf = img.to_rgba8();
        for y in 0..n {
            for x in 0..n {
                let i = x + y * n;
                let color = pat[i];
                let pixel = img_buf.get_pixel(x as u32, y as u32);
                let (r, g, b, a) = (pixel[0], pixel[1], pixel[2], pixel[3]);
                assert_eq!(
                    color,
                    ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
                );
            }
        }
    }

    // #[test]
    // fn test_img_to_css_bg() {
    //     let img = DynamicImage::new_rgba8(1, 1);
    //     let css = img_to_repr(img, 1);
    //     assert!(css.starts_with("url('data:image/png;base64,"));
    //     assert!(css.ends_with("')"));
    // }
}
