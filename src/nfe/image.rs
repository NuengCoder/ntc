//! Inline image preview for the NFE TUI (half-block color cells).
//!
//! Each terminal cell shows two image pixels: the foreground color draws
//! the upper half-block (`▀`) over the background lower pixel, so one
//! text row carries two rows of image. This works in any terminal
//! (no Kitty/Sixel/iTerm protocols needed).
//!
//! Decoding uses the `image` crate behind the `image2doc` feature (same
//! gating as `qr`/`image` shell commands); without the feature only
//! extension detection is available.

use std::path::Path;

/// Extensions previewable as images (decode support comes from the
/// `image` crate: raster formats render the frame, `gif` its first frame).
pub const IMAGE_EXTS: &[&str] = &["jpg", "jpeg", "png", "bmp", "ico", "gif", "webp"];

/// Upper bound on decoded sample pixels (cols × rows × 2); larger
/// requests are rejected instead of allocating a giant buffer.
pub const MAX_SAMPLE_PIXELS: usize = 4_000_000;

pub fn is_image_file(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| IMAGE_EXTS.iter().any(|x| x.eq_ignore_ascii_case(e)))
        .unwrap_or(false)
}

/// One preview cell: `ch` is `'▀'` (fg = top pixel, bg = bottom pixel)
/// or `' '` when both pixels match (solid bg block reads cleaner).
/// Sextant mode reuses the same struct with 2x3 micro-block glyphs.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ImageCell {
    pub ch: char,
    pub fg: (u8, u8, u8),
    pub bg: (u8, u8, u8),
}

/// Image preview density. Sextants pack 6 sub-pixels per cell (3x the
/// vertical detail of half-blocks) and are the default; Half remains
/// for terminals without sextant glyphs (tofu boxes).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ImageMode {
    #[default]
    Sextant,
    Half,
}

/// Sub-cell mask (bit d-1 = sextant digit d) to glyph. Digits run
/// row-major (1 2 / 3 4 / 5 6) per the Unicode proposal — confirmed by
/// the omissions: {135} and {246} are the pre-existing half blocks
/// U+258C/U+2590, {123456} is U+2588, so all three are reused here.
/// Table generated from UnicodeData 16.0 — do not hand-edit.
pub const SEXTANT_GLYPHS: [char; 64] = [
    ' ',         // 0b000000 blank
    '\u{1fb00}', // 0b000001 1
    '\u{1fb01}', // 0b000010 2
    '\u{1fb02}', // 0b000011 12
    '\u{1fb03}', // 0b000100 3
    '\u{1fb04}', // 0b000101 13
    '\u{1fb05}', // 0b000110 23
    '\u{1fb06}', // 0b000111 123
    '\u{1fb07}', // 0b001000 4
    '\u{1fb08}', // 0b001001 14
    '\u{1fb09}', // 0b001010 24
    '\u{1fb0a}', // 0b001011 124
    '\u{1fb0b}', // 0b001100 34
    '\u{1fb0c}', // 0b001101 134
    '\u{1fb0d}', // 0b001110 234
    '\u{1fb0e}', // 0b001111 1234
    '\u{1fb0f}', // 0b010000 5
    '\u{1fb10}', // 0b010001 15
    '\u{1fb11}', // 0b010010 25
    '\u{1fb12}', // 0b010011 125
    '\u{1fb13}', // 0b010100 35
    '\u{258c}',  // 0b010101 left half block (no SEXTANT-135)
    '\u{1fb14}', // 0b010110 235
    '\u{1fb15}', // 0b010111 1235
    '\u{1fb16}', // 0b011000 45
    '\u{1fb17}', // 0b011001 145
    '\u{1fb18}', // 0b011010 245
    '\u{1fb19}', // 0b011011 1245
    '\u{1fb1a}', // 0b011100 345
    '\u{1fb1b}', // 0b011101 1345
    '\u{1fb1c}', // 0b011110 2345
    '\u{1fb1d}', // 0b011111 12345
    '\u{1fb1e}', // 0b100000 6
    '\u{1fb1f}', // 0b100001 16
    '\u{1fb20}', // 0b100010 26
    '\u{1fb21}', // 0b100011 126
    '\u{1fb22}', // 0b100100 36
    '\u{1fb23}', // 0b100101 136
    '\u{1fb24}', // 0b100110 236
    '\u{1fb25}', // 0b100111 1236
    '\u{1fb26}', // 0b101000 46
    '\u{1fb27}', // 0b101001 146
    '\u{2590}',  // 0b101010 right half block (no SEXTANT-246)
    '\u{1fb28}', // 0b101011 1246
    '\u{1fb29}', // 0b101100 346
    '\u{1fb2a}', // 0b101101 1346
    '\u{1fb2b}', // 0b101110 2346
    '\u{1fb2c}', // 0b101111 12346
    '\u{1fb2d}', // 0b110000 56
    '\u{1fb2e}', // 0b110001 156
    '\u{1fb2f}', // 0b110010 256
    '\u{1fb30}', // 0b110011 1256
    '\u{1fb31}', // 0b110100 356
    '\u{1fb32}', // 0b110101 1356
    '\u{1fb33}', // 0b110110 2356
    '\u{1fb34}', // 0b110111 12356
    '\u{1fb35}', // 0b111000 456
    '\u{1fb36}', // 0b111001 1456
    '\u{1fb37}', // 0b111010 2456
    '\u{1fb38}', // 0b111011 12456
    '\u{1fb39}', // 0b111100 3456
    '\u{1fb3a}', // 0b111101 13456
    '\u{1fb3b}', // 0b111110 23456
    '\u{2588}',  // 0b111111 full block (no all-on sextant)
];

