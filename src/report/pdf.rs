use crate::config::Config;
use crate::explorer::{generate_tree, format_tree};
use crate::output::cat_file_with_line_numbers;
use anyhow::Result;
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

const PAGE_W: Mm = Mm(210.0);
const PAGE_H: Mm = Mm(297.0);
const MARGIN: Mm = Mm(20.0);
const FONT_SIZE: f64 = 9.0;
const TITLE_SIZE: f64 = 18.0;
const HEADING_SIZE: f64 = 14.0;
const LINE_HEIGHT: f64 = 4.5;

fn collect_files(dir_path: &Path, max_depth: usize) -> (Vec<PathBuf>, Vec<PathBuf>) {
    super::collect_report_files(dir_path, max_depth)
}

pub fn generate_pdf_report(dir_path: &Path, output_path: &Path) -> Result<()> {
    let dir_name = dir_path.file_name().unwrap_or_default().to_string_lossy().to_string();
    let show_lines = Config::global_get_show_line_numbers();
    let max_depth = Config::global_get_max_depth();
    let tree = generate_tree(dir_path.to_string_lossy().as_ref(), None, true, None);

    let (doc, first_page, first_layer) = PdfDocument::new(
        format!("{} directory report", dir_name),
        PAGE_W,
        PAGE_H,
        "Layer 1".to_string(),
    );

    let font_regular = doc.add_builtin_font(BuiltinFont::Helvetica)?;
    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)?;
    let font_mono = doc.add_builtin_font(BuiltinFont::Courier)?;

    struct PageState {
        doc: PdfDocumentReference,
        pages: Vec<(PdfPageIndex, PdfLayerIndex)>,
        current: usize,
        y: Mm,
    }

    impl PageState {
        fn layer(&self) -> PdfLayerReference {
            let (pi, li) = self.pages[self.current];
            self.doc.get_page(pi).get_layer(li)
        }

        fn new_page(&mut self) {
            let (pi, li) = self.doc.add_page(PAGE_W, PAGE_H, "Layer 1".to_string());
            self.pages.push((pi, li));
            self.current = self.pages.len() - 1;
            self.y = Mm(PAGE_H.0 - MARGIN.0);
        }

        fn write_text(&mut self, font: &IndirectFontRef, text: &str) {
            if self.y.0 < MARGIN.0 + 5.0 {
                self.new_page();
            }
            let layer = self.layer();
            layer.use_text(text.to_string(), FONT_SIZE, MARGIN, self.y, font);
            self.y = Mm(self.y.0 - LINE_HEIGHT);
        }

        fn ensure_space(&mut self, needed: f64) {
            if self.y.0 < MARGIN.0 + needed {
                self.new_page();
            }
        }
    }

    let mut st = PageState {
        pages: vec![(first_page, first_layer)],
        current: 0,
        y: Mm(PAGE_H.0 - MARGIN.0),
        doc,
    };

    // Title
    let title = format!("{} - Directory Report", dir_name);
    st.layer().use_text(title, TITLE_SIZE, MARGIN, st.y, &font_bold);
    st.y = Mm(st.y.0 - TITLE_SIZE - 6.0);

    let date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    st.layer().use_text(date, 10.0, MARGIN, st.y, &font_regular);
    st.y = Mm(st.y.0 - 10.0 - 10.0);

    // Separator
    st.write_text(&font_mono, &"=".repeat(90));

    // Tree heading
    st.ensure_space(20.0);
    st.layer().use_text("Directory Tree".to_string(), HEADING_SIZE, MARGIN, st.y, &font_bold);
    st.y = Mm(st.y.0 - HEADING_SIZE - 4.0);

    // Tree content
    let tree_str = format_tree(&tree, "", true);
    for line in tree_str.lines() {
        st.write_text(&font_mono, line);
    }

    st.y = Mm(st.y.0 - 4.0);

    // Content section header
    st.write_text(&font_mono, &"=".repeat(90));
    st.layer().use_text(dir_name.clone(), HEADING_SIZE, MARGIN, st.y, &font_bold);
    st.y = Mm(st.y.0 - HEADING_SIZE - 4.0);
    st.write_text(&font_mono, &"=".repeat(90));

    // File contents
    let (supported_files, unsupported_files) = collect_files(dir_path, max_depth);

    for file_path in &supported_files {
        let name = file_path.file_name().unwrap_or_default().to_string_lossy().to_string();

        st.ensure_space(20.0);
        st.write_text(&font_mono, &format!("── {} ──", name));

        match cat_file_with_line_numbers(file_path, show_lines) {
            Ok(content) => {
                for line in content.lines() {
                    st.write_text(&font_mono, line);
                }
            }
            Err(_) => {
                st.write_text(&font_mono, "[Error reading file]");
            }
        }

        st.y = Mm(st.y.0 - 2.0);
    }

    if !unsupported_files.is_empty() {
        st.ensure_space(20.0);
        st.write_text(&font_mono, "── Unsupported Files (skipped) ──");
        for file_path in &unsupported_files {
            let name = file_path.file_name().unwrap_or_default().to_string_lossy();
            st.write_text(&font_mono, &format!("Skipped: {}", name));
        }
    }

    st.doc.save(&mut BufWriter::new(File::create(output_path)?))?;
    Ok(())
}
