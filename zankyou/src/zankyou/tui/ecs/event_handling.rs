mod async_queue;
mod cursor_pos;
mod flow;
mod focus;
mod queue;

pub use async_queue::AsyncEventQueue;
pub use cursor_pos::CursorPos;
pub use flow::EventFlow;
pub use focus::Focus;
pub use queue::EventQueue;

use bevy_ecs::{
	entity::Entity,
	hierarchy::{ChildOf, Children},
	query::Without,
	system::{In, InMut, InRef, Local, Query, Res, ResMut, RunSystemOnce as _},
	world::World,
};
use color_eyre::eyre;
use crossterm::event::{MouseEvent, MouseEventKind};
use ratatui::layout::Position;

use super::{
	Area, Dispatch, Event, EventDispatch, Viewport,
	ui_component::{UpdateHandle, UpdateSystemId},
};

#[derive(Debug)]
struct EntityUpdateInfo<E>
where
	E: 'static,
{
	entity: Entity,
	system: UpdateSystemId<E>,
}

#[derive(Debug)]
pub(super) struct UpdateContext<E>
where
	E: 'static,
{
	update_queue: Vec<EntityUpdateInfo<E>>,
}

impl<E> Default for UpdateContext<E> {
	fn default() -> Self {
		Self {
			update_queue: Default::default(),
		}
	}
}

