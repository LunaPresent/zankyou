use std::collections::VecDeque;

use bevy_ecs::resource::Resource;

use crate::tui::event::{Dispatch, Event, EventDispatch};

#[derive(Debug, Resource)]
pub struct EventQueue<E>(VecDeque<EventDispatch<E>>);

impl<E> Default for EventQueue<E> {
	fn default() -> Self {
		Self(VecDeque::default())
	}
}

impl<E> EventQueue<E> {
	pub fn push(&mut self, dispatch: Dispatch, app_event: E) {
		self.0.push_back(EventDispatch {
			dispatch,
			event: Event::App(app_event),
		});
	}

	pub(crate) fn pop(&mut self) -> Option<EventDispatch<E>> {
		self.0.pop_front()
	}
}
