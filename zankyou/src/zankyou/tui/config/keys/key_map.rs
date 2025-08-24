use std::cmp::Ordering;

use derive_more::Deref;

use super::{key_chord::KeyChord, key_sequence::KeySequence};

type MapIdx = u16;

/// Denotes a mapping from [`KeySequence`] to a generic [`AppEvent`][ae]
///
/// [ae]: crate::tui::event::AppEvent
#[derive(Debug)]
pub struct KeyMapping<E> {
	/// The complete key sequence
	pub key_sequence: KeySequence,
	/// The event that should be triggered
	pub app_event: E,
}

/// A key-value map with keys of type [`KeySequence`]
/// and values representing a generic [`AppEvent`][ae]
///
/// [ae]: crate::tui::event::AppEvent
#[derive(Debug, Deref)]
pub struct KeyMap<E>(Vec<KeyMapping<E>>);

impl<E> From<Vec<KeyMapping<E>>> for KeyMap<E> {
	fn from(value: Vec<KeyMapping<E>>) -> Self {
		Self(value).sorted()
	}
}

impl<E> KeyMap<E> {
	// TODO: documentation
	pub fn match_key(&self, key_chord: KeyChord, prev_match: KeyMapMatch) -> KeyMapMatch {
		let mut match_start = prev_match.match_start as usize;
		let mut match_end = (prev_match.match_end as usize).min(self.len());
		let next_key_idx = prev_match.next_key_idx as usize;

		let match_idx = loop {
			let middle = (match_end + match_start) / 2;
			if middle >= self.len() {
				break None;
			}
			match self[middle]
				.key_sequence
				.get(next_key_idx)
				.cmp(&Some(&key_chord))
			{
				Ordering::Equal => break Some(middle),
				Ordering::Greater => {
					match_end = middle;
				}
				Ordering::Less => {
					match_start = middle + 1;
				}
			}
			if match_start > match_end {
				break None;
			}
		};

		if let Some(idx) = match_idx {
			// unless intentionally stress testing, the amount of matching mappings should be
			// reasonably small here, therefore let's linear search starting from `match_idx`

			for idx in (match_start..idx).rev() {
				if self[idx].key_sequence.get(next_key_idx) != Some(&key_chord) {
					match_start = idx + 1;
					break;
				}
			}
			for idx in (idx + 1)..match_end {
				if self[idx].key_sequence.get(next_key_idx) != Some(&key_chord) {
					match_end = idx;
					break;
				}
			}
		}

		KeyMapMatch {
			match_start: match_start as MapIdx,
			match_end: match_end as MapIdx,
			next_key_idx: prev_match.next_key_idx + 1,
		}
	}

	fn sorted(mut self) -> Self {
		self.0
			.sort_unstable_by(|lhs, rhs| lhs.key_sequence.cmp(&rhs.key_sequence));
		self
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyMapMatch {
	match_start: MapIdx,
	match_end: MapIdx,
	next_key_idx: u8,
}

impl Default for KeyMapMatch {
	fn default() -> Self {
		Self {
			match_start: 0,
			match_end: MapIdx::MAX,
			next_key_idx: 0,
		}
	}
}

impl KeyMapMatch {
	/// Creates a new "blank" match
	///
	/// This is equivalent to [`KeyMapMatch::default`]
	fn new() -> Self {
		Self::default()
	}

	/// Returns all partially or fully matching key mappings
	///
	/// A partial match is any key mapping for which the first n key chords of the key
	/// sequence are equal to all n key chords recorded in this `KeyMapMatch`, in order
	pub fn partial_matches<'a, E>(self, key_map: &'a KeyMap<E>) -> &'a [KeyMapping<E>] {
		let match_start = self.match_start as usize;
		let match_end = (self.match_end as usize).min(key_map.len());
		&key_map.0[match_start..match_end]
	}

