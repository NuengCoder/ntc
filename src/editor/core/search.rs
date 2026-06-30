use crate::editor::Editor;

impl Editor {
    pub(crate) fn rebuild_search_matches(&mut self) {
        let q = self.search.query.to_lowercase();
        if !self.search.dirty && self.search.cached_query == q {
            return;
        }
        self.search.matches.clear();
        self.search.cached_query = q.clone();
        self.search.dirty = false;
        if q.is_empty() {
            self.mark_all_dirty();
            return;
        }
        for (li, line) in self.lines.iter().enumerate() {
            let lower = line.to_lowercase();
            let mut start = 0;
            while let Some(pos) = lower[start..].find(&q) {
                let abs = start + pos;
                self.search.matches.push((li, abs, abs + q.len()));
                start = abs + 1;
                if start >= lower.len() {
                    break;
                }
            }
        }
        self.mark_all_dirty();
    }

    pub(crate) fn jump_to_match(&mut self, idx: usize) {
        if self.search.matches.is_empty() {
            return;
        }
        let (ly, sb, _eb) = self.search.matches[idx];
        self.cursor.y = ly;
        self.cursor.byte = sb;
    }

    pub(crate) fn search_next(&mut self) {
        if self.search.matches.is_empty() {
            self.status_msg = Some(format!("Pattern not found: {}", self.search.query));
            return;
        }
        self.search.match_idx = (self.search.match_idx + 1) % self.search.matches.len();
        self.jump_to_match(self.search.match_idx);
    }

    pub(crate) fn search_prev(&mut self) {
        if self.search.matches.is_empty() {
            self.status_msg = Some(format!("Pattern not found: {}", self.search.query));
            return;
        }
        if self.search.match_idx == 0 {
            self.search.match_idx = self.search.matches.len() - 1;
        } else {
            self.search.match_idx -= 1;
        }
        self.jump_to_match(self.search.match_idx);
    }
}
