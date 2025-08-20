use crate::tui::event;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppEvent {
	Quit,
	MoveCursor(Direction),
}

impl event::AppEvent for AppEvent {
	fn is_quit(&self) -> bool {
		self == &Self::Quit
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}

impl Direction {
	pub fn x(self) -> i16 {
		match self {
			Self::Left => -1,
			Self::Right => 1,
			_ => 0,
		}
	}

	pub fn y(self) -> i16 {
		match self {
			Self::Up => -1,
			Self::Down => 1,
			_ => 0,
		}
	}
}
