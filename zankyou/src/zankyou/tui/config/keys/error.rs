use thiserror::Error;

#[derive(Debug, Error, Clone, Copy)]
pub enum KeyChordParseError {
	#[error("empty key chord string")]
	EmptyString,
	#[error("invalid modifier identifier")]
	InvalidModifier,
	#[error("invalid key identifier")]
	InvalidKey,
	#[error("key chord has no closing tag")]
	UnclosedTag,
}
