//! The sole SQLite access point for local player data and packaged dictionaries.

use rusqlite::{params, Connection, OptionalExtension, Result, Transaction};

use std::collections::HashSet;

use crate::{normalize, PersistedRound, Round};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerProgress {
    pub total_score: u32,
    pub current_streak: u32,
    pub best_streak: u32,
    pub solved_rounds: u32,
    pub failed_rounds: u32,
    pub sound_enabled: bool,
    pub badges: Vec<String>,
    pub length_stats: Vec<LengthStats>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LengthStats {
    pub word_length: u8,
    pub solved_rounds: u32,
    pub failed_rounds: u32,
}

/// The locally packaged vocabulary required to run one round.
///
/// It is deliberately loaded only from reviewed SQLite records.  The runtime
/// must never fall back to a development fixture or an online source.
pub struct RoundVocabulary {
    pub target: String,
    pub guesses: HashSet<String>,
}

pub struct LocalStore {
    connection: Connection,
}

impl LocalStore {
    pub fn open(path: &str) -> Result<Self> {
        let connection = Connection::open(path)?;
        let store = Self { connection };
        store.migrate()?;
        Ok(store)
    }

    pub fn in_memory() -> Result<Self> {
        let connection = Connection::open_in_memory()?;
        let store = Self { connection };
        store.migrate()?;
        Ok(store)
    }

    pub fn progress(&self) -> Result<PlayerProgress> {
        let (total_score, current_streak, best_streak, solved_rounds, failed_rounds, sound_enabled) = self.connection.query_row(
            "SELECT total_score, current_streak, best_streak, solved_rounds, failed_rounds, sound_enabled FROM player_progress WHERE id = 1",
            [],
            |row| Ok((row.get::<_, u32>(0)?, row.get::<_, u32>(1)?, row.get::<_, u32>(2)?, row.get::<_, u32>(3)?, row.get::<_, u32>(4)?, row.get::<_, bool>(5)?)),
        )?;
        let mut statement = self
            .connection
            .prepare("SELECT code FROM badge ORDER BY code")?;
        let badges = statement
            .query_map([], |row| row.get(0))?
            .collect::<Result<Vec<String>>>()?;
        let mut statement = self.connection.prepare(
            "SELECT word_length, solved_rounds, failed_rounds FROM length_stat ORDER BY word_length",
        )?;
        let length_stats = statement
            .query_map([], |row| {
                Ok(LengthStats {
                    word_length: row.get(0)?,
                    solved_rounds: row.get(1)?,
                    failed_rounds: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<LengthStats>>>()?;
        Ok(PlayerProgress {
            total_score,
            current_streak,
            best_streak,
            solved_rounds,
            failed_rounds,
            sound_enabled,
            badges,
            length_stats,
        })
    }

    pub fn set_sound_enabled(&self, enabled: bool) -> Result<()> {
        self.connection.execute(
            "UPDATE player_progress SET sound_enabled = ?1 WHERE id = 1",
            params![enabled],
        )?;
        Ok(())
    }

    pub fn round_vocabulary(&self, word_length: u8) -> Result<Option<RoundVocabulary>> {
        let mut guesses_statement = self.connection.prepare(
            "SELECT normalized_word
             FROM dictionary_word AS word
             WHERE word.length = ?1
               AND word.can_be_guess = 1
               AND NOT EXISTS (
                   SELECT 1 FROM dictionary_restriction AS restriction
                   WHERE restriction.normalized_word = word.normalized_word
               )",
        )?;
        let guesses = guesses_statement
            .query_map(params![word_length], |row| row.get::<_, String>(0))?
            .collect::<Result<HashSet<_>>>()?;

        let target = self
            .connection
            .query_row(
                "SELECT word.normalized_word
                 FROM dictionary_word AS word
                 WHERE word.length = ?1
                   AND word.can_be_guess = 1
                   AND word.can_be_answer = 1
                   AND length(trim(word.definition)) > 0
                   AND NOT EXISTS (
                       SELECT 1 FROM dictionary_restriction AS restriction
                       WHERE restriction.normalized_word = word.normalized_word
                   )
                 ORDER BY word.normalized_word
                 LIMIT 1",
                params![word_length],
                |row| row.get::<_, String>(0),
            )
            .optional()?;
        Ok(target.map(|target| RoundVocabulary {
            target: normalize(&target),
            guesses: guesses.into_iter().map(|word| normalize(&word)).collect(),
        }))
    }

    pub fn record_round(
        &mut self,
        score: Option<u16>,
        attempt: Option<u8>,
        word_length: u8,
    ) -> Result<PlayerProgress> {
        let transaction = self.connection.transaction()?;
        let (mut total_score, mut current_streak, mut best_streak, mut solved_rounds, mut failed_rounds): (u32, u32, u32, u32, u32) = transaction.query_row(
            "SELECT total_score, current_streak, best_streak, solved_rounds, failed_rounds FROM player_progress WHERE id = 1",
            [],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?)),
        )?;
        if let Some(score) = score {
            total_score += u32::from(score);
            current_streak += 1;
            best_streak = best_streak.max(current_streak);
            solved_rounds += 1;
            award_badges(&transaction, current_streak, solved_rounds, attempt)?;
        } else {
            current_streak = 0;
            failed_rounds += 1;
        }
        transaction.execute(
            "UPDATE player_progress SET total_score = ?1, current_streak = ?2, best_streak = ?3, solved_rounds = ?4, failed_rounds = ?5 WHERE id = 1",
            params![total_score, current_streak, best_streak, solved_rounds, failed_rounds],
        )?;
        let column = if score.is_some() {
            "solved_rounds"
        } else {
            "failed_rounds"
        };
        transaction.execute(
            &format!("UPDATE length_stat SET {column} = {column} + 1 WHERE word_length = ?1"),
            params![word_length],
        )?;
        transaction.commit()?;
        self.progress()
    }

    pub fn save_pending_round(&self, round: &Round) -> Result<()> {
        let persisted = round.persistable();
        self.connection.execute(
            "INSERT INTO pending_round (id, target, revealed_positions, attempts_used, deadline_ms, status) VALUES (1, ?1, ?2, ?3, ?4, ?5) ON CONFLICT(id) DO UPDATE SET target = excluded.target, revealed_positions = excluded.revealed_positions, attempts_used = excluded.attempts_used, deadline_ms = excluded.deadline_ms, status = excluded.status",
            params![persisted.target, persisted.revealed_positions, persisted.attempts_used, persisted.deadline_ms, persisted.status],
        )?;
        Ok(())
    }

    pub fn load_pending_round(&self) -> Result<Option<Round>> {
        let raw = self.connection.query_row(
            "SELECT target, revealed_positions, attempts_used, deadline_ms, status FROM pending_round WHERE id = 1",
            [],
            |row| Ok(PersistedRound { target: row.get(0)?, revealed_positions: row.get(1)?, attempts_used: row.get(2)?, deadline_ms: row.get(3)?, status: row.get(4)? }),
        ).optional()?;
        match raw {
            Some(raw) => Round::from_persisted(raw)
                .map(Some)
                .map_err(|_| rusqlite::Error::InvalidQuery),
            None => Ok(None),
        }
    }

    fn migrate(&self) -> Result<()> {
        self.connection.execute_batch(
            "
            PRAGMA foreign_keys = ON;
            CREATE TABLE IF NOT EXISTS player_progress (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                total_score INTEGER NOT NULL DEFAULT 0,
                current_streak INTEGER NOT NULL DEFAULT 0,
                best_streak INTEGER NOT NULL DEFAULT 0,
                solved_rounds INTEGER NOT NULL DEFAULT 0,
                failed_rounds INTEGER NOT NULL DEFAULT 0,
                sound_enabled INTEGER NOT NULL DEFAULT 1
            );
            INSERT OR IGNORE INTO player_progress (id) VALUES (1);
            CREATE TABLE IF NOT EXISTS badge (
                code TEXT PRIMARY KEY NOT NULL,
                awarded_at_ms INTEGER NOT NULL DEFAULT 0
            );
            CREATE TABLE IF NOT EXISTS length_stat (
                word_length INTEGER PRIMARY KEY CHECK (word_length BETWEEN 4 AND 6),
                solved_rounds INTEGER NOT NULL DEFAULT 0,
                failed_rounds INTEGER NOT NULL DEFAULT 0
            );
            INSERT OR IGNORE INTO length_stat (word_length) VALUES (4), (5), (6);
            CREATE TABLE IF NOT EXISTS pending_round (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                target TEXT NOT NULL,
                revealed_positions TEXT NOT NULL,
                attempts_used INTEGER NOT NULL,
                deadline_ms INTEGER NOT NULL,
                status TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS dictionary_word (
                normalized_word TEXT PRIMARY KEY NOT NULL,
                length INTEGER NOT NULL CHECK (length BETWEEN 4 AND 6),
                definition TEXT NOT NULL,
                part_of_speech TEXT,
                source TEXT NOT NULL,
                source_version TEXT NOT NULL,
                licence TEXT NOT NULL,
                difficulty INTEGER NOT NULL DEFAULT 1,
                can_be_guess INTEGER NOT NULL,
                can_be_answer INTEGER NOT NULL
            );
            CREATE TABLE IF NOT EXISTS dictionary_restriction (
                normalized_word TEXT PRIMARY KEY NOT NULL,
                reason TEXT NOT NULL
            );
            ",
        )
    }
}

fn award_badges(
    transaction: &Transaction<'_>,
    streak: u32,
    solved: u32,
    attempt: Option<u8>,
) -> Result<()> {
    let mut codes = Vec::new();
    if solved == 1 {
        codes.push("first_solved");
    }
    if solved == 100 {
        codes.push("solved_100");
    }
    if attempt == Some(1) {
        codes.push("first_attempt");
    }
    for (threshold, code) in [(5, "streak_5"), (10, "streak_10"), (25, "streak_25")] {
        if streak >= threshold {
            codes.push(code);
        }
    }
    for code in codes {
        transaction.execute(
            "INSERT OR IGNORE INTO badge (code) VALUES (?1)",
            params![code],
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progress_and_badges_are_transactional_and_idempotent() {
        let mut store = LocalStore::in_memory().unwrap();
        assert!(store.progress().unwrap().sound_enabled);
        let progress = store.record_round(Some(1_000), Some(1), 5).unwrap();
        assert_eq!(progress.total_score, 1_000);
        assert_eq!(progress.current_streak, 1);
        assert_eq!(progress.badges, vec!["first_attempt", "first_solved"]);
        let progress = store.record_round(None, None, 4).unwrap();
        assert_eq!(progress.current_streak, 0);
        assert_eq!(progress.failed_rounds, 1);
        assert_eq!(
            progress.length_stats[0],
            LengthStats {
                word_length: 4,
                solved_rounds: 0,
                failed_rounds: 1
            }
        );
        assert_eq!(
            progress.length_stats[1],
            LengthStats {
                word_length: 5,
                solved_rounds: 1,
                failed_rounds: 0
            }
        );
        store.set_sound_enabled(false).unwrap();
        assert!(!store.progress().unwrap().sound_enabled);
    }

    #[test]
    fn pending_round_round_trips_with_revealed_hints() {
        let store = LocalStore::in_memory().unwrap();
        let round = Round::new("KALEM", 123).unwrap();
        store.save_pending_round(&round).unwrap();
        let restored = store.load_pending_round().unwrap().unwrap();
        assert_eq!(restored.answer(), "KALEM");
        assert_eq!(restored.attempts_used(), 0);
        assert_eq!(restored.deadline_ms(), 20_123);
        assert_eq!(restored.revealed_positions()[0], Some('K'));
    }

    #[test]
    fn round_vocabulary_requires_reviewed_answer_and_honors_restrictions() {
        let store = LocalStore::in_memory().unwrap();
        assert!(store.round_vocabulary(4).unwrap().is_none());
        store.connection.execute(
            "INSERT INTO dictionary_word (normalized_word, length, definition, source, source_version, licence, can_be_guess, can_be_answer)
             VALUES ('KAPI', 4, 'Giriş bölümü.', 'review', 'v1', 'MPL-2.0', 1, 1)",
            [],
        ).unwrap();
        store.connection.execute(
            "INSERT INTO dictionary_word (normalized_word, length, definition, source, source_version, licence, can_be_guess, can_be_answer)
             VALUES ('KARE', 4, '', 'review', 'v1', 'MPL-2.0', 1, 1)",
            [],
        ).unwrap();
        let vocabulary = store.round_vocabulary(4).unwrap().unwrap();
        assert_eq!(vocabulary.target, "KAPI");
        assert!(vocabulary.guesses.contains("KARE"));
        store.connection.execute(
            "INSERT INTO dictionary_restriction (normalized_word, reason) VALUES ('KAPI', 'reviewed exclusion')",
            [],
        ).unwrap();
        assert!(store.round_vocabulary(4).unwrap().is_none());
    }
}
