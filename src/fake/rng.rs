// src/fake/rng.rs
//! Seeded / thread random source shared by every `fake` generator.

use rand::rngs::{StdRng, ThreadRng};
use rand::{Rng, RngExt, SeedableRng};

use super::locale::{Locale, LocaleData};

enum Inner {
    Seeded(Box<StdRng>),
    Thread(ThreadRng),
}

/// Random source for `fake`. Wraps either a deterministically seeded
/// [`StdRng`] (`fake gen -s <n>`) or the thread RNG. Also carries the active
/// [`Locale`] so culture-sensitive generators can pick locale-aware pools.
pub struct Gen {
    inner: Inner,
    locale: Locale,
    /// Roman (ASCII) form of the first name picked in the current row, if any.
    roman_first: Option<String>,
    /// Roman (ASCII) form of the last name picked in the current row, if any.
    roman_last: Option<String>,
    /// Index of the row currently being generated (drives the `id` type).
    row_index: usize,
}

impl Gen {
    /// Deterministic source: same seed ⇒ same output.
    pub fn seeded(seed: u64) -> Self {
        Gen {
            inner: Inner::Seeded(Box::new(StdRng::seed_from_u64(seed))),
            locale: Locale::EnUs,
            roman_first: None,
            roman_last: None,
            row_index: 0,
        }
    }

    /// Non-deterministic source.
    pub fn thread() -> Self {
        Gen {
            inner: Inner::Thread(rand::rng()),
            locale: Locale::EnUs,
            roman_first: None,
            roman_last: None,
            row_index: 0,
        }
    }

    /// Reset the per-row name scratch and record the row index. Called by the
    /// row engine before generating each row (and before each unique retry).
    pub fn begin_row(&mut self, index: usize) {
        self.row_index = index;
        self.roman_first = None;
        self.roman_last = None;
    }

    /// Zero-based index of the row being generated.
    pub fn row_index(&self) -> usize {
        self.row_index
    }

    /// Remember the roman form of the first name picked this row.
    pub fn set_roman_first(&mut self, roman: &str) {
        self.roman_first = Some(roman.to_string());
    }

    /// Remember the roman form of the last name picked this row.
    pub fn set_roman_last(&mut self, roman: &str) {
        self.roman_last = Some(roman.to_string());
    }

    pub fn roman_first(&self) -> Option<&str> {
        self.roman_first.as_deref()
    }

    pub fn roman_last(&self) -> Option<&str> {
        self.roman_last.as_deref()
    }

    /// Active locale (defaults to `en_US`).
    pub fn locale(&self) -> Locale {
        self.locale
    }

    /// Switch the active locale.
    pub fn set_locale(&mut self, locale: Locale) {
        self.locale = locale;
    }

    /// The active locale's pools/patterns.
    pub fn locale_data(&self) -> &'static LocaleData {
        self.locale.data()
    }

    /// Inclusive integer range `[lo, hi]`; returns `lo` when the range is empty.
    pub fn range_usize(&mut self, lo: usize, hi: usize) -> usize {
        if hi <= lo {
            return lo;
        }
        match &mut self.inner {
            Inner::Seeded(r) => rand_range(r, lo, hi),
            Inner::Thread(r) => rand_range(r, lo, hi),
        }
    }

    /// Inclusive signed range `[lo, hi]`.
    pub fn range_i64(&mut self, lo: i64, hi: i64) -> i64 {
        if hi <= lo {
            return lo;
        }
        match &mut self.inner {
            Inner::Seeded(r) => rand_range_i64(r, lo, hi),
            Inner::Thread(r) => rand_range_i64(r, lo, hi),
        }
    }

    /// Uniform float in `[lo, hi)`.
    pub fn range_f64(&mut self, lo: f64, hi: f64) -> f64 {
        if hi <= lo {
            return lo;
        }
        match &mut self.inner {
            Inner::Seeded(r) => r.random_range(lo..hi),
            Inner::Thread(r) => r.random_range(lo..hi),
        }
    }

    /// `true` with probability `p`.
    pub fn chance(&mut self, p: f64) -> bool {
        match &mut self.inner {
            Inner::Seeded(r) => r.random_bool(p),
            Inner::Thread(r) => r.random_bool(p),
        }
    }

    /// Random element, panicking only when `items` is empty.
    pub fn pick<'a, T>(&mut self, items: &'a [T]) -> &'a T {
        let i = self.range_usize(0, items.len().saturating_sub(1));
        &items[i]
    }

    /// Random `&'static str` from a slice.
    pub fn pick_str(&mut self, items: &[&'static str]) -> &'static str {
        let i = self.range_usize(0, items.len().saturating_sub(1));
        items[i]
    }

    /// `n` random ASCII digits.
    pub fn digits(&mut self, n: usize) -> String {
        let mut s = String::with_capacity(n);
        for _ in 0..n {
            s.push((b'0' + self.range_usize(0, 9) as u8) as char);
        }
        s
    }

    /// `n` random lowercase hex characters.
    pub fn hex(&mut self, n: usize) -> String {
        const HEX: &[u8] = b"0123456789abcdef";
        let mut s = String::with_capacity(n);
        for _ in 0..n {
            s.push(HEX[self.range_usize(0, 15)] as char);
        }
        s
    }

    /// `n` random characters from the given alphabet.
    pub fn from_alphabet(&mut self, n: usize, alphabet: &[u8]) -> String {
        let mut s = String::with_capacity(n);
        for _ in 0..n {
            s.push(alphabet[self.range_usize(0, alphabet.len() - 1)] as char);
        }
        s
    }

    fn bytes16(&mut self) -> [u8; 16] {
        match &mut self.inner {
            Inner::Seeded(r) => r.random(),
            Inner::Thread(r) => r.random(),
        }
    }

    /// Raw random bytes (used by uuid / ulid / nanoid).
    pub fn bytes(&mut self, n: usize) -> Vec<u8> {
        let mut out = Vec::with_capacity(n);
        while out.len() < n {
            out.extend_from_slice(&self.bytes16());
        }
        out.truncate(n);
        out
    }
}

fn rand_range<R: Rng + ?Sized>(r: &mut R, lo: usize, hi: usize) -> usize {
    r.random_range(lo..=hi)
}

fn rand_range_i64<R: Rng + ?Sized>(r: &mut R, lo: i64, hi: i64) -> i64 {
    r.random_range(lo..=hi)
}
