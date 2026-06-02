use anyhow::Result;
use notify::{Event, EventKind, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

const DEBOUNCE_MS: u64 = 400;

pub struct WatcherHandle {
    pub changed: Arc<AtomicBool>,
    pub last_event: Arc<Mutex<Instant>>,
    watcher: Mutex<notify::RecommendedWatcher>,
    current_path: Mutex<PathBuf>,
}

impl WatcherHandle {
    /// Returns `true` if a debounced change should trigger a refresh.
    pub fn should_refresh(&self) -> bool {
        if !self.changed.load(Ordering::Acquire) {
            return false;
        }
        let elapsed = self.last_event.lock().unwrap().elapsed();
        if elapsed >= Duration::from_millis(DEBOUNCE_MS) {
            self.changed.store(false, Ordering::Relaxed);
            true
        } else {
            false
        }
    }

    /// Update the watched path (e.g. after navigation).
    pub fn update_path(&self, new_path: &Path) -> Result<()> {
        let old = self.current_path.lock().unwrap().clone();
        if old == new_path.to_path_buf() {
            return Ok(());
        }

        let mut watcher = self.watcher.lock().unwrap();
        // Unwatch old, watch new
        let _ = watcher.unwatch(&old);
        watcher.watch(new_path, RecursiveMode::Recursive)?;
        *self.current_path.lock().unwrap() = new_path.to_path_buf();
        Ok(())
    }
}

/// Start watching a directory tree for changes.
/// Returns a `WatcherHandle` that can be queried (with debouncing) and updated.
pub fn start_watcher(path: &Path) -> Result<WatcherHandle> {
    let changed = Arc::new(AtomicBool::new(false));
    let last_event = Arc::new(Mutex::new(Instant::now()));
    let changed_clone = changed.clone();
    let last_event_clone = last_event.clone();
    let watch_path = path.to_path_buf();

    let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
        if let Ok(event) = res {
            match event.kind {
                EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => {
                    *last_event_clone.lock().unwrap() = Instant::now();
                    changed_clone.store(true, Ordering::Release);
                }
                _ => {}
            }
        }
    })?;

    watcher.watch(&watch_path, RecursiveMode::Recursive)?;

    Ok(WatcherHandle {
        changed,
        last_event,
        watcher: Mutex::new(watcher),
        current_path: Mutex::new(watch_path),
    })
}
