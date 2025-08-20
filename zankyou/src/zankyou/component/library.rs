use std::iter;

use bevy_ecs::{
	component::Component,
	entity::Entity,
	system::{Commands, In, InMut, InRef, Query, ResMut},
};
use color_eyre::eyre;
use ratatui::layout::{Flex, Layout};

use super::AlbumCardComponent;
use crate::{
	app_event::AppEvent,
	tui::{
		ecs::{
			Area, EntityCommandsExt, EventFlow, Focus, InitInput, InitSystem, RenderInput,
			RenderSystem, UpdateInput, UpdateSystem, Viewport,
		},
		event::Event,
	},
};

#[derive(Debug, Component, Default)]
#[require(
	InitSystem::new(Self::init),
	UpdateSystem::<AppEvent>::new(Self::update),
	RenderSystem::new(Self::render),
	Viewport
)]
pub struct LibraryComponent {
	album_cards: Vec<Entity>,
}

impl LibraryComponent {
	fn init(
		In(entity): InitInput,
		mut focus: ResMut<Focus>,
		mut query: Query<&mut Self>,
		mut cmd: Commands,
	) -> eyre::Result<()> {
		let mut comp = query.get_mut(entity)?;
		let mut ec = cmd.entity(entity);

		comp.album_cards.reserve(50);
		for _ in 0..50 {
			comp.album_cards
				.push(ec.spawn_child(AlbumCardComponent::default()).id());
		}
		focus.target = *comp.album_cards.first().unwrap();

		Ok(())
	}

	fn update(
		(In(entity), InRef(event)): UpdateInput<AppEvent>,
		mut query: Query<&mut Viewport>,
	) -> eyre::Result<EventFlow> {
		let mut viewport = query.get_mut(entity)?;
		Ok(match event {
			Event::App(AppEvent::MoveCursor(direction)) => {
				viewport.offset.y = viewport.offset.y.saturating_add_signed(direction.y());
				EventFlow::Consume
			}
			_ => EventFlow::Propagate,
		})
	}

	fn render(
		(In(entity), InMut(_buf)): RenderInput,
		mut query: Query<(&Self, &mut Viewport)>,
		mut areas: Query<&mut Area>,
	) -> eyre::Result<()> {
		let (comp, mut viewport) = query.get_mut(entity)?;
		let area = **areas.get(entity)?;

		viewport.size.width = area.width;
		viewport.size.height = 3 * area.height;
		viewport.clamp_offset(area.as_size())?;
		let area = viewport.area();

		let card_width = 22;
		let card_height = 14;
		let horizontal_gap = 3;
		let vertical_gap = 1;
		let horizontal_fit = (area.width / (card_width + horizontal_gap)) as usize;
		let vertical_fit = (area.height / (card_height + vertical_gap)) as usize;

		let columns = Layout::horizontal(iter::repeat_n(card_width, horizontal_fit))
			.spacing(horizontal_gap)
			.flex(Flex::Center);
		let rows =
			Layout::vertical(iter::repeat_n(card_height, vertical_fit)).spacing(vertical_gap);

		for i in 0..50 {
			**areas.get_mut(comp.album_cards[i])? = Default::default();
		}
		for (y, &row) in rows.split(area).iter().enumerate() {
			for (x, &column) in columns.split(row).iter().enumerate() {
				let i = x + y * horizontal_fit;
				if i >= comp.album_cards.len() {
					break;
				}
				**areas.get_mut(comp.album_cards[i])? = column;
			}
		}

		Ok(())
	}
}
