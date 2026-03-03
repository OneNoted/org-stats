use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

use crate::github::OrgData;

const TTL: Duration = Duration::from_secs(3600);

#[derive(Clone)]
struct CachedEntry {
    data: OrgData,
    inserted_at: Instant,
}

#[derive(Clone)]
pub struct Cache {
    inner: Arc<RwLock<HashMap<String, CachedEntry>>>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get(&self, key: &str) -> Option<OrgData> {
        let map = self.inner.read().await;
        let entry = map.get(key)?;
        if entry.inserted_at.elapsed() < TTL {
            Some(entry.data.clone())
        } else {
            None
        }
    }

    /// Get even if expired (for serving stale on error)
    pub async fn get_stale(&self, key: &str) -> Option<OrgData> {
        let map = self.inner.read().await;
        map.get(key).map(|e| e.data.clone())
    }

    pub async fn set(&self, key: String, data: OrgData) {
        let mut map = self.inner.write().await;
        map.insert(
            key,
            CachedEntry {
                data,
                inserted_at: Instant::now(),
            },
        );
    }
}
