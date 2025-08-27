use tokio::sync::mpsc;

use super::{Dispatch, Event, EventDispatch};

pub(crate) trait EventSender<E> {
	/// Get a reference to the sender
	///
	/// This sender can be cloned and moved into a thread to implement async replies
	fn sender(&self) -> &mpsc::UnboundedSender<EventDispatch<E>>;

	/// Queue an event to be sent to the event receiver.
	///
	/// This is useful for sending events to the event handler which will be processed by the next
	/// iteration of the application's event loop.
	fn send(&self, dispatch: Dispatch, event: Event<E>) {
		let _ = self.sender().send(EventDispatch { dispatch, event });
	}
}
