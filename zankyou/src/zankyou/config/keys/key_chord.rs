use std::{fmt, str::FromStr};

use crossterm::event::{KeyCode, KeyModifiers};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use thiserror::Error;

#[derive(Debug, SerializeDisplay, DeserializeFromStr)]
pub struct KeyChord {
	pub key: KeyCode,
	pub mods: KeyModifiers,
}

impl KeyChord {
	pub fn from_char(ch: char) -> Self {
		Self {
			key: KeyCode::Char(ch),
			mods: KeyModifiers::NONE,
		}
	}

	pub fn new(key: KeyCode, mods: KeyModifiers) -> Self {
		Self { key, mods }
	}
}

impl fmt::Display for KeyChord {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.mods.contains(KeyModifiers::SUPER) {
			write!(f, "D-")?;
		}
		if self.mods.contains(KeyModifiers::ALT) {
			write!(f, "A-")?;
		}
		if self.mods.contains(KeyModifiers::CONTROL) {
			write!(f, "C-")?;
		}
		if self.mods.contains(KeyModifiers::SHIFT) {
			write!(f, "S-")?;
		}
		match self.key {
			KeyCode::Null => write!(f, "Nul"),
			KeyCode::Backspace => write!(f, "BS"),
			KeyCode::Tab => write!(f, "Tab"),
			KeyCode::Enter => write!(f, "CR"),
			KeyCode::Esc => write!(f, "Esc"),
			KeyCode::Char(' ') => write!(f, "Space"),
			KeyCode::Char('<') => write!(f, "lt"),
			KeyCode::Char('\\') => write!(f, "Bslash"),
			KeyCode::Char('|') => write!(f, "Bar"),
			KeyCode::Delete => write!(f, "Del"),
			KeyCode::Up => write!(f, "Up"),
			KeyCode::Down => write!(f, "Down"),
			KeyCode::Left => write!(f, "Left"),
			KeyCode::Right => write!(f, "Right"),
			KeyCode::Home => write!(f, "Home"),
			KeyCode::End => write!(f, "End"),
			KeyCode::PageUp => write!(f, "PageUp"),
			KeyCode::PageDown => write!(f, "PageDown"),
			KeyCode::Insert => write!(f, "Insert"),
			key => write!(f, "{}", key),
		}
	}
}

impl FromStr for KeyChord {
	type Err = KeyChordParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.chars().count() {
			0 => Err(KeyChordParseError::EmptyString),
			1 => Ok(KeyChord::from_char(s.chars().next().unwrap())),
			_ => {
				let mut mods = KeyModifiers::NONE;
				let split_i = s[..s.len() - 1]
					.rfind('-')
					.map(|i| i + '-'.len_utf8())
					.unwrap_or(0);
				let (mod_str, key_str) = s.split_at(split_i);
				for m in mod_str.split_terminator('-') {
					mods.insert(match m {
						"S" | "s" => KeyModifiers::SHIFT,
						"C" | "c" => KeyModifiers::CONTROL,
						"A" | "a" | "M" | "m" => KeyModifiers::ALT,
						"D" | "d" => KeyModifiers::SUPER,
						_ => return Err(KeyChordParseError::InvalidModifier),
					});
				}
				let key = if key_str.chars().count() == 1 {
					KeyCode::Char(key_str.chars().next().unwrap())
				} else {
					match key_str.to_lowercase().as_str() {
						"nul" => KeyCode::Null,
						"bs" => KeyCode::Backspace,
						"tab" => KeyCode::Tab,
						"cr" | "return" | "enter" | "eol" => KeyCode::Enter,
						"esc" => KeyCode::Esc,
						"space" => KeyCode::Char(' '),
						"lt" => KeyCode::Char('<'),
						"bslash" => KeyCode::Char('\\'),
						"bar" => KeyCode::Char('|'),
						"del" => KeyCode::Delete,
						"up" => KeyCode::Up,
						"down" => KeyCode::Down,
						"left" => KeyCode::Left,
						"right" => KeyCode::Right,
						"home" => KeyCode::Home,
						"end" => KeyCode::End,
						"pageup" => KeyCode::PageUp,
						"pagedown" => KeyCode::PageDown,
						"insert" => KeyCode::Insert,
						_ => return Err(KeyChordParseError::InvalidKey),
					}
				};
				Ok(KeyChord::new(key, mods))
			}
		}
	}
}

#[derive(Debug, Error, Clone, Copy)]
pub enum KeyChordParseError {
	#[error("empty string")]
	EmptyString,
	#[error("invalid modifier identifier")]
	InvalidModifier,
	#[error("invalid key identifier")]
	InvalidKey,
}
