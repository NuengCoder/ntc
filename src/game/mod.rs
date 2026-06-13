use anyhow::{bail, Result};
use crossterm::{
    cursor::{Hide, Show, MoveTo},
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    style::Print,
};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use std::io::{stdout, Write};
use std::path::PathBuf;

const W: usize = 60;
const H: usize = 20;
const GROUND: usize = H - 2;
const DINO_X: usize = 8;
const BIRD_Y: usize = 9;

#[derive(Serialize, Deserialize)]
struct DinoConfig {
    high_score: u64,
}

impl DinoConfig {
    fn path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("ntc")
            .join("dino.toml")
    }

    fn load() -> Self {
        let path = Self::path();
        if path.exists() {
            if let Ok(s) = std::fs::read_to_string(&path) {
                if let Ok(cfg) = toml::from_str(&s) {
                    return cfg;
                }
            }
        }
        Self { high_score: 0 }
    }

    fn save(&self) {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(s) = toml::to_string_pretty(self) {
            let _ = std::fs::write(&path, s);
        }
    }
}

struct Dino {
    y: f64,
    vy: f64,
    jumping: bool,
}

impl Dino {
    fn new() -> Self {
        Self { y: 0.0, vy: 0.0, jumping: false }
    }

    fn jump(&mut self) {
        if !self.jumping {
            self.vy = -12.0;
            self.jumping = true;
        }
    }

    fn update(&mut self, dt: f64) {
        if self.jumping {
            self.vy += 35.0 * dt;
            self.y += self.vy * dt;
            if self.y >= 0.0 {
                self.y = 0.0;
                self.vy = 0.0;
                self.jumping = false;
            }
        }
    }

    fn sy(&self) -> usize {
        let offset = (self.y * 2.0).round() as isize;
        (GROUND as isize + offset).clamp(0, GROUND as isize) as usize
    }
}

struct Spike {
    x: usize,
    scored: bool,
}

impl Spike {
    fn new(x: usize) -> Self {
        Self { x, scored: false }
    }

    fn update(&mut self, speed: f64) {
        if self.x > 0 {
            self.x = (self.x as f64 - speed).round() as usize;
        }
    }

    fn hit(&self, sy: usize) -> bool {
        let db = sy;
        let dt = sy.saturating_sub(3);
        let ol = self.x;
        let or = self.x + 2;
        or >= DINO_X && ol <= DINO_X + 3 && db >= GROUND - 1 && dt <= GROUND
    }
}

struct Bird {
    x: usize,
    scored: bool,
}

impl Bird {
    fn new(x: usize) -> Self {
        Self { x, scored: false }
    }

    fn update(&mut self, speed: f64) {
        if self.x > 0 {
            self.x = (self.x as f64 - speed).round() as usize;
        }
    }

    fn hit(&self, sy: usize) -> bool {
        let db = sy;
        let dt = sy.saturating_sub(3);
        let ol = self.x;
        let or = self.x + 3;
        or >= DINO_X && ol <= DINO_X + 3 && db >= BIRD_Y && dt <= BIRD_Y + 2
    }
}

fn frame_buffer(score: u64, high_score: u64, speed: f64, sy: usize, spikes: &[Spike], birds: &[Bird], over: bool) -> String {
    let mut buf = String::with_capacity(W * H * 4);

    let spd = speed / 2.0;
    let top = format!(
        " DINO RUNNER   SCORE:{:04}   HI:{:04}   SPD:{:.1}x ",
        score, high_score, spd
    );
    buf.push_str(&format!("┌{}┐\n", "─".repeat(W)));
    buf.push_str(&format!("│{:w$}│\n", top, w = W));
    buf.push_str(&format!("├{}┤\n", "─".repeat(W)));

    for row in 0..GROUND {
        buf.push('│');
        for col in 0..W {
            let mut ch = ' ';

            for s in spikes {
                if col >= s.x && col < s.x + 2 && row >= GROUND - 1 {
                    ch = if row == GROUND - 1 { '▲' } else { ' ' };
                }
            }

            for b in birds {
                if col >= b.x && col < b.x + 3 && (BIRD_Y..=BIRD_Y + 1).contains(&row) {
                    let local = col - b.x;
                    let brow = row - BIRD_Y;
                    ch = match (brow, local) {
                        (0, 0) => '╱',
                        (0, 1) => '╲',
                        (0, 2) => '╱',
                        (1, 0) => '▔',
                        (1, 1) => '▔',
                        (1, 2) => '▔',
                        _ => ' ',
                    };
                }
            }

            if (DINO_X..DINO_X + 4).contains(&col) && !over {
                let drow = sy;
                if row == drow.saturating_sub(3) {
                    ch = if col == DINO_X + 1 || col == DINO_X + 2 { '▀' } else { ' ' };
                } else if row == drow.saturating_sub(2) {
                    ch = match col - DINO_X { 0 => '█', 1 => '█', 2 => '█', _ => ' ' };
                } else if row == drow.saturating_sub(1) {
                    ch = match col - DINO_X { 0 => '╱', 1 => '█', 2 => '█', 3 => '╲', _ => ' ' };
                } else if row == drow {
                    ch = if col == DINO_X + 1 || col == DINO_X + 2 { '▄' } else { ' ' };
                }
            }

            if row == GROUND - 1 {
                let pat = (score / 3) % 4;
                let is_ground = match pat {
                    0 => col % 4 == 0 || col % 4 == 1,
                    1 => col % 4 == 1 || col % 4 == 2,
                    2 => col % 4 == 2 || col % 4 == 3,
                    _ => col % 4 == 0 || col % 4 == 3,
                };
                if is_ground && ch == ' ' { ch = '▄'; }
            }

            if over && (GROUND / 2 - 1..=GROUND / 2 + 1).contains(&row) {
                let msg = " GAME OVER ";
                let start = W / 2 - 5;
                if col >= start && col < start + msg.len() {
                    let idx = col - start;
                    ch = msg.as_bytes()[idx] as char;
                }
            }

            buf.push(ch);
        }
        buf.push_str("│\n");
    }

    buf.push_str(&format!("└{}┘\n", "─".repeat(W)));

    if over {
        buf.push_str(&format!("{:^w$}\n", "SPACE / ENTER = restart    ESC = quit", w = W + 2));
    } else {
        buf.push_str(&format!("{:^w$}\n", "SPACE / UP = jump    ESC = quit", w = W + 2));
    }

    buf
}

