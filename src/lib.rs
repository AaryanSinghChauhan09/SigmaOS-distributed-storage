// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-distributed-storage: distributed storage component model.
//!
//! Real implementation: storages, events, validation, deterministic storage.
//! Replaces the placeholder stub.

/// Distributed filesystem: nodes, volumes, cluster health.
pub mod sigmadistfs;

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Storage {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DistributedStorageError {
    EmptyStorageName,
    DuplicateStorage(String),
    EmptyEventSubject,
}

impl fmt::Display for DistributedStorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyStorageName => write!(f, "storage name must not be empty"),
            Self::DuplicateStorage(id) => write!(f, "duplicate storage {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for DistributedStorageError {}

pub struct DistributedStorage {
    pub name: String,
    storages: BTreeMap<String, Storage>,
    events: BTreeMap<String, Event>,
}

impl DistributedStorage {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            storages: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_storage(&mut self, item: Storage) -> Result<(), DistributedStorageError> {
        if item.name.trim().is_empty() {
            return Err(DistributedStorageError::EmptyStorageName);
        }
        let id = item.id.clone();
        if self.storages.contains_key(&id) {
            return Err(DistributedStorageError::DuplicateStorage(id.clone()));
        }
        self.storages.insert(id, item);
        Ok(())
    }

    pub fn storage_count(&self) -> usize {
        self.storages.len()
    }

    pub fn get_storage(&self, id: &str) -> Option<&Storage> {
        self.storages.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), DistributedStorageError> {
        if event.subject.trim().is_empty() {
            return Err(DistributedStorageError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_storage(id: &str, name: &str) -> Storage {
        Storage {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = DistributedStorage::new("DistributedStorage");
        assert_eq!(m.name, "DistributedStorage");
        assert_eq!(m.storage_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_storage() {
        let mut m = DistributedStorage::new("DistributedStorage");
        m.add_storage(sample_storage("a1", "Alpha")).unwrap();
        assert_eq!(m.storage_count(), 1);
        assert!(m.get_storage("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = DistributedStorage::new("DistributedStorage");
        assert!(m.add_storage(sample_storage("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = DistributedStorage::new("DistributedStorage");
        m.add_storage(sample_storage("x", "One")).unwrap();
        assert!(m.add_storage(sample_storage("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = DistributedStorage::new("DistributedStorage");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
