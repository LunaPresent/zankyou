use std::time::Duration;

use bevy_ecs::{
	component::Component,
	system::{In, InRef, Query, ResMut},
};
use color_eyre::eyre;

use super::KeyMap;
use crate::tui::{
	config::{KeyChord, KeyMapMatch},
	ecs::{EventFlow, EventQueue, UpdateInput, UpdateSystem},
	event::{Dispatch, Event},
};

#[derive(Debug, Component)]
#[require(UpdateSystem::<E>::new(Self::update))]
pub struct KeyHandler<E>
where
	E: Send + Sync + Clone + 'static,
{
	key_map: KeyMap<E>,
	key_map_match: KeyMapMatch,
	timeout: Duration,
	timeoutlen: Duration,
}

impl<E> KeyHandler<E>
where
	E: Send + Sync + Clone + 'static,
{
	pub fn new(key_map: KeyMap<E>) -> Self {
		Self {
			key_map,
			key_map_match: KeyMapMatch::new(),
			timeout: Duration::ZERO,
			timeoutlen: Duration::from_secs(1),
		}
	}
}

impl<E> KeyHandler<E>
where
	E: Send + Sync + Clone + 'static,
{
	fn update(
		(In(entity), InRef(event)): UpdateInput<E>,
		mut event_queue: ResMut<EventQueue<E>>,
		mut query: Query<&mut Self>,
	) -> eyre::Result<EventFlow> {
		let mut comp = query.get_mut(entity)?;

		let flow = match event {
			Event::Tick(delta) => {
				if !comp.key_map_match.matches(&comp.key_map).is_empty() {
					comp.timeout += *delta;
				}
				if comp.timeout > comp.timeoutlen {
					let matches = comp.key_map_match.full_matches(&comp.key_map);
					for app_event in matches.iter().map(|m| m.app_event.clone()) {
						event_queue.push(Dispatch::Input, app_event);
					}
					comp.key_map_match = KeyMapMatch::new();
					comp.timeout = Duration::ZERO;
				}
				EventFlow::Propagate
			}
			Event::Key(key_event) => {
				comp.timeout = Duration::ZERO;
				let key_chord = KeyChord::from_event(*key_event);
				comp.key_map_match = comp.key_map.match_key(key_chord, comp.key_map_match);

				if comp.key_map_match.matches(&comp.key_map).is_empty() {
					EventFlow::Propagate
				} else if comp.key_map_match.partial_matches(&comp.key_map).is_empty() {
					let matches = comp.key_map_match.full_matches(&comp.key_map);
					for app_event in matches.iter().map(|m| m.app_event.clone()) {
						event_queue.push(Dispatch::Input, app_event);
					}
					comp.key_map_match = KeyMapMatch::new();
					EventFlow::Consume
				} else {
					EventFlow::Consume
				}
			}
			_ => EventFlow::Propagate,
		};
		Ok(flow)
	}
}
