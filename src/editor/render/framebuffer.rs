use crossterm::cursor::MoveTo;
use crossterm::queue;
use crossterm::style::{Print, SetBackgroundColor, SetForegroundColor};
use std::io::Write;

/// A single cell in the framebuffer.
#[derive(Clone, PartialEq, Default)]
pub(crate) struct Cell {
    pub ch: char,
    pub fg: (u8, u8, u8),
    pub bg: (u8, u8, u8),
}

impl Cell {
    pub fn _new(ch: char, fg: (u8, u8, u8), bg: (u8, u8, u8)) -> Self {
        Self { ch, fg, bg }
    }
}

/// Double-buffered terminal framebuffer.
/// Stores the rendered state and diffs against the previous frame
/// to only emit changed cells.
pub(crate) struct FrameBuffer {
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>,
    prev_cells: Vec<Vec<Cell>>,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let empty = vec![vec![Cell::default(); width]; height];
        Self {
            width,
            height,
            cells: empty.clone(),
            prev_cells: empty,
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        if width != self.width || height != self.height {
            self.width = width;
            self.height = height;
            let empty = vec![vec![Cell::default(); width]; height];
            self.cells = empty.clone();
            self.prev_cells = empty;
        }
    }

    /// Set a cell at (x, y) in the current frame.
    pub fn _set(&mut self, x: usize, y: usize, ch: char, fg: (u8, u8, u8), bg: (u8, u8, u8)) {
        if y < self.height && x < self.width {
            self.cells[y][x] = Cell::_new(ch, fg, bg);
        }
    }

    /// Clear a row in the current frame.
    pub fn _clear_row(&mut self, y: usize) {
        if y < self.height {
            self.cells[y].fill(Cell::default());
        }
    }

    /// Diff current vs previous frame and emit only changed cells to stdout.
    /// Returns the number of cells written.
    pub fn diff_and_flush(&mut self, stdout: &mut impl Write) -> std::io::Result<usize> {
        let mut written = 0;
        let mut last_fg = (0u8, 0u8, 0u8);
        let mut last_bg = (0u8, 0u8, 0u8);
        let mut need_move = true;

        for y in 0..self.height {
            for x in 0..self.width {
                let cur = &self.cells[y][x];
                let prev = &self.prev_cells[y][x];

                if cur != prev {
                    // Only emit if something changed
                    if need_move || cur.fg != last_fg || cur.bg != last_bg {
                        queue!(stdout, MoveTo(x as u16, y as u16))?;
                        if cur.fg != last_fg {
                            queue!(
                                stdout,
                                SetForegroundColor(crossterm::style::Color::Rgb {
                                    r: cur.fg.0,
                                    g: cur.fg.1,
                                    b: cur.fg.2,
                                })
                            )?;
                        }
                        if cur.bg != last_bg {
                            queue!(
                                stdout,
                                SetBackgroundColor(crossterm::style::Color::Rgb {
                                    r: cur.bg.0,
                                    g: cur.bg.1,
                                    b: cur.bg.2,
                                })
                            )?;
                        }
                        last_fg = cur.fg;
                        last_bg = cur.bg;
                        need_move = false;
                    }
                    queue!(stdout, Print(cur.ch))?;
                    written += 1;
                } else {
                    need_move = true;
                }
            }
        }

        // Swap buffers
        std::mem::swap(&mut self.cells, &mut self.prev_cells);

        // Clear current frame for next render
        for row in &mut self.cells {
            row.fill(Cell::default());
        }

        Ok(written)
    }

    /// Reset the previous frame to force a full redraw on next diff.
    pub fn _reset_prev(&mut self) {
        for row in &mut self.prev_cells {
            row.fill(Cell::default());
        }
    }
}
