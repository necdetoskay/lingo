//! The sole SQLite access point for local player data and packaged dictionaries.

use rusqlite::{params, Connection, Result, Transaction};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerProgress {
    pub total_score: u32,
    pub current_streak: u32,
    pub best_streak: u32,
    pub solved_rounds: u32,
    pub failed_rounds: u32,
    pub sound_enabled: bool,
    pub badges: Vec<String>,
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
        Ok(PlayerProgress {
            total_score,
            current_streak,
            best_streak,
            solved_rounds,
            failed_rounds,
            sound_enabled,
            badges,
        })
    }

    pub fn set_sound_enabled(&self, enabled: bool) -> Result<()> {
        self.connection.execute(
            "UPDATE player_progress SET sound_enabled = ?1 WHERE id = 1",
            params![enabled],
        )?;
        Ok(())
    }

    pub fn record_round(
        &mut self,
        score: Option<u16>,
        attempt: Option<u8>,
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
        transaction.commit()?;
        self.progress()
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
        let progress = store.record_round(Some(1_000), Some(1)).unwrap();
        assert_eq!(progress.total_score, 1_000);
        assert_eq!(progress.current_streak, 1);
        assert_eq!(progress.badges, vec!["first_attempt", "first_solved"]);
        let progress = store.record_round(None, None).unwrap();
        assert_eq!(progress.current_streak, 0);
        assert_eq!(progress.failed_rounds, 1);
        store.set_sound_enabled(false).unwrap();
        assert!(!store.progress().unwrap().sound_enabled);
    }
}
