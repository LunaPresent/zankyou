mod action;
mod error;
mod key_chord;
mod key_map;
mod key_sequence;

pub use action::Action;
pub use key_chord::KeyChord;

use serde::Deserialize;
use serde_with::{KeyValueMap, serde_as};

use super::util::OneOrMany;
use key_map::KeyMap;
use key_sequence::KeySequence;

#[derive(Debug, Deserialize)]
struct InputMapping<A> {
	#[serde(rename = "$key$")]
	action: A,
	#[serde(flatten)]
	inputs: OneOrMany<KeySequence>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct KeyConfig<A>(#[serde_as(as = "KeyValueMap<_>")] Vec<InputMapping<A>>)
where
	for<'a> A: Deserialize<'a>;

impl<A> KeyConfig<A>
where
	for<'a> A: Deserialize<'a> + Action,
{
	fn make_key_map(&self) -> KeyMap<A::AppEvent> {
		let mut vec = Vec::with_capacity(
			self.0
				.iter()
				.map(|input_mapping| input_mapping.inputs.len())
				.sum(),
		);

		for input_mapping in &self.0 {
			for input in &input_mapping.inputs {
				vec.push(key_map::KeyMapping {
					key_sequence: input.clone(),
					app_event: input_mapping.action.into_app_event(),
				});
			}
		}

		KeyMap::from(vec)
	}
}
