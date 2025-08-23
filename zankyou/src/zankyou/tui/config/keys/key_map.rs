use super::key_sequence::KeySequence;

#[derive(Debug)]
pub struct KeyMapping<E> {
	pub key_sequence: KeySequence,
	pub app_event: E,
}

#[derive(Debug)]
pub struct KeyMap<E>(Vec<KeyMapping<E>>);

impl<E> From<Vec<KeyMapping<E>>> for KeyMap<E> {
	fn from(value: Vec<KeyMapping<E>>) -> Self {
		Self(value).sorted()
	}
}

impl<E> KeyMap<E> {
	fn sorted(mut self) -> Self {
		self.0
			.sort_unstable_by(|lhs, rhs| lhs.key_sequence.cmp(&rhs.key_sequence));
		self
	}
}
