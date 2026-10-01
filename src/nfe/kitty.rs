//! Kitty graphics protocol output for the NFE image viewer/panel.
//!
//! Covers kitty, Ghostty (`xterm-ghostty`), WezTerm and iTerm2 (which
//! documents Kitty-protocol support). Always quiet (`q=2`) so the
//! terminal never replies with acks that would pollute input.
//!
//! Framing (spec): `ESC _ G <ctrl> ; <base64 payload> ESC \`, payload
//! chunked to <= 4096 bytes (`m=1` more, `m=0` last; every chunk but
//! the last a multiple of 4 — 4096 always is). First chunk carries
//! `a=T,f=100,q=2,i=<id>,c=<cols>,r=<rows>` (transmit PNG + display
//! in a `cols × rows` cell rect at the final cursor position).
//! Cleanup: `ESC _ G a=d,d=I,i=<id>,q=2 ESC \` deletes the image and
//! frees its data before every re-transmit and on viewer close.

/// Fixed image id for NFE blits (single image at a time).
pub const KITTY_IMAGE_ID: u32 = 7;

/// Max base64 payload bytes per escape sequence (spec limit).
pub const CHUNK_SIZE: usize = 4096;

/// Encode RGB pixels as PNG bytes (transmission payload).
#[cfg(feature = "image2doc")]
pub fn encode_png(pixels: &image::RgbImage) -> anyhow::Result<Vec<u8>> {
    use std::io::Cursor;
    let mut buf = Vec::new();
    image::DynamicImage::ImageRgb8(pixels.clone())
        .write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Png)
        .map_err(|e| anyhow::anyhow!("{}", e))?;
    Ok(buf)
}

/// Split base64 PNG bytes into transmit chunk sequences.
/// First chunk carries the full control block; the rest carry only
/// `m=`. The final chunk is `m=0` (with payload when the data ends
/// mid-chunk, empty when it aligns exactly).
pub fn transmit_chunks(png_b64: &str, cols: usize, rows: usize) -> Vec<String> {
    let mut out = Vec::new();
    let bytes = png_b64.as_bytes();
    let mut chunks: Vec<&[u8]> = bytes.chunks(CHUNK_SIZE).collect();
    if chunks.is_empty() {
        chunks.push(&[]);
    }
    for (i, chunk) in chunks.iter().enumerate() {
        let last = i + 1 == chunks.len();
        let m = if last { 0 } else { 1 };
        let s = if i == 0 {
            format!(
                "\x1b_Ga=T,f=100,q=2,i={},c={},r={},m={};{}",
                KITTY_IMAGE_ID,
                cols,
                rows,
                m,
                String::from_utf8_lossy(chunk)
            )
        } else {
            format!("\x1b_Gm={};{}", m, String::from_utf8_lossy(chunk))
        };
        out.push(format!("{}\x1b\\", s));
    }
    out
}

/// Delete command: removes image id 7 and frees its data (`d=I`).
/// Send before every re-transmit and on viewer close so placements
/// never stack across terminals with different replace semantics.
pub fn delete_cmd() -> String {
    format!("\x1b_Ga=d,d=I,i={},q=2\x1b\\", KITTY_IMAGE_ID)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chunks_of(n: usize) -> Vec<String> {
        transmit_chunks(&"A".repeat(n), 80, 24)
    }

    #[test]
    fn single_chunk_framing() {
        let seqs = chunks_of(100);
        assert_eq!(seqs.len(), 1);
        let s = &seqs[0];
        assert!(s.starts_with("\x1b_Ga=T,f=100,q=2,i=7,c=80,r=24,m=0;"));
        assert!(s.ends_with("\x1b\\"));
        assert_eq!(s.matches("\x1b_G").count(), 1);
    }

    #[test]
    fn multi_chunk_framing() {
        // 9000 chars -> 4096 + 4096 + 824.
        let seqs = chunks_of(9000);
        assert_eq!(seqs.len(), 3);
        assert!(seqs[0].contains("m=1;"));
        assert!(seqs[0].starts_with("\x1b_Ga=T,f=100,q=2,i=7,"));
        assert_eq!(seqs[1], format!("\x1b_Gm=1;{}\x1b\\", "A".repeat(4096)));
        assert!(seqs[2].starts_with("\x1b_Gm=0;"));
        // Middle/final chunks carry only m (plus payload).
        assert!(!seqs[1].contains("a=T"));
        assert!(!seqs[2].contains("a=T"));
    }

    #[test]
    fn exact_multiple_needs_no_terminator() {
        // 8192 chars -> two full chunks; the final chunk carries m=0
        // WITH its payload (reference Python behavior — every chunk
        // is a multiple of 4, so no empty terminator is required).
        let seqs = chunks_of(8192);
        assert_eq!(seqs.len(), 2);
        assert!(seqs[0].contains("m=1;"));
        assert!(seqs[1].starts_with("\x1b_Gm=0;"));
        assert!(seqs[1].ends_with(&format!("{}\x1b\\", "A".repeat(4096))));
    }

    #[test]
    fn delete_command_bytes() {
        assert_eq!(delete_cmd(), "\x1b_Ga=d,d=I,i=7,q=2\x1b\\");
    }

    #[cfg(feature = "image2doc")]
    #[test]
    fn png_roundtrip_dims() {
        let img = image::RgbImage::from_pixel(37, 21, image::Rgb([9, 99, 199]));
        let bytes = encode_png(&img).unwrap();
        assert!(!bytes.is_empty());
        let back = image::load_from_memory(&bytes).unwrap();
        assert_eq!((back.width(), back.height()), (37, 21));
    }
}
