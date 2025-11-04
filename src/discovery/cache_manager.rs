use std::path::PathBuf;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use tracing::{error, info, warn};

use super::{save_cache, DiscoveredProject};

/// Cache manager with async, non-blocking saves
/// Deduplicates: if save is already pending, ignores new requests until write completes
#[derive(Clone)]
pub struct CacheManager {
    tx: mpsc::UnboundedSender<Vec<DiscoveredProject>>,
}

impl CacheManager {
    /// Create a new cache manager and spawn background worker
    pub fn new(cache_location: PathBuf) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        // Spawn background worker
        tokio::spawn(cache_worker(rx, cache_location));

        Self { tx }
    }

    /// Queue a cache save (non-blocking, returns immediately)
    /// If save is already pending, no-ops silently
    pub fn queue_save(&self, projects: Vec<DiscoveredProject>) {
        // Send is cheap (just puts in queue), doesn't block
        // Worker will deduplicate if save already pending
        if let Err(e) = self.tx.send(projects) {
            error!("⚠️  Failed to queue cache save: {}", e);
        }
    }
}

/// Background worker that processes cache save requests
async fn cache_worker(
    mut rx: mpsc::UnboundedReceiver<Vec<DiscoveredProject>>,
    cache_location: PathBuf,
) {
    let mut pending: Option<Vec<DiscoveredProject>> = None;

    loop {
        // Wait for a save request or timeout
        tokio::select! {
            // New save request received
            Some(projects) = rx.recv() => {
                // Only store if no save is already pending (deduplication)
                if pending.is_none() {
                    pending = Some(projects);
                }
                // Otherwise silently ignore (already have pending save)
            }

            // Timeout: write pending if any
            _ = sleep(Duration::from_millis(100)), if pending.is_some() => {
                if let Some(projects) = pending.take() {
                    // Write cache in background (releases mutex)
                    if let Err(e) = save_cache(&projects, &cache_location) {
                        warn!("⚠️  Cache save failed: {}", e);
                    } else {
                        info!("💾 Cache saved ({} projects)", projects.len());
                    }
                }
            }
        }
    }
}
