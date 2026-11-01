use std::sync::{Mutex, OnceLock};

use crate::storage::{LocalStore, PlayerProgress};

static ACTIVE_STORE: OnceLock<Mutex<Option<LocalStore>>> = OnceLock::new();

#[derive(Clone)]
pub struct ProgressSnapshot {
    pub total_score: u32,
    pub current_streak: u32,
    pub best_streak: u32,
    pub solved_rounds: u32,
    pub failed_rounds: u32,
    pub sound_enabled: bool,
    pub badges: Vec<String>,
}

pub fn initialize_storage(path: String) -> Result<ProgressSnapshot, String> {
    let store = LocalStore::open(&path).map_err(|error| error.to_string())?;
    let progress = store.progress().map_err(|error| error.to_string())?;
    *active_store().lock().expect("active store lock poisoned") = Some(store);
    Ok(snapshot(progress))
}

pub fn progress() -> Result<ProgressSnapshot, String> {
    let guard = active_store().lock().expect("active store lock poisoned");
    let store = guard
        .as_ref()
        .ok_or_else(|| "storage has not been initialized".to_owned())?;
    store
        .progress()
        .map(snapshot)
        .map_err(|error| error.to_string())
}

pub fn set_sound_enabled(enabled: bool) -> Result<ProgressSnapshot, String> {
    let guard = active_store().lock().expect("active store lock poisoned");
    let store = guard
        .as_ref()
        .ok_or_else(|| "storage has not been initialized".to_owned())?;
    store
        .set_sound_enabled(enabled)
        .and_then(|()| store.progress())
        .map(snapshot)
        .map_err(|error| error.to_string())
}

pub(crate) fn record_completed_round(
    score: Option<u16>,
    attempt: Option<u8>,
) -> Result<(), String> {
    let mut guard = active_store().lock().expect("active store lock poisoned");
    if let Some(store) = guard.as_mut() {
        store
            .record_round(score, attempt)
            .map_err(|error| error.to_string())?;
    }
    Ok(())
}

fn active_store() -> &'static Mutex<Option<LocalStore>> {
    ACTIVE_STORE.get_or_init(|| Mutex::new(None))
}

fn snapshot(progress: PlayerProgress) -> ProgressSnapshot {
    ProgressSnapshot {
        total_score: progress.total_score,
        current_streak: progress.current_streak,
        best_streak: progress.best_streak,
        solved_rounds: progress.solved_rounds,
        failed_rounds: progress.failed_rounds,
        sound_enabled: progress.sound_enabled,
        badges: progress.badges,
    }
}
