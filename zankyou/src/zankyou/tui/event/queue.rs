use color_eyre::eyre::OptionExt;
use tokio::sync::mpsc;

use super::{EventDispatch, EventSender};

#[derive(Debug)]
pub(crate) struct EventQueue<E> {
	sender: mpsc::UnboundedSender<EventDispatch<E>>,
	receiver: mpsc::UnboundedReceiver<EventDispatch<E>>,
}

impl<E: Send + 'static> EventQueue<E> {
	/// Constructs a new instance of [`EventQueue`] and spawns a new thread to handle events.
	pub fn new() -> Self {
		let (sender, receiver) = mpsc::unbounded_channel();
		let actor = super::task::EventTask::new(sender.clone());
		tokio::spawn(async { actor.run().await });
		Self { sender, receiver }
	}

	/// Receives an event from the sender.
	///
	/// This function blocks until an event is received.
	///
	/// # Errors
	///
	/// This function returns an error if the sender channel is disconnected. This can happen if an
	/// error occurs in the event thread. In practice, this should not happen unless there is a
	/// problem with the underlying terminal.
	pub async fn next(&mut self) -> color_eyre::Result<EventDispatch<E>> {
		self.receiver
			.recv()
			.await
			.ok_or_eyre("Failed to receive event")
	}
}

impl<E> EventSender<E> for EventQueue<E> {
	fn sender(&self) -> &mpsc::UnboundedSender<EventDispatch<E>> {
		&self.sender
	}
}
