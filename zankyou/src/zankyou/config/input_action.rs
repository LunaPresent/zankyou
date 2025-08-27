use serde::{Deserialize, Serialize};

use crate::{AppEvent, app_event::Direction, tui::config::Action};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InputAction {
	Quit,
	CursorUp,
	CursorDown,
	CursorLeft,
	CursorRight,
}

impl Action for InputAction {
	type AppEvent = AppEvent;

	fn into_app_event(&self) -> Self::AppEvent {
		match *self {
			InputAction::Quit => AppEvent::Quit,
			InputAction::CursorUp => AppEvent::MoveCursor(Direction::Up),
			InputAction::CursorDown => AppEvent::MoveCursor(Direction::Down),
			InputAction::CursorLeft => AppEvent::MoveCursor(Direction::Left),
			InputAction::CursorRight => AppEvent::MoveCursor(Direction::Right),
		}
	}
}