impl<E> UpdateContext<E>
where
	E: 'static,
{
	pub fn handle_event(
		&mut self,
		ed: EventDispatch<E>,
		world: &mut World,
	) -> eyre::Result<Option<Event<E>>> {
		self.update_queue.clear();

		match ed.dispatch {
			Dispatch::Input => {
				world.run_system_once_with(Self::find_input_entities, &mut self.update_queue)?
			}
			Dispatch::Broadcast => {
				world.run_system_once_with(Self::find_broadcast_entities, &mut self.update_queue)?
			}
			Dispatch::Cursor { x, y } => world.run_system_cached_with(
				Self::find_cursor_entities,
				(&mut self.update_queue, &ed.event, x, y),
			)??,
			Dispatch::Target(target) => world.run_system_once_with(
				Self::find_target_entities,
				(&mut self.update_queue, target),
			)?,
		}

		Ok(match ed.dispatch {
			Dispatch::Broadcast => {
				for target in &self.update_queue {
					world.run_system_with(target.system, (target.entity, &ed.event))??;
				}
				Some(ed.event)
			}
			_ => self.target_dispatch(ed.event, world)?,
		})
	}

	fn target_dispatch(
		&self,
		event: Event<E>,
		world: &mut World,
	) -> eyre::Result<Option<Event<E>>> {
		for target in &self.update_queue {
			let flow = world.run_system_with(target.system, (target.entity, &event))??;
			match flow {
				EventFlow::Consume => {
					return Ok(None);
				}
				EventFlow::Propagate => (),
			}
		}
		Ok(Some(event))
	}

	fn find_input_entities(
		InMut(targets): InMut<Vec<EntityUpdateInfo<E>>>,
		focus: Res<Focus>,
		handles: Query<&UpdateHandle<E>>,
		parents: Query<&ChildOf>,
	) {
		Self::bubble_entities(focus.target, targets, handles, parents);
	}

	fn find_broadcast_entities(
		InMut(targets): InMut<Vec<EntityUpdateInfo<E>>>,
		components: Query<(Entity, &UpdateHandle<E>)>,
	) {
		for (entity, handle) in components {
			for system in handle.iter() {
				targets.push(EntityUpdateInfo {
					entity,
					system: *system,
				});
			}
		}
	}

	#[allow(
		clippy::type_complexity,
		reason = "separating the tuple into a typedef makes it less clear what is going on"
	)]
	fn find_cursor_entities(
		(InMut(targets), InRef(event), In(x), In(y)): (
			InMut<Vec<EntityUpdateInfo<E>>>,
			InRef<Event<E>>,
			In<u16>,
			In<u16>,
		),
		mut clicked: Local<Option<Entity>>,
		mut cursor_pos: ResMut<CursorPos>,
		broadcast_components: Query<(Entity, &UpdateHandle<E>)>,
		root_entities: Query<Entity, Without<ChildOf>>,
		areas: Query<(Option<&Area>, Option<&Children>, Option<&Viewport>)>,
		handles: Query<&UpdateHandle<E>>,
		parents: Query<&ChildOf>,
	) -> eyre::Result<()> {
		cursor_pos.x = x;
		cursor_pos.y = y;

		let target = match event {
			Event::Mouse(MouseEvent {
				kind: MouseEventKind::Up(_) | MouseEventKind::Drag(_),
				..
			}) => *clicked,
			Event::Mouse(MouseEvent {
				kind: MouseEventKind::Moved,
				..
			}) => {
				for (entity, handle) in broadcast_components {
					for system in handle.iter() {
						targets.push(EntityUpdateInfo {
							entity,
							system: *system,
						});
					}
				}
				return Ok(());
			}
			_ => Self::find_cursor_target(x, y, root_entities, areas)?,
		};
		if let Event::Mouse(mouse_event) = event {
			match mouse_event.kind {
				MouseEventKind::Down(_) => *clicked = target,
				MouseEventKind::Up(_) => *clicked = None,
				_ => (),
			}
		}

		if let Some(target) = target {
			Self::bubble_entities(target, targets, handles, parents);
		}
		Ok(())
	}

	fn find_target_entities(
		(InMut(targets), In(target)): (InMut<Vec<EntityUpdateInfo<E>>>, In<Entity>),
		handles: Query<&UpdateHandle<E>>,
		parents: Query<&ChildOf>,
	) {
		Self::bubble_entities(target, targets, handles, parents);
	}

	fn bubble_entities(
		head: Entity,
		targets: &mut Vec<EntityUpdateInfo<E>>,
		handles: Query<&UpdateHandle<E>>,
		parents: Query<&ChildOf>,
	) {
		if let Ok(handle) = handles.get(head) {
			for system in handle.iter() {
				targets.push(EntityUpdateInfo {
					entity: head,
					system: *system,
				});
			}
		}
		if let Ok(parent) = parents.get(head) {
			Self::bubble_entities(parent.parent(), targets, handles, parents);
		}
	}

	fn find_cursor_target(
		x: u16,
		y: u16,
		root_entities: Query<Entity, Without<ChildOf>>,
		areas: Query<(Option<&Area>, Option<&Children>, Option<&Viewport>)>,
	) -> eyre::Result<Option<Entity>> {
		for entity in root_entities {
			if let Some(target) = Self::find_cursor_target_inner(Position { x, y }, entity, areas)?
			{
				return Ok(Some(target));
			}
		}
		Ok(None)
	}

	/// result is based on the following "truth table"
	///
	/// | has area | in area | has children | result........... |
	/// | -------- | ------- | ------------ | ----------------- |
	/// | .......0 | ....../ | ...........0 | none............. |
	/// | .......1 | ......0 | ...........0 | none............. |
	/// | .......1 | ......1 | ...........0 | self............. |
	/// | .......0 | ....../ | ...........1 | recurse then none |
	/// | .......1 | ......0 | ...........1 | none............. |
	/// | .......1 | ......1 | ...........1 | recurse then self |
	fn find_cursor_target_inner(
		mut pos: Position,
		entity: Entity,
		areas: Query<(Option<&Area>, Option<&Children>, Option<&Viewport>)>,
	) -> eyre::Result<Option<Entity>> {
		let (area, children, viewport) = areas.get(entity)?;
		if let Some(area) = area
			&& !area.contains(pos)
		{
			Ok(None)
		} else {
			if let (Some(area), Some(viewport)) = (area, viewport) {
				pos.x = pos.x - area.x + viewport.offset.x;
				pos.y = pos.y - area.y + viewport.offset.y;
			}
			if let Some(children) = children {
				for &child in children {
					if let Some(target) = Self::find_cursor_target_inner(pos, child, areas)? {
						return Ok(Some(target));
					}
				}
			}
			if area.is_some() {
				Ok(Some(entity))
			} else {
				Ok(None)
			}
		}
	}
}