	/// Returns all fully matching key mappings
	///
	/// This is a subset of the set returned by [`KeyMapMatch::partial_matches`],
	/// To be precise: any partial match where the key sequence has the same length as
	/// the amount of key chords recorded in this `KeyMapMatch` is a full match
	pub fn full_matches<'a, E>(self, key_map: &'a KeyMap<E>) -> &'a [KeyMapping<E>] {
		// as None is less than Some(_), shorter sequences always come at the front
		let match_start = self.match_start as usize;
		let match_end = (self.match_end as usize).min(key_map.len());
		let match_end = key_map.0[match_start..match_end]
			.iter()
			.enumerate()
			.find_map(|(i, k)| {
				(k.key_sequence.len() > self.next_key_idx as usize).then_some(i + match_start)
			})
			.unwrap_or(match_end);
		&key_map.0[match_start..match_end]
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn create_key_map() -> KeyMap<i32> {
		KeyMap::from(
			[
				"ba", "zz", "f", "b", "bb", "goo", "a", "y", "gz", "zb", "go", "goa", "za",
			]
			.iter()
			.map(|s| KeyMapping {
				key_sequence: s.parse().unwrap(),
				app_event: 0,
			})
			.collect::<Vec<_>>(),
		)
	}

	#[test]
	fn sort_on_create() {
		let key_map = create_key_map();
		let sorted = [
			"a", "b", "ba", "bb", "f", "go", "goa", "goo", "gz", "y", "za", "zb", "zz",
		];

		for (key_mapping, reference) in key_map.0.iter().zip(sorted.into_iter()) {
			assert_eq!(key_mapping.key_sequence.to_string(), reference);
		}
	}

	#[test]
	fn matcher_find_random() {
		let key_map = create_key_map();
		let key_match = key_map.match_key(KeyChord::from_char('b'), KeyMapMatch::new());

		assert_eq!(key_match.match_start, 1);
		assert_eq!(key_match.match_end, 4);
		assert_eq!(key_match.next_key_idx, 1);

		let key_match = key_map.match_key(KeyChord::from_char('a'), key_match);

		assert_eq!(key_match.match_start, 2);
		assert_eq!(key_match.match_end, 3);
		assert_eq!(key_match.next_key_idx, 2);

		let key_match = key_map.match_key(KeyChord::from_char('z'), key_match);

		assert!(key_match.match_start >= key_match.match_end);
		assert_eq!(key_match.next_key_idx, 3);
	}

	#[test]
	fn matcher_find_middle() {
		let key_map = create_key_map();
		let key_match = key_map.match_key(KeyChord::from_char('g'), KeyMapMatch::new());

		assert_eq!(key_match.match_start, 5);
		assert_eq!(key_match.match_end, 9);
		assert_eq!(key_match.next_key_idx, 1);

		let key_match = key_map.match_key(KeyChord::from_char('o'), key_match);

		assert_eq!(key_match.match_start, 5);
		assert_eq!(key_match.match_end, 8);
		assert_eq!(key_match.next_key_idx, 2);

		let key_match = key_map.match_key(KeyChord::from_char('o'), key_match);

		assert_eq!(key_match.match_start, 7);
		assert_eq!(key_match.match_end, 8);
		assert_eq!(key_match.next_key_idx, 3);

		let key_match = key_map.match_key(KeyChord::from_char('s'), key_match);

		assert!(key_match.match_start >= key_match.match_end);
		assert_eq!(key_match.next_key_idx, 4);
	}

	#[test]
	fn matcher_find_first() {
		let key_map = create_key_map();
		let key_match = key_map.match_key(KeyChord::from_char('a'), KeyMapMatch::new());

		assert_eq!(key_match.match_start, 0);
		assert_eq!(key_match.match_end, 1);
		assert_eq!(key_match.next_key_idx, 1);

		let key_match = key_map.match_key(KeyChord::from_char('a'), key_match);

		assert!(key_match.match_start >= key_match.match_end);
		assert_eq!(key_match.next_key_idx, 2);
	}

	#[test]
	fn matcher_find_last() {
		let key_map = create_key_map();
		let key_match = key_map.match_key(KeyChord::from_char('z'), KeyMapMatch::new());

		assert_eq!(key_match.match_start, 10);
		assert_eq!(key_match.match_end, 13);
		assert_eq!(key_match.next_key_idx, 1);

		let key_match = key_map.match_key(KeyChord::from_char('z'), key_match);

		assert_eq!(key_match.match_start, 12);
		assert_eq!(key_match.match_end, 13);
		assert_eq!(key_match.next_key_idx, 2);

		let key_match = key_map.match_key(KeyChord::from_char('z'), key_match);

		assert!(key_match.match_start >= key_match.match_end);
		assert_eq!(key_match.next_key_idx, 3);
	}

	#[test]
	fn partial_matches() {
		let key_map = create_key_map();
		let key_match = KeyMapMatch {
			match_start: 5,
			match_end: 8,
			next_key_idx: 2,
		};

		let matches = key_match.partial_matches(&key_map);
		assert_eq!(matches.len(), 3);
		assert_eq!(matches[0].key_sequence.to_string(), "go");
		assert_eq!(matches[1].key_sequence.to_string(), "goa");
		assert_eq!(matches[2].key_sequence.to_string(), "goo");
	}

	#[test]
	fn full_matches() {
		let key_map = create_key_map();
		let key_match = KeyMapMatch {
			match_start: 5,
			match_end: 8,
			next_key_idx: 2,
		};

		let matches = key_match.full_matches(&key_map);
		assert_eq!(matches.len(), 1);
		assert_eq!(matches[0].key_sequence.to_string(), "go");
	}
}
