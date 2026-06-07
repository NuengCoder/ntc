// src/fuzzy.rs
// Fuzzy string matching using Jaro-Winkler distance
// Version: v1.8.0
// Used by: fs (file search) and ds (directory search) commands

/// Jaro-Winkler similarity score between two strings.
/// Returns a value between 0.0 (no match) and 1.0 (identical).
///
/// The Jaro-Winkler algorithm gives higher weight to strings that match from the beginning.
/// This is ideal for filename matching because prefixes matter.
///
/// # Examples
/// ```
/// let score = jaro_winkler("main.c", "mian.c");
/// assert!(score > 0.8);
///
/// let score = jaro_winkler("main.c", "completely_different.txt");
/// assert!(score < 0.3);
/// ```
pub fn jaro_winkler(s1: &str, s2: &str) -> f64 {
    // Handle empty strings
    if s1.is_empty() && s2.is_empty() {
        return 1.0;
    }
    if s1.is_empty() || s2.is_empty() {
        return 0.0;
    }

    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();
    let len1 = s1_chars.len();
    let len2 = s2_chars.len();

    // Maximum distance for a match (floor of max length / 2 - 1)
    let match_distance = (len1.max(len2) / 2).saturating_sub(1);

    // Find matching characters
    let mut s1_matches = vec![false; len1];
    let mut s2_matches = vec![false; len2];
    let mut matches = 0;

    for i in 0..len1 {
        let start = i.saturating_sub(match_distance);
        // FIX: was `.min(len2).saturating_sub(1)` which incorrectly shrunk the window by 1
        let end = (i + match_distance).min(len2 - 1);
        if start > end {
            continue;
        }
        for j in start..=end {
            if !s2_matches[j] && s1_chars[i] == s2_chars[j] {
                s1_matches[i] = true;
                s2_matches[j] = true;
                matches += 1;
                break;
            }
        }
    }

    if matches == 0 {
        return 0.0;
    }

    // Count transpositions
    let mut transpositions = 0;
    let mut k = 0;
    for i in 0..len1 {
        if s1_matches[i] {
            while !s2_matches[k] {
                k += 1;
            }
            if s1_chars[i] != s2_chars[k] {
                transpositions += 1;
            }
            k += 1;
        }
    }

    let transpositions_half = transpositions as f64 / 2.0;

    // Jaro similarity
    let m = matches as f64;
    let jaro = (m / len1 as f64 + m / len2 as f64 + (m - transpositions_half) / m) / 3.0;

    // Winkler adjustment (boost for common prefix, capped at 4 chars)
    let prefix_len = s1_chars
        .iter()
        .zip(s2_chars.iter())
        .take_while(|(a, b)| a == b)
        .count()
        .min(4);

    let winkler = jaro + (prefix_len as f64 * 0.1 * (1.0 - jaro));

    winkler.min(1.0)
}

/// Case-insensitive fuzzy match between two strings.
/// Converts both strings to lowercase before applying Jaro-Winkler.
///
/// Note: If you are calling this in a hot loop over many candidates,
/// pre-lowercase the query once outside the loop and call `jaro_winkler` directly
/// to avoid repeated allocations on the query string.
pub fn jaro_winkler_case_insensitive(s1: &str, s2: &str) -> f64 {
    jaro_winkler(&s1.to_lowercase(), &s2.to_lowercase())
}

/// Threshold for fuzzy matching files (0.75 = 75% similarity).
/// Raised from 0.65 to reduce false positives on short unrelated filenames
/// e.g. "main.rs" vs "test.py" can score ~0.66 which is a false positive.
pub const FUZZY_THRESHOLD: f64 = 0.75;

/// Threshold for fuzzy matching directories (0.72).
/// Slightly more lenient than file threshold since directory names
/// tend to be longer and more distinct words.
pub const FUZZY_DIR_THRESHOLD: f64 = 0.72;

/// Maximum number of fuzzy suggestions to display (top 3)
pub const MAX_FUZZY_SUGGESTIONS: usize = 3;

/// Determine if a filename should be considered a fuzzy match candidate.
/// Uses the stricter file threshold (0.75).
pub fn is_fuzzy_match(score: f64) -> bool {
    score >= FUZZY_THRESHOLD
}

/// Determine if a directory name should be considered a fuzzy match candidate.
/// Uses the more lenient directory threshold (0.72).
pub fn is_fuzzy_dir_match(score: f64) -> bool {
    score >= FUZZY_DIR_THRESHOLD
}

/// Calculate fuzzy match score and return it with the matched string.
/// Useful for sorting results by relevance.
pub fn score_candidate(candidate: &str, query: &str) -> (String, f64) {
    let score = jaro_winkler_case_insensitive(candidate, query);
    (candidate.to_string(), score)
}

