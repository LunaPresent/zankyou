mod input_action;

use serde::Deserialize;

use crate::tui::config::KeyConfig;
use input_action::InputAction;

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Config {
	pub keys: KeyConfig<InputAction>,
}