/// A decoded preview grid plus the source dimensions for the header line.
#[derive(Debug, Clone, PartialEq)]
pub struct ImagePreview {
    pub rows: Vec<Vec<ImageCell>>,
    pub orig_w: u32,
    pub orig_h: u32,
}

impl ImagePreview {
    pub fn cells_wide(&self) -> usize {
        self.rows.first().map(|r| r.len()).unwrap_or(0)
    }

    pub fn cells_high(&self) -> usize {
        self.rows.len()
    }
}

/// Geometry of a blitted pixel image in cells: centering padding
/// plus the shown size. Used to draw the surrounding border box.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BlitGeom {
    pub pad_x: usize,
    pub pad_y: usize,
    pub w: usize,
    pub h: usize,
}

/// Grid dimensions (cells) for a `cols × rows` panel: full width, full
/// height — one cell already packs two vertical pixels.
pub fn grid_for_panel(cols: usize, rows: usize) -> (usize, usize) {
    (cols.max(1), rows.max(1))
}

/// Decode `path` and downscale to a `cols × rows` half-block grid.
#[cfg(feature = "image2doc")]
pub fn load_halfblock(path: &Path, cols: usize, rows: usize) -> anyhow::Result<ImagePreview> {
    let (gw, gh) = grid_for_panel(cols, rows);
    if gw.saturating_mul(gh).saturating_mul(2) > MAX_SAMPLE_PIXELS {
        anyhow::bail!("image preview too large for panel");
    }
    let rgb = decode_rgb(path)?;
    let sample_h = (gh as u32).saturating_mul(2).max(1);
    let small = resize_rgb(&rgb.pixels, gw as u32, sample_h);

    let mut out: Vec<Vec<ImageCell>> = Vec::with_capacity(gh);
    for cy in 0..gh {
        let mut row = Vec::with_capacity(gw);
        for cx in 0..gw {
            let top = sample_px(&small, cx as u32, (cy as u32) * 2);
            let bot = sample_px(&small, cx as u32, (cy as u32) * 2 + 1);
            if top == bot {
                row.push(ImageCell {
                    ch: ' ',
                    fg: top,
                    bg: top,
                });
            } else {
                row.push(ImageCell {
                    ch: '▀',
                    fg: top,
                    bg: bot,
                });
            }
        }
        out.push(row);
    }
    Ok(ImagePreview {
        rows: out,
        orig_w: rgb.orig_w,
        orig_h: rgb.orig_h,
    })
}

#[cfg(feature = "image2doc")]
fn sample_px(img: &image::RgbImage, x: u32, y: u32) -> (u8, u8, u8) {
    let (w, h) = (img.width(), img.height());
    let p = img.get_pixel(x.min(w.saturating_sub(1)), y.min(h.saturating_sub(1)));
    (p[0], p[1], p[2])
}

/// Rec. 601 luma, the brightness axis for quantization.
pub fn luminance(px: (u8, u8, u8)) -> f32 {
    0.299 * px.0 as f32 + 0.587 * px.1 as f32 + 0.114 * px.2 as f32
}

/// 4x4 Bayer threshold matrix (row-major, 0..15).
const BAYER4: [f32; 16] = [
    0., 8., 2., 10., 12., 4., 14., 6., 3., 11., 1., 9., 15., 7., 13., 5.,
];

/// Ordered-dither bias for absolute pixel coords, in [-1.0, 1.0].
/// Multiplied by the dither amplitude before the luminance split.
pub fn bayer_bias(x: u32, y: u32) -> f32 {
    BAYER4[((y % 4) * 4 + (x % 4)) as usize] / 8.0 - 1.0
}

