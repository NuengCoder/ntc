use crate::editor::{auto_pair, Editor};

impl Editor {
    pub(crate) fn handle_char_input(&mut self, c: char) {
        if self.has_multiple_cursors() {
            self.multi_insert_at(c);
        } else {
            self.snapshot();
            if let Some(closing) = auto_pair(c) {
                self.insert_at(c);
                self.insert_at_raw(closing);
                self.cursor.byte = self.cursor.byte.saturating_sub(closing.len_utf8());
            } else {
                self.insert_at(c);
            }
        }
        if self.syntax.language == Some(crate::syntax::SyntaxLanguage::NtcMath) {
            self.trigger_completion();
        }
    }
}