/// Get the top N fuzzy matches from a list of candidates (files).
/// Uses FUZZY_THRESHOLD (0.75).
/// Returns Vec of (candidate, score) sorted by score descending.
///
/// Pre-lowercases the query once to avoid repeated allocations in the inner loop.
pub fn top_fuzzy_matches<'a>(
    candidates: &'a [String],
    query: &str,
    limit: usize,
) -> Vec<(&'a String, f64)> {
    let query_lower = query.to_lowercase();
    let mut scored: Vec<(&'a String, f64)> = candidates
        .iter()
        .filter_map(|candidate| {
            let score = jaro_winkler(&candidate.to_lowercase(), &query_lower);
            if score >= FUZZY_THRESHOLD {
                Some((candidate, score))
            } else {
                None
            }
        })
        .collect();

    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    scored.truncate(limit);
    scored
}

/// Get the top N fuzzy matches from a list of candidates (directories).
/// Uses FUZZY_DIR_THRESHOLD (0.72).
/// Returns Vec of (candidate, score) sorted by score descending.
///
/// Pre-lowercases the query once to avoid repeated allocations in the inner loop.
pub fn top_fuzzy_dir_matches<'a>(
    candidates: &'a [String],
    query: &str,
    limit: usize,
) -> Vec<(&'a String, f64)> {
    let query_lower = query.to_lowercase();
    let mut scored: Vec<(&'a String, f64)> = candidates
        .iter()
        .filter_map(|candidate| {
            let score = jaro_winkler(&candidate.to_lowercase(), &query_lower);
            if score >= FUZZY_DIR_THRESHOLD {
                Some((candidate, score))
            } else {
                None
            }
        })
        .collect();

    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    scored.truncate(limit);
    scored
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match() {
        let score = jaro_winkler("main.c", "main.c");
        assert!((score - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_case_insensitive() {
        let score = jaro_winkler_case_insensitive("Main.C", "main.c");
        assert!((score - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_similar_typo() {
        let score = jaro_winkler("main.c", "mian.c");
        assert!(score > 0.85);
        assert!(is_fuzzy_match(score));
    }

    #[test]
    fn test_unrelated() {
        let score = jaro_winkler("main.c", "xyz.txt");
        assert!(score < 0.3);
        assert!(!is_fuzzy_match(score));
    }

    #[test]
    fn test_prefix_boost() {
        // "ma" prefix gives extra boost
        let score1 = jaro_winkler("main.c", "mian.c");
        let score2 = jaro_winkler("ain.c", "mian.c");
        assert!(score1 > score2);
    }

    #[test]
    fn test_top_fuzzy_matches() {
        let candidates = vec![
            "main.c".to_string(),
            "mian.c".to_string(),
            "main.h".to_string(),
            "test.c".to_string(),
        ];
        let results = top_fuzzy_matches(&candidates, "mian.c", 2);
        assert_eq!(results.len(), 2);
        assert!(results[0].0 == "main.c" || results[0].0 == "mian.c");
    }

    #[test]
    fn test_empty_strings() {
        assert_eq!(jaro_winkler("", ""), 1.0);
        assert_eq!(jaro_winkler("main.c", ""), 0.0);
        assert_eq!(jaro_winkler("", "main.c"), 0.0);
    }

    /// Regression: false positive that was passing at old threshold 0.65
    #[test]
    fn test_no_false_positive_unrelated_files() {
        let score = jaro_winkler_case_insensitive("main.rs", "test.py");
        // Should NOT be a file fuzzy match at new threshold 0.75
        assert!(!is_fuzzy_match(score), "score was {:.3}", score);
    }

    /// Confirm end-bound fix: match_distance window is not under-counted
    #[test]
    fn test_end_bound_fix() {
        // "abcde" vs "bcdea" — last char transposition
        // Old buggy code would miss matches near the end of short strings
        let score = jaro_winkler("abcde", "bcdea");
        assert!(score > 0.8, "score was {:.3}", score);
    }

    #[test]
    fn test_dir_threshold_more_lenient() {
        // A marginal score that passes dir threshold but not file threshold
        let score = jaro_winkler_case_insensitive("modules", "module");
        assert!(is_fuzzy_dir_match(score), "score was {:.3}", score);
    }

    #[test]
    fn test_top_fuzzy_dir_matches() {
        let candidates = vec![
            "src".to_string(),
            "source".to_string(),
            "scripts".to_string(),
            "target".to_string(),
        ];
        let results = top_fuzzy_dir_matches(&candidates, "srce", 3);
        // "src" should rank highest
        assert!(!results.is_empty());
        assert_eq!(results[0].0, "src");
    }

    #[test]
    fn test_query_lowercased_once() {
        // Ensure case-insensitive top matches work correctly
        let candidates = vec![
            "Main.rs".to_string(),
            "main.rs".to_string(),
            "MAIN.RS".to_string(),
        ];
        let results = top_fuzzy_matches(&candidates, "MAIN.RS", 3);
        assert_eq!(results.len(), 3);
        // All three should score 1.0
        for (_, score) in &results {
            assert!((*score - 1.0).abs() < 0.001, "score was {:.3}", score);
        }
    }
}