/// Dither amplitude in luminance levels: strong enough to break
/// gradient banding, weak enough that solid 0/255 pixels never flip.
pub const DITHER_AMP: f32 = 10.0;

fn mean_color(samples: &[(u8, u8, u8)]) -> (u8, u8, u8) {
    if samples.is_empty() {
        return (0, 0, 0);
    }
    let n = samples.len() as f32;
    (
        (samples.iter().map(|p| p.0 as f32).sum::<f32>() / n) as u8,
        (samples.iter().map(|p| p.1 as f32).sum::<f32>() / n) as u8,
        (samples.iter().map(|p| p.2 as f32).sum::<f32>() / n) as u8,
    )
}

/// Quantize one 2x3 sextant cell: split the 6 samples by dithered
/// luminance around the cell mean; fg = bright mean, bg = dark mean.
/// `(ax, ay)` is the absolute top-left sample coordinate (dither phase).
/// Samples are row-major: index = row * 2 + col (digit = index + 1).
pub fn sextant_cell(samples: [(u8, u8, u8); 6], ax: u32, ay: u32) -> ImageCell {
    let mean = samples.iter().map(|p| luminance(*p)).sum::<f32>() / 6.0;
    let mut mask: u8 = 0;
    let mut hi: Vec<(u8, u8, u8)> = Vec::with_capacity(6);
    let mut lo: Vec<(u8, u8, u8)> = Vec::with_capacity(6);
    for (i, px) in samples.iter().enumerate() {
        // Sub-cell (col,row) -> absolute coords for the dither phase.
        let sx = ax + (i % 2) as u32;
        let sy = ay + (i / 2) as u32;
        let on = luminance(*px) + bayer_bias(sx, sy) * DITHER_AMP >= mean;
        if on {
            mask |= 1 << i;
            hi.push(*px);
        } else {
            lo.push(*px);
        }
    }
    if hi.is_empty() || lo.is_empty() {
        let solid = mean_color(&samples);
        return ImageCell {
            ch: ' ',
            fg: solid,
            bg: solid,
        };
    }
    let fg = mean_color(&hi);
    let bg = mean_color(&lo);
    // Dither can split an exactly-solid fill into a patterned mask;
    // collapse it back to a space cell (identical pixels, less noise).
    if fg == bg {
        return ImageCell { ch: ' ', fg, bg };
    }
    ImageCell {
        ch: SEXTANT_GLYPHS[mask as usize],
        fg,
        bg,
    }
}

/// Decode `path` and downscale to a `cols × rows` sextant grid
/// (2x3 micro-pixels per cell: 3x the vertical detail of half-blocks).
#[cfg(feature = "image2doc")]
pub fn load_sextants(path: &Path, cols: usize, rows: usize) -> anyhow::Result<ImagePreview> {
    let (gw, gh) = grid_for_panel(cols, rows);
    if gw.saturating_mul(2).saturating_mul(gh.saturating_mul(3)) > MAX_SAMPLE_PIXELS {
        anyhow::bail!("image preview too large for panel");
    }
    let rgb = decode_rgb(path)?;
    let small = resize_rgb(
        &rgb.pixels,
        (gw as u32).saturating_mul(2).max(1),
        (gh as u32).saturating_mul(3).max(1),
    );
    let mut out: Vec<Vec<ImageCell>> = Vec::with_capacity(gh);
    for cy in 0..gh {
        let mut row = Vec::with_capacity(gw);
        for cx in 0..gw {
            let ax = (cx as u32) * 2;
            let ay = (cy as u32) * 3;
            let mut samples = [(0u8, 0u8, 0u8); 6];
            for r in 0..3 {
                for c in 0..2 {
                    samples[r * 2 + c] = sample_px(&small, ax + c as u32, ay + r as u32);
                }
            }
            row.push(sextant_cell(samples, ax, ay));
        }
        out.push(row);
    }
    Ok(ImagePreview {
        rows: out,
        orig_w: rgb.orig_w,
        orig_h: rgb.orig_h,
    })
}

/// Decoded source image kept for re-gridding (viewer zoom/resize)
/// without re-reading the file.
#[cfg(feature = "image2doc")]
#[derive(Clone)]
pub struct DecodedImage {
    pub pixels: image::RgbImage,
    pub orig_w: u32,
    pub orig_h: u32,
}

#[cfg(feature = "image2doc")]
pub(crate) struct RawDecoded {
    pub pixels: image::RgbImage,
    pub orig_w: u32,
    pub orig_h: u32,
}