fn _run() -> Result<()> {
    execute!(stdout(), EnterAlternateScreen, Hide)?;
    terminal::enable_raw_mode()?;

    let cfg = DinoConfig::load();
    let mut dino = Dino::new();
    let mut spikes: Vec<Spike> = Vec::new();
    let mut birds: Vec<Bird> = Vec::new();
    let mut score: u64 = 0;
    let mut high_score: u64 = cfg.high_score;
    let mut over = false;
    let mut frame_start = Instant::now();
    let mut spawn_timer = 0.0;
    let mut speed = 2.0;
    let (mut cols, mut rows) = terminal::size()?;
    let mut prev_frame = String::new();

    loop {
        let new_size = terminal::size()?;
        if new_size != (cols, rows) {
            (cols, rows) = new_size;
            execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
            prev_frame.clear();
        }

        let dt = frame_start.elapsed().as_secs_f64().min(0.05);
        frame_start = Instant::now();

        if !over {
            speed = 2.0 + (score as f64 / 60.0).min(12.0);

            dino.update(dt);

            spawn_timer += dt;
            let interval = 0.8_f64.max(1.8 - score as f64 / 200.0);
            if spawn_timer >= interval {
                spawn_timer = 0.0;

                let spawn_bird = score > 30 && (score / 50).is_multiple_of(2);
                let too_close = if !spikes.is_empty() { W - spikes.last().unwrap().x <= 18 } else { false };

                if spawn_bird && !too_close && (birds.is_empty() || W - birds.last().unwrap().x > 25) {
                    birds.push(Bird::new(W));
                } else if !too_close {
                    spikes.push(Spike::new(W));
                }
            }

            for s in &mut spikes {
                s.update(speed);
                if !s.scored && s.x + 2 < DINO_X {
                    s.scored = true;
                    score += 1 + (s.x as u64 % 3);
                }
            }
            spikes.retain(|s| s.x + 2 > 0);

            for b in &mut birds {
                b.update(speed);
                if !b.scored && b.x + 3 < DINO_X {
                    b.scored = true;
                    score += 2 + (b.x as u64 % 3);
                }
            }
            birds.retain(|b| b.x + 3 > 0);

            for s in &spikes {
                if s.hit(dino.sy()) {
                    over = true;
                    break;
                }
            }
            if !over {
                for b in &birds {
                    if b.hit(dino.sy()) {
                        over = true;
                        break;
                    }
                }
            }
            if over
                && score > high_score {
                    high_score = score;
                    DinoConfig { high_score }.save();
                }
        }

        let sy = dino.sy();
        let frame = frame_buffer(score, high_score, speed, sy, &spikes, &birds, over);

        if frame != prev_frame {
            execute!(
                stdout(),
                MoveTo(0, 0),
                terminal::Clear(terminal::ClearType::All),
                Print(&frame),
            )?;
            stdout().flush()?;
            prev_frame = frame;
        }

        if event::poll(Duration::from_millis(25))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc | KeyCode::Char('q') => break,
                        KeyCode::Enter | KeyCode::Char(' ') => {
                            if over {
                                dino = Dino::new();
                                spikes.clear();
                                birds.clear();
                                score = 0;
                                speed = 2.0;
                                spawn_timer = 0.0;
                                over = false;
                            } else {
                                dino.jump();
                            }
                        }
                        KeyCode::Up
                            if !over => {
                                dino.jump();
                            }
                        _ => {}
                    }
                }
            }
        }
    }

    terminal::disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen, Show)?;
    println!("Score: {}  |  High Score: {}  |  Saved: dino.toml", score, high_score);
    Ok(())
}

pub fn run() -> Result<()> {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Result<()> {
        _run()
    }));
    match result {
        Ok(r) => r,
        Err(_) => {
            let _ = terminal::disable_raw_mode();
            let _ = execute!(stdout(), LeaveAlternateScreen, Show);
            bail!("Game crashed — terminal state restored");
        }
    }
}
