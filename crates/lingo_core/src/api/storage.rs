use std::sync::{Mutex, OnceLock};

use crate::storage::{LengthStats, LocalStore, PlayerProgress};
use crate::Round;

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
    pub length_stats: Vec<LengthStatsSnapshot>,
}

#[derive(Clone)]
pub struct LengthStatsSnapshot {
    pub word_length: u8,
    pub solved_rounds: u32,
    pub failed_rounds: u32,
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
    word_length: u8,
) -> Result<(), String> {
    let mut guard = active_store().lock().expect("active store lock poisoned");
    if let Some(store) = guard.as_mut() {
        store
            .record_round(score, attempt, word_length)
            .map_err(|error| error.to_string())?;
    }
    Ok(())
}

pub(crate) fn save_active_round(round: &Round) -> Result<(), String> {
    let guard = active_store().lock().expect("active store lock poisoned");
    if let Some(store) = guard.as_ref() {
        store
            .save_pending_round(round)
            .map_err(|error| error.to_string())?;
    }
    Ok(())
}

pub(crate) fn load_active_round() -> Result<Option<Round>, String> {
    let guard = active_store().lock().expect("active store lock poisoned");
    let store = guard
        .as_ref()
        .ok_or_else(|| "storage has not been initialized".to_owned())?;
    store
        .load_pending_round()
        .map_err(|error| error.to_string())
}

pub(crate) fn round_vocabulary(word_length: u8) -> Result<crate::storage::RoundVocabulary, String> {
    let guard = active_store().lock().expect("active store lock poisoned");
    let store = guard
        .as_ref()
        .ok_or_else(|| "storage has not been initialized".to_owned())?;
    store
        .round_vocabulary(word_length)
        .map_err(|error| error.to_string())?
        .ok_or_else(|| format!("no reviewed {word_length}-letter dictionary round is installed"))
}

pub(crate) fn definition_for(normalized_word: &str) -> Result<Option<String>, String> {
    let guard = active_store().lock().expect("active store lock poisoned");
    let store = guard
        .as_ref()
        .ok_or_else(|| "storage has not been initialized".to_owned())?;
    store
        .definition_for(normalized_word)
        .map_err(|error| error.to_string())
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
        length_stats: progress
            .length_stats
            .into_iter()
            .map(length_snapshot)
            .collect(),
    }
}

fn length_snapshot(stats: LengthStats) -> LengthStatsSnapshot {
    LengthStatsSnapshot {
        word_length: stats.word_length,
        solved_rounds: stats.solved_rounds,
        failed_rounds: stats.failed_rounds,
    }
}
