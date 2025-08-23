use std::{fmt, str::FromStr};

use serde_with::{DeserializeFromStr, SerializeDisplay};
use smallvec::SmallVec;

use super::error::KeyChordParseError;
use super::key_chord::KeyChord;

#[derive(
	Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, SerializeDisplay, DeserializeFromStr,
)]
pub struct KeySequence(SmallVec<[KeyChord; 4]>);

impl fmt::Display for KeySequence {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		todo!()
	}
}

impl FromStr for KeySequence {
	type Err = KeyChordParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut key_chords = SmallVec::default();
		let mut in_tag = false;
		let mut chord_start = 0;

		for (idx, c) in s.char_indices() {
			if in_tag {
				if c == '>' {
					in_tag = false;
					key_chords.push(s[chord_start..idx].parse()?);
				}
			} else {
				if c == '<' {
					in_tag = true;
					chord_start = idx + 1;
				} else {
					key_chords.push(s[idx..idx + 1].parse()?);
				}
			}
		}

		if in_tag {
			Err(KeyChordParseError::UnclosedTag)
		} else {
			Ok(KeySequence(key_chords))
		}
	}
}
