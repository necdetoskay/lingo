//! Deterministic, UI-independent rules for the Lingo V1 game loop.

pub mod api;
mod frb_generated;
mod pilot_dictionary;
pub mod storage;

use std::collections::HashSet;

pub const ATTEMPT_LIMIT: u8 = 5;
pub const ATTEMPT_DURATION_MS: u64 = 20_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileFeedback {
    Correct,
    Present,
    Absent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoundStatus {
    InProgress,
    Won,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GuessOutcome {
    InvalidWord,
    Accepted {
        feedback: Vec<TileFeedback>,
        score: Option<u16>,
    },
    TimedOut,
    RoundComplete,
}

#[derive(Debug, Clone)]
pub struct Round {
    target: Vec<char>,
    revealed_positions: Vec<Option<char>>,
    attempts_used: u8,
    deadline_ms: u64,
    status: RoundStatus,
}

#[derive(Debug, Clone)]
pub(crate) struct PersistedRound {
    pub target: String,
    pub revealed_positions: String,
    pub attempts_used: u8,
    pub deadline_ms: u64,
    pub status: String,
}

impl Round {
    pub fn new(target: &str, now_ms: u64) -> Result<Self, &'static str> {
        let target: Vec<char> = normalize(target).chars().collect();
        if !(4..=6).contains(&target.len()) {
            return Err("target must contain 4, 5, or 6 letters");
        }
        let mut revealed_positions = vec![None; target.len()];
        revealed_positions[0] = Some(target[0]);
        Ok(Self {
            target,
            revealed_positions,
            attempts_used: 0,
            deadline_ms: now_ms + ATTEMPT_DURATION_MS,
            status: RoundStatus::InProgress,
        })
    }

    pub fn status(&self) -> RoundStatus {
        self.status.clone()
    }
    pub fn attempts_used(&self) -> u8 {
        self.attempts_used
    }
    pub fn deadline_ms(&self) -> u64 {
        self.deadline_ms
    }
    pub fn answer(&self) -> String {
        self.target.iter().collect()
    }
    pub fn revealed_positions(&self) -> &[Option<char>] {
        &self.revealed_positions
    }

    pub(crate) fn persistable(&self) -> PersistedRound {
        PersistedRound {
            target: self.answer(),
            revealed_positions: self
                .revealed_positions
                .iter()
                .map(|letter| letter.unwrap_or('_'))
                .collect(),
            attempts_used: self.attempts_used,
            deadline_ms: self.deadline_ms,
            status: match self.status {
                RoundStatus::InProgress => "in_progress",
                RoundStatus::Won => "won",
                RoundStatus::Failed => "failed",
            }
            .to_owned(),
        }
    }

    pub(crate) fn from_persisted(value: PersistedRound) -> Result<Self, &'static str> {
        let target: Vec<char> = normalize(&value.target).chars().collect();
        let revealed_positions: Vec<Option<char>> = value
            .revealed_positions
            .chars()
            .map(|letter| if letter == '_' { None } else { Some(letter) })
            .collect();
        if !(4..=6).contains(&target.len())
            || target.len() != revealed_positions.len()
            || value.attempts_used > ATTEMPT_LIMIT
            || revealed_positions.first().copied().flatten() != target.first().copied()
        {
            return Err("invalid persisted round");
        }
        let status = match value.status.as_str() {
            "in_progress" => RoundStatus::InProgress,
            "won" => RoundStatus::Won,
            "failed" => RoundStatus::Failed,
            _ => return Err("invalid persisted round status"),
        };
        Ok(Self {
            target,
            revealed_positions,
            attempts_used: value.attempts_used,
            deadline_ms: value.deadline_ms,
            status,
        })
    }

    pub fn submit_guess(
        &mut self,
        guess: &str,
        valid_words: &HashSet<String>,
        now_ms: u64,
    ) -> GuessOutcome {
        if self.status != RoundStatus::InProgress {
            return GuessOutcome::RoundComplete;
        }
        if now_ms > self.deadline_ms {
            return self.expire();
        }
        let normalized = normalize(guess);
        let letters: Vec<char> = normalized.chars().collect();
        if letters.len() != self.target.len() || !valid_words.contains(&normalized) {
            return GuessOutcome::InvalidWord;
        }
        if letters[0] != self.target[0] {
            return GuessOutcome::InvalidWord;
        }
        let feedback = evaluate(&self.target, &letters);
        self.attempts_used += 1;
        for (index, feedback) in feedback.iter().enumerate() {
            if *feedback == TileFeedback::Correct {
                self.revealed_positions[index] = Some(self.target[index]);
            }
        }
        if letters == self.target {
            self.status = RoundStatus::Won;
            return GuessOutcome::Accepted {
                feedback,
                score: Some(score_for(self.attempts_used)),
            };
        }
        if self.attempts_used == ATTEMPT_LIMIT {
            self.status = RoundStatus::Failed;
        } else {
            self.deadline_ms = now_ms + ATTEMPT_DURATION_MS;
        }
        GuessOutcome::Accepted {
            feedback,
            score: None,
        }
    }

    pub fn reconcile_time(&mut self, now_ms: u64) -> u8 {
        let mut expired = 0;
        while self.status == RoundStatus::InProgress && now_ms > self.deadline_ms {
            self.expire();
            expired += 1;
        }
        expired
    }

    fn expire(&mut self) -> GuessOutcome {
        self.attempts_used += 1;
        if self.attempts_used == ATTEMPT_LIMIT {
            self.status = RoundStatus::Failed;
        } else {
            self.deadline_ms += ATTEMPT_DURATION_MS;
        }
        GuessOutcome::TimedOut
    }
}

