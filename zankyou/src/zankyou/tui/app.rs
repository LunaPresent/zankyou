use std::io;

use bevy_ecs::bundle::Bundle;
use color_eyre::eyre;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::tui::event::{AppEvent, EventSender};

use super::{
	ecs::ComponentSystem,
	event::{Event, EventDispatch, EventQueue},
	terminal::Terminal,
};

// TODO: documentation
#[derive(Debug)]
pub struct App<E>
where
	E: 'static,
{
	should_quit: bool,
	should_suspend: bool,
	events: EventQueue<E>,
	ecs: ComponentSystem<E>,
}

impl<E> App<E>
where
	E: AppEvent + Send + Sync + Clone + 'static,
{
	// TODO: documentation
	pub fn new() -> Self {
		let events = EventQueue::new();
		let ecs = ComponentSystem::new(events.sender().clone());
		Self {
			should_quit: false,
			should_suspend: false,
			events,
			ecs,
		}
	}

	/// Adds a new component to the bevy ecs
	pub fn with_component<B>(mut self, component_bundle: B) -> eyre::Result<Self>
	where
		B: Bundle,
	{
		self.ecs.add_component(component_bundle);
		self.ecs.init()?;
		Ok(self)
	}

	/// Adds a new component to the bevy ecs and focusses it
	pub fn with_main_component<B>(mut self, component_bundle: B) -> eyre::Result<Self>
	where
		B: Bundle,
	{
		let entity = self.ecs.add_component(component_bundle);
		self.ecs.set_focus(entity);
		self.ecs.init()?;
		Ok(self)
	}

	// TODO: documentation
	pub async fn run(mut self) -> eyre::Result<()> {
		self.ecs.init()?;
		let mut tui = Terminal::new()?;
		tui.enter()?;
		while !self.should_quit {
			let mut next_ed = Some(self.events.next().await?);
			while let Some(ed) = next_ed {
				let result = self.ecs.handle_event(ed)?;
				if let Some(event) = result.propagated {
					self.handle_propagated_event(&mut tui, event)?;
				}
				next_ed = result.requeued;
			}
			if self.should_suspend {
				self.should_suspend = false;
				tui.suspend()?;
				tui.clear()?;
				tui.resume()?;
			}
		}
		tui.exit()?;
		Ok(())
	}

	fn handle_propagated_event(&mut self, tui: &mut Terminal, event: Event<E>) -> eyre::Result<()> {
		match event {
			Event::Render(_) => {
				tui.try_draw(|frame| self.ecs.draw(frame).map_err(io::Error::other))?;
			}
			Event::Key(key_event) => {
				self.handle_special_keys(key_event);
			}
			Event::App(app_event) if app_event.is_quit() => self.should_quit = true,
			_ => (),
		}
		Ok(())
	}

	fn handle_special_keys(&mut self, key_event: KeyEvent) {
		match key_event.code {
			KeyCode::Char('c') if key_event.modifiers == KeyModifiers::CONTROL => {
				self.should_quit = true;
			}
			KeyCode::Char('z') if key_event.modifiers == KeyModifiers::CONTROL => {
				self.should_suspend = true;
			}
			_ => (),
		}
	}
}
