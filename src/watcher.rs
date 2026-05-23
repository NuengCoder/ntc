use anyhow::Result;
use notify::{Event, EventKind, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

/// Start watching a directory for changes. Returns a flag that gets set when changes occur.
pub fn start_watcher(path: &Path) -> Result<(notify::RecommendedWatcher, Arc<AtomicBool>)> {
    let changed = Arc::new(AtomicBool::new(false));
    let changed_clone = changed.clone();
    let path = path.to_path_buf();

    let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
        if let Ok(event) = res {
            match event.kind {
                EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => {
                    changed_clone.store(true, Ordering::Release);
                }
                _ => {}
            }
        }
    })?;

    watcher.configure(notify::Config::default().with_poll_interval(Duration::from_secs(2)))?;
    watcher.watch(&path, RecursiveMode::NonRecursive)?;

    Ok((watcher, changed))
}