pub fn score_for(attempt: u8) -> u16 {
    1100 - 100 * attempt as u16
}

pub fn normalize(value: &str) -> String {
    value
        .trim()
        .chars()
        .flat_map(|character| match character {
            'i' | 'İ' => "İ".chars().collect::<Vec<_>>(),
            'ı' | 'I' => "I".chars().collect::<Vec<_>>(),
            other => other.to_uppercase().collect(),
        })
        .collect()
}

pub fn evaluate(target: &[char], guess: &[char]) -> Vec<TileFeedback> {
    let mut result = vec![TileFeedback::Absent; target.len()];
    let mut remaining = target.to_vec();
    for index in 0..target.len() {
        if guess[index] == target[index] {
            result[index] = TileFeedback::Correct;
            remaining[index] = '\0';
        }
    }
    for index in 0..target.len() {
        if result[index] != TileFeedback::Correct {
            if let Some(found) = remaining.iter().position(|letter| *letter == guess[index]) {
                result[index] = TileFeedback::Present;
                remaining[found] = '\0';
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    fn words(values: &[&str]) -> HashSet<String> {
        values.iter().map(|value| normalize(value)).collect()
    }
    #[test]
    fn repeated_letters_present_in_target_remain_yellow_regardless_of_count() {
        // Accepted UI rule: exact positions are green first; every additional
        // wrong-position repetition of a present letter remains yellow.
        assert_eq!(
            evaluate(&['A', 'L', 'E', 'M'], &['A', 'A', 'A', 'A']),
            vec![
                TileFeedback::Correct,
                TileFeedback::Present,
                TileFeedback::Present,
                TileFeedback::Present
            ]
        );
        assert_eq!(
            evaluate(&['K', 'A', 'L', 'E', 'M'], &['K', 'A', 'A', 'A', 'A']),
            vec![
                TileFeedback::Correct,
                TileFeedback::Correct,
                TileFeedback::Present,
                TileFeedback::Present,
                TileFeedback::Present
            ]
        );
        assert_eq!(
            evaluate(&['A', 'L', 'E', 'M'], &['A', 'Z', 'Z', 'Z']),
            vec![
                TileFeedback::Correct,
                TileFeedback::Absent,
                TileFeedback::Absent,
                TileFeedback::Absent
            ]
        );
    }
    #[test]
    fn first_letter_is_required_but_revealed_greens_are_hints() {
        let mut round = Round::new("KALEM", 0).unwrap();
        let result = round.submit_guess("KALAS", &words(&["KALAS"]), 1);
        assert!(matches!(result, GuessOutcome::Accepted { score: None, .. }));
        assert_eq!(round.revealed_positions()[0], Some('K'));
        assert_eq!(round.revealed_positions()[1], Some('A'));
        assert!(matches!(
            round.submit_guess("KILEM", &words(&["KILEM"]), 2),
            GuessOutcome::Accepted { score: None, .. }
        ));
        assert!(matches!(
            round.submit_guess("ALEM", &words(&["ALEM"]), 3),
            GuessOutcome::InvalidWord
        ));
    }
    #[test]
    fn invalid_guess_keeps_attempt_and_deadline() {
        let mut round = Round::new("KALEM", 0).unwrap();
        let deadline = round.deadline_ms();
        assert!(matches!(
            round.submit_guess("XXXXX", &words(&[]), 1),
            GuessOutcome::InvalidWord
        ));
        assert_eq!(round.attempts_used(), 0);
        assert_eq!(round.deadline_ms(), deadline);
    }
    #[test]
    fn scoring_matches_spec() {
        assert_eq!([1, 2, 3, 4, 5].map(score_for), [1000, 900, 800, 700, 600]);
    }
    #[test]
    fn background_reconciliation_fails_after_five_timeouts() {
        let mut round = Round::new("KALEM", 0).unwrap();
        assert_eq!(round.reconcile_time(100_001), 5);
        assert_eq!(round.status(), RoundStatus::Failed);
        assert_eq!(round.answer(), "KALEM");
    }
}
