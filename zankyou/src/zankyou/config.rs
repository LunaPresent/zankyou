mod keys;

use keys::KeyConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
	pub keys: KeyConfig,
}
