//! Checked-in, offline pilot dictionary candidates.
//!
//! Candidate rows do not enter gameplay until their review state is changed to
//! `approved`. This keeps provenance and human approval separate from source
//! extraction.

use crate::normalize;

const CANDIDATES: &str = include_str!("../../../data/pilot-candidates.tsv");

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PilotEntry {
    pub word: String,
    pub definition: String,
    pub part_of_speech: String,
    pub difficulty: u8,
    pub review_state: String,
}

pub(crate) fn entries() -> Result<Vec<PilotEntry>, String> {
    let mut entries = Vec::new();
    for (line_number, line) in CANDIDATES.lines().enumerate().skip(1) {
        if line.trim().is_empty() {
            continue;
        }
        let columns: Vec<_> = line.split('\t').collect();
        if columns.len() != 5 {
            return Err(format!(
                "pilot row {} has invalid column count",
                line_number + 1
            ));
        }
        let word = normalize(columns[0]);
        if word != columns[0] || !(4..=6).contains(&word.chars().count()) {
            return Err(format!("pilot row {} has invalid word", line_number + 1));
        }
        let definition = columns[1].trim();
        if definition.is_empty() {
            return Err(format!("pilot row {} has no definition", line_number + 1));
        }
        let difficulty = columns[3]
            .parse::<u8>()
            .map_err(|_| format!("pilot row {} has invalid difficulty", line_number + 1))?;
        if !(1..=3).contains(&difficulty) {
            return Err(format!(
                "pilot row {} has invalid difficulty",
                line_number + 1
            ));
        }
        if !matches!(columns[4], "candidate" | "approved" | "rejected") {
            return Err(format!(
                "pilot row {} has invalid review state",
                line_number + 1
            ));
        }
        entries.push(PilotEntry {
            word,
            definition: definition.to_owned(),
            part_of_speech: columns[2].trim().to_owned(),
            difficulty,
            review_state: columns[4].to_owned(),
        });
    }
    Ok(entries)
}

pub(crate) fn approved_entries() -> Result<Vec<PilotEntry>, String> {
    entries().map(|entries| {
        entries
            .into_iter()
            .filter(|entry| entry.review_state == "approved")
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn candidates_are_well_formed_and_balanced() {
        let entries = entries().unwrap();
        assert_eq!(entries.len(), 150);
        let words: HashSet<_> = entries.iter().map(|entry| &entry.word).collect();
        assert_eq!(words.len(), 150);
        for length in 4..=6 {
            assert_eq!(
                entries
                    .iter()
                    .filter(|entry| entry.word.chars().count() == length)
                    .count(),
                50
            );
        }
    }

    #[test]
    fn approved_entries_are_balanced_after_review() {
        let entries = approved_entries().unwrap();
        assert_eq!(entries.len(), 150);
        for length in 4..=6 {
            assert_eq!(
                entries
                    .iter()
                    .filter(|entry| entry.word.chars().count() == length)
                    .count(),
                50
            );
        }
    }
}
