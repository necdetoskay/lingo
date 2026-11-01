use std::sync::{Mutex, OnceLock};

use crate::{GuessOutcome, Round, RoundStatus};

static ACTIVE_ROUND: OnceLock<Mutex<Option<Round>>> = OnceLock::new();

#[derive(Clone)]
pub struct RoundSnapshot {
    pub word_length: u8,
    pub attempts_used: u8,
    pub attempts_remaining: u8,
    pub deadline_ms: i64,
    pub status: String,
    pub answer: Option<String>,
    pub revealed_letters: Vec<Option<String>>,
}

#[derive(Clone)]
pub struct GuessSubmission {
    pub result: String,
    pub feedback: Vec<u8>,
    pub score: Option<u16>,
    pub snapshot: RoundSnapshot,
}

pub fn start_round(word_length: u8, now_ms: i64) -> Result<RoundSnapshot, String> {
    if !(4..=6).contains(&word_length) {
        return Err("word length must be 4, 5, or 6".to_owned());
    }
    let vocabulary = crate::api::storage::round_vocabulary(word_length)?;
    let round = Round::new(&vocabulary.target, clock(now_ms)).map_err(str::to_owned)?;
    let snapshot = snapshot(&round);
    crate::api::storage::save_active_round(&round)?;
    *active_round().lock().expect("active round lock poisoned") = Some(round);
    Ok(snapshot)
}

pub fn submit_guess(guess: String, now_ms: i64) -> Result<GuessSubmission, String> {
    let mut guard = active_round().lock().expect("active round lock poisoned");
    let round = guard
        .as_mut()
        .ok_or_else(|| "start a round before submitting a guess".to_owned())?;
    let was_in_progress = round.status() == RoundStatus::InProgress;
    let vocabulary = crate::api::storage::round_vocabulary(round.revealed_positions().len() as u8)?;
    let outcome = round.submit_guess(&guess, &vocabulary.guesses, clock(now_ms));
    record_terminal_transition(was_in_progress, &outcome, round)?;
    crate::api::storage::save_active_round(round)?;
    Ok(submission(outcome, round))
}

pub fn reconcile_round(now_ms: i64) -> Result<RoundSnapshot, String> {
    let mut guard = active_round().lock().expect("active round lock poisoned");
    let round = guard
        .as_mut()
        .ok_or_else(|| "start a round before reconciling time".to_owned())?;
    let was_in_progress = round.status() == RoundStatus::InProgress;
    round.reconcile_time(clock(now_ms));
    if was_in_progress && round.status() == RoundStatus::Failed {
        crate::api::storage::record_completed_round(
            None,
            None,
            round.revealed_positions().len() as u8,
        )?;
    }
    crate::api::storage::save_active_round(round)?;
    Ok(snapshot(round))
}

pub fn restore_round() -> Result<Option<RoundSnapshot>, String> {
    let Some(round) = crate::api::storage::load_active_round()? else {
        return Ok(None);
    };
    let snapshot = snapshot(&round);
    *active_round().lock().expect("active round lock poisoned") = Some(round);
    Ok(Some(snapshot))
}

/// Returns a reviewed definition only after a round has reached a terminal state.
pub fn current_round_definition() -> Result<Option<String>, String> {
    let guard = active_round().lock().expect("active round lock poisoned");
    let Some(round) = guard.as_ref() else {
        return Ok(None);
    };
    if round.status() == RoundStatus::InProgress {
        return Ok(None);
    }
    crate::api::storage::definition_for(&round.answer())
}

fn record_terminal_transition(
    was_in_progress: bool,
    outcome: &GuessOutcome,
    round: &Round,
) -> Result<(), String> {
    if !was_in_progress || round.status() == RoundStatus::InProgress {
        return Ok(());
    }
    let (score, attempt) = match outcome {
        GuessOutcome::Accepted { score, .. } => (*score, Some(round.attempts_used())),
        GuessOutcome::TimedOut | GuessOutcome::InvalidWord | GuessOutcome::RoundComplete => {
            (None, None)
        }
    };
    crate::api::storage::record_completed_round(
        score,
        attempt,
        round.revealed_positions().len() as u8,
    )
}

fn active_round() -> &'static Mutex<Option<Round>> {
    ACTIVE_ROUND.get_or_init(|| Mutex::new(None))
}

fn clock(now_ms: i64) -> u64 {
    now_ms.max(0) as u64
}

fn snapshot(round: &Round) -> RoundSnapshot {
    RoundSnapshot {
        word_length: round.revealed_positions().len() as u8,
        attempts_used: round.attempts_used(),
        attempts_remaining: crate::ATTEMPT_LIMIT - round.attempts_used(),
        deadline_ms: round.deadline_ms() as i64,
        status: match round.status() {
            RoundStatus::InProgress => "in_progress",
            RoundStatus::Won => "won",
            RoundStatus::Failed => "failed",
        }
        .to_owned(),
        answer: match round.status() {
            RoundStatus::InProgress => None,
            RoundStatus::Won | RoundStatus::Failed => Some(round.answer()),
        },
        revealed_letters: round
            .revealed_positions()
            .iter()
            .map(|letter| letter.map(|value| value.to_string()))
            .collect(),
    }
}

fn submission(outcome: GuessOutcome, round: &Round) -> GuessSubmission {
    let (result, feedback, score) = match outcome {
        GuessOutcome::InvalidWord => ("invalid_word", Vec::new(), None),
        GuessOutcome::Accepted { feedback, score } => (
            "accepted",
            feedback
                .into_iter()
                .map(|tile| match tile {
                    crate::TileFeedback::Absent => 0,
                    crate::TileFeedback::Present => 1,
                    crate::TileFeedback::Correct => 2,
                })
                .collect(),
            score,
        ),
        GuessOutcome::TimedOut => ("timed_out", Vec::new(), None),
        GuessOutcome::RoundComplete => ("round_complete", Vec::new(), None),
    };
    GuessSubmission {
        result: result.to_owned(),
        feedback,
        score,
        snapshot: snapshot(round),
    }
}