#[cfg(feature = "image2doc")]
fn decode_rgb(path: &Path) -> anyhow::Result<RawDecoded> {
    use image::GenericImageView;
    let img = image::open(path).map_err(|e| anyhow::anyhow!("{}", e))?;
    let (orig_w, orig_h) = img.dimensions();
    Ok(RawDecoded {
        pixels: img.to_rgb8(),
        orig_w,
        orig_h,
    })
}

/// Decode once for the viewer cache (no downscale yet).
#[cfg(feature = "image2doc")]
pub fn decode_for_viewer(path: &Path) -> anyhow::Result<DecodedImage> {
    let raw = decode_rgb(path)?;
    // Refuse absurd allocations before they happen.
    let px = raw.orig_w as u64 * raw.orig_h as u64;
    if px > 100_000_000 {
        anyhow::bail!("image too large (>100MP)");
    }
    Ok(DecodedImage {
        pixels: raw.pixels,
        orig_w: raw.orig_w,
        orig_h: raw.orig_h,
    })
}

/// Grid a cached decode at an explicit sample size (viewer zoom/pan).
/// `(ox, oy)` is the top-left sample offset into the resized image.
#[cfg(feature = "image2doc")]
pub fn grid_sextants_from(
    src: &DecodedImage,
    samp_w: u32,
    samp_h: u32,
    ox: u32,
    oy: u32,
    cols: usize,
    rows: usize,
) -> ImagePreview {
    let small = resize_rgb(&src.pixels, samp_w.max(1), samp_h.max(1));
    let (w, h) = (small.width(), small.height());
    let mut out: Vec<Vec<ImageCell>> = Vec::with_capacity(rows);
    for cy in 0..rows {
        let mut row = Vec::with_capacity(cols);
        for cx in 0..cols {
            let ax = ox + (cx as u32) * 2;
            let ay = oy + (cy as u32) * 3;
            let mut samples = [(0u8, 0u8, 0u8); 6];
            for r in 0..3 {
                for c in 0..2 {
                    let x = (ax + c as u32).min(w.saturating_sub(1));
                    let y = (ay + r as u32).min(h.saturating_sub(1));
                    let p = small.get_pixel(x, y);
                    samples[r * 2 + c] = (p[0], p[1], p[2]);
                }
            }
            row.push(sextant_cell(samples, ax, ay));
        }
        out.push(row);
    }
    ImagePreview {
        rows: out,
        orig_w: src.orig_w,
        orig_h: src.orig_h,
    }
}

/// Grid a cached decode at an explicit sample size (viewer zoom/pan).
/// Half-block variant: 1 wide x 2 tall samples per cell.
#[cfg(feature = "image2doc")]
pub fn grid_half_from(
    src: &DecodedImage,
    samp_w: u32,
    samp_h: u32,
    ox: u32,
    oy: u32,
    cols: usize,
    rows: usize,
) -> ImagePreview {
    let small = resize_rgb(&src.pixels, samp_w.max(1), samp_h.max(1));
    let mut out: Vec<Vec<ImageCell>> = Vec::with_capacity(rows);
    for cy in 0..rows {
        let mut row = Vec::with_capacity(cols);
        for cx in 0..cols {
            let top = sample_px(&small, ox + cx as u32, oy + cy as u32 * 2);
            let bot = sample_px(&small, ox + cx as u32, oy + cy as u32 * 2 + 1);
            if top == bot {
                row.push(ImageCell {
                    ch: ' ',
                    fg: top,
                    bg: top,
                });
            } else {
                row.push(ImageCell {
                    ch: '▀',
                    fg: top,
                    bg: bot,
                });
            }
        }
        out.push(row);
    }
    ImagePreview {
        rows: out,
        orig_w: src.orig_w,
        orig_h: src.orig_h,
    }
}

