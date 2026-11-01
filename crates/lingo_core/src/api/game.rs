use std::collections::HashSet;
use std::sync::{Mutex, OnceLock};

use crate::{normalize, GuessOutcome, Round, RoundStatus};

static ACTIVE_ROUND: OnceLock<Mutex<Option<Round>>> = OnceLock::new();

#[derive(Clone)]
pub struct RoundSnapshot {
    pub word_length: u8,
    pub attempts_used: u8,
    pub attempts_remaining: u8,
    pub deadline_ms: i64,
    pub status: String,
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
    let target = target_for_length(word_length)
        .ok_or_else(|| "word length must be 4, 5, or 6".to_owned())?;
    let round = Round::new(target, clock(now_ms)).map_err(str::to_owned)?;
    let snapshot = snapshot(&round);
    *active_round().lock().expect("active round lock poisoned") = Some(round);
    Ok(snapshot)
}

pub fn submit_guess(guess: String, now_ms: i64) -> Result<GuessSubmission, String> {
    let mut guard = active_round().lock().expect("active round lock poisoned");
    let round = guard
        .as_mut()
        .ok_or_else(|| "start a round before submitting a guess".to_owned())?;
    let outcome = round.submit_guess(&guess, &dictionary(), clock(now_ms));
    Ok(submission(outcome, round))
}

pub fn reconcile_round(now_ms: i64) -> Result<RoundSnapshot, String> {
    let mut guard = active_round().lock().expect("active round lock poisoned");
    let round = guard
        .as_mut()
        .ok_or_else(|| "start a round before reconciling time".to_owned())?;
    round.reconcile_time(clock(now_ms));
    Ok(snapshot(round))
}

fn active_round() -> &'static Mutex<Option<Round>> {
    ACTIVE_ROUND.get_or_init(|| Mutex::new(None))
}

fn clock(now_ms: i64) -> u64 {
    now_ms.max(0) as u64
}

fn target_for_length(word_length: u8) -> Option<&'static str> {
    match word_length {
        4 => Some("KAPI"),
        5 => Some("KALEM"),
        6 => Some("KELİME"),
        _ => None,
    }
}

fn dictionary() -> HashSet<String> {
    [
        "KAPI", "KARE", "KASA", "KALEM", "KALAS", "KİTAP", "KELİME", "KEMER",
    ]
    .into_iter()
    .map(normalize)
    .collect()
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
