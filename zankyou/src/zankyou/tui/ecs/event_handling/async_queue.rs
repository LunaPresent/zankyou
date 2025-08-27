use bevy_ecs::resource::Resource;
use tokio::sync::mpsc;

use crate::tui::event::{Dispatch, Event, EventDispatch};

#[derive(Debug, Resource)]
pub struct AsyncEventQueue<E> {
	sender: mpsc::UnboundedSender<EventDispatch<E>>,
}

impl<E> AsyncEventQueue<E> {
	pub(crate) fn new(sender: mpsc::UnboundedSender<EventDispatch<E>>) -> Self {
		Self { sender }
	}

	pub fn sender(&self) -> AsyncSender<E> {
		AsyncSender(self.sender.clone())
	}
}

#[derive(Debug, Clone)]
pub struct AsyncSender<E>(mpsc::UnboundedSender<EventDispatch<E>>);

impl<E> AsyncSender<E> {
	pub fn send(&mut self, dispatch: Dispatch, event: E) {
		let _ = self.0.send(EventDispatch {
			dispatch,
			event: Event::App(event),
		});
	}
}
