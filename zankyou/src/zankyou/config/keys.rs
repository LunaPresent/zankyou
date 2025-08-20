mod input_action;
mod key_chord;

use std::collections::HashMap;

use input_action::InputAction;
use key_chord::KeyChord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyConfig {
	keymap: HashMap<InputAction, KeyChord>,
}