#[cfg(feature = "image2doc")]
fn resize_rgb(img: &image::RgbImage, w: u32, h: u32) -> image::RgbImage {
    // Lanczos3 preserves edge contrast through the downscale, which
    // matters disproportionately at micro-block sizes (Triangle blurs
    // one-cell features into mush).
    image::imageops::resize(
        img,
        w.max(1),
        h.max(1),
        image::imageops::FilterType::Lanczos3,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ext_detection() {
        for ext in ["jpg", "JPG", "jpeg", "png", "bmp", "ico", "gif", "webp"] {
            assert!(is_image_file(Path::new(&format!("a.{}", ext))), "{}", ext);
        }
        for ext in ["txt", "rs", "mq5", "mp3", "svg", ""] {
            assert!(!is_image_file(Path::new(&format!("a.{}", ext))), "{}", ext);
        }
        assert!(!is_image_file(Path::new("noext")));
    }

    #[test]
    fn grid_dims_follow_panel() {
        assert_eq!(grid_for_panel(40, 20), (40, 20));
        assert_eq!(grid_for_panel(0, 0), (1, 1));
    }

    #[test]
    fn sextant_table_pins_verified_mapping() {
        assert_eq!(SEXTANT_GLYPHS.len(), 64);
        assert_eq!(SEXTANT_GLYPHS[0], ' ');
        assert_eq!(SEXTANT_GLYPHS[63], '█');
        // Spot checks against UnicodeData names (verified 16.0):
        // bit d-1 = sextant digit d, row-major (1 2 / 3 4 / 5 6).
        assert_eq!(SEXTANT_GLYPHS[0b000001], '\u{1fb00}'); // SEXTANT-1
        assert_eq!(SEXTANT_GLYPHS[0b000011], '\u{1fb02}'); // SEXTANT-12
        assert_eq!(SEXTANT_GLYPHS[0b001111], '\u{1fb0e}'); // SEXTANT-1234
        assert_eq!(SEXTANT_GLYPHS[0b010101], '▌'); // left half (no SEXTANT-135)
        assert_eq!(SEXTANT_GLYPHS[0b101010], '▐'); // right half (no SEXTANT-246)
    }

    #[test]
    fn sextant_cell_geometry_and_colors() {
        let black = (0u8, 0u8, 0u8);
        let white = (255u8, 255u8, 255u8);
        // Only the top-left sub-cell bright (digit 1 -> bit 0).
        // Extreme values: dither (+/-10) cannot flip the split.
        let mut s = [black; 6];
        s[0] = white;
        let cell = sextant_cell(s, 0, 0);
        assert_eq!(cell.ch, '\u{1fb00}');
        assert_eq!(cell.fg, white);
        assert_eq!(cell.bg, black);
        // Bottom row bright (digits 5,6 -> bits 4,5).
        let mut s = [black; 6];
        s[4] = white;
        s[5] = white;
        let cell = sextant_cell(s, 0, 0);
        assert_eq!(cell.ch, SEXTANT_GLYPHS[0b110000]);
        // Solid color collapses to a space cell.
        let cell = sextant_cell([white; 6], 0, 0);
        assert_eq!(cell.ch, ' ');
        assert_eq!((cell.fg, cell.bg), (white, white));
    }

    #[test]
    fn dither_is_deterministic() {
        let px = [
            (30, 30, 30),
            (200, 200, 200),
            (90, 90, 90),
            (120, 120, 120),
            (60, 60, 60),
            (150, 150, 150),
        ];
        assert_eq!(sextant_cell(px, 7, 3), sextant_cell(px, 7, 3));
        // Phase matters (different absolute coords may quantize
        // borderline cells differently) but stays a valid glyph.
        let c = sextant_cell(px, 1, 2);
        assert!(SEXTANT_GLYPHS.contains(&c.ch));
    }

    #[cfg(feature = "image2doc")]
    #[test]
    fn halfblock_shape_and_aspect() {
        use image::RgbImage;
        // Wide red image: grid keeps panel shape, solid cells use space.
        let img = RgbImage::from_pixel(200, 100, image::Rgb([255, 0, 0]));
        let dir = std::env::temp_dir().join(format!("ntc_img_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("red.png");
        img.save(&path).unwrap();
        let prev = load_halfblock(&path, 40, 20).unwrap();
        assert_eq!((prev.orig_w, prev.orig_h), (200, 100));
        assert_eq!(prev.cells_high(), 20);
        assert_eq!(prev.cells_wide(), 40);
        assert!(
            prev.rows.iter().flatten().all(|c| c.ch == ' '),
            "solid image must use space cells"
        );
        // Two-tone split: top red / bottom blue rows use half-blocks.
        let mut img2 = RgbImage::new(10, 10);
        for y in 0..10 {
            for x in 0..10 {
                let c = if y < 5 {
                    image::Rgb([255, 0, 0])
                } else {
                    image::Rgb([0, 0, 255])
                };
                img2.put_pixel(x, y, c);
            }
        }
        let path2 = dir.join("split.png");
        img2.save(&path2).unwrap();
        let prev2 = load_halfblock(&path2, 10, 5).unwrap();
        assert!(prev2.rows.iter().flatten().any(|c| c.ch == '▀'));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[cfg(feature = "image2doc")]
    #[test]
    fn corrupt_file_errors() {
        let dir = std::env::temp_dir().join(format!("ntc_img_bad_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("bad.png");
        std::fs::write(&path, b"not an image at all").unwrap();
        assert!(load_halfblock(&path, 40, 20).is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
