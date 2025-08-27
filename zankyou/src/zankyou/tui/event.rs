mod queue;
mod sender;
mod task;

use std::time::Duration;

use bevy_ecs::entity::Entity;
use crossterm::event::{KeyEvent, MouseEvent};

pub(super) use queue::EventQueue;
pub(super) use sender::EventSender;

#[derive(Debug, Clone)]
pub struct EventDispatch<E> {
	pub dispatch: Dispatch,
	pub event: Event<E>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dispatch {
	Input,
	Broadcast,
	Cursor {
		x: u16,
		y: u16,
	},
	#[allow(
		dead_code,
		reason = "this is not constructed by system events, but may be used in app logic"
	)]
	Target(Entity),
}

pub trait AppEvent {
	fn is_quit(&self) -> bool;
}

#[derive(Debug, Clone)]
pub enum Event<E> {
	Tick(Duration),
	Render(Duration),
	App(E),
	FocusGained,
	FocusLost,
	Key(KeyEvent),
	Mouse(MouseEvent),
	#[allow(dead_code, reason = "inner value to be used by app logic")]
	Paste(String),
	#[allow(dead_code, reason = "inner value to be used by app logic")]
	Resize {
		width: u16,
		height: u16,
	},
}
