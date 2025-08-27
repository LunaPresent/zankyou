use crossterm::event::Event as CrosstermEvent;
use futures::{FutureExt, StreamExt};
use std::time::Duration;
use tokio::sync::mpsc;

use super::{Dispatch, Event, EventDispatch, EventSender};

const TPS: f64 = 8.0;
const FPS: f64 = 30.0;

pub(super) struct EventTask<E> {
	sender: mpsc::UnboundedSender<EventDispatch<E>>,
}

impl<E> EventTask<E> {
	/// Constructs a new instance of [`EventTask`].
	pub fn new(sender: mpsc::UnboundedSender<EventDispatch<E>>) -> Self {
		Self { sender }
	}

	/// Runs the event thread.
	///
	/// This function emits tick events at a fixed rate and polls for crossterm events in between.
	pub async fn run(self) -> color_eyre::Result<()> {
		let tick_rate = Duration::from_secs_f64(1.0 / TPS);
		let frame_rate = Duration::from_secs_f64(1.0 / FPS);
		let mut crossterm_events = crossterm::event::EventStream::new();
		let mut tick_interval = tokio::time::interval(tick_rate);
		let mut render_interval = tokio::time::interval(frame_rate);
		loop {
			tokio::select! {
				_ = self.sender.closed() => {
					break;
				}
				_ = tick_interval.tick() => {
					self.send(Dispatch::Broadcast, Event::Tick(tick_rate));
				}
				_ = render_interval.tick() => {
					self.send(Dispatch::Broadcast, Event::Render(frame_rate));
				}
				Some(Ok(evt)) = crossterm_events.next().fuse() => {
					self.handle_crossterm_event(evt);
				}
			};
		}
		Ok(())
	}

	fn handle_crossterm_event(&self, evt: CrosstermEvent) {
		match evt {
			CrosstermEvent::FocusGained => {
				self.send(Dispatch::Broadcast, Event::FocusGained);
			}
			CrosstermEvent::FocusLost => {
				self.send(Dispatch::Broadcast, Event::FocusLost);
			}
			CrosstermEvent::Key(key_event) => {
				self.send(Dispatch::Input, Event::Key(key_event));
			}
			CrosstermEvent::Mouse(mouse_event) => {
				self.send(
					Dispatch::Cursor {
						x: mouse_event.column,
						y: mouse_event.row,
					},
					Event::Mouse(mouse_event),
				);
			}
			CrosstermEvent::Paste(s) => {
				self.send(Dispatch::Input, Event::Paste(s));
			}
			CrosstermEvent::Resize(width, height) => {
				self.send(Dispatch::Broadcast, Event::Resize { width, height });
			}
		}
	}
}

impl<E> EventSender<E> for EventTask<E> {
	fn sender(&self) -> &mpsc::UnboundedSender<EventDispatch<E>> {
		&self.sender
	}
}
