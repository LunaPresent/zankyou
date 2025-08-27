use bevy_ecs::{
	component::Component,
	entity::Entity,
	system::{Commands, In, InMut, Query, Res},
};
use color_eyre::eyre;
use ratatui::{
	layout::{Constraint, Layout, Margin, Rect},
	symbols,
	widgets::{Block, BorderType, Borders, Widget as _, WidgetRef as _},
};

use super::{ControlPanelComponent, LibraryComponent, NavbarComponent};
use crate::{
	config::UserConfig,
	tui::{
		config::{Config, KeyHandler},
		ecs::{Area, EntityCommandsExt as _, InitInput, InitSystem, RenderInput, RenderSystem},
	},
};

#[derive(Debug, Component)]
#[require(InitSystem::new(Self::init), RenderSystem::new(Self::render))]
pub struct RootComponent {
	control_panel: Entity,
	nav_bar: Entity,
	library: Entity,
}

impl Default for RootComponent {
	fn default() -> Self {
		Self {
			control_panel: Entity::PLACEHOLDER,
			nav_bar: Entity::PLACEHOLDER,
			library: Entity::PLACEHOLDER,
		}
	}
}

impl RootComponent {
	fn init(
		In(entity): InitInput,
		config: Res<Config<UserConfig>>,
		mut query: Query<&mut Self>,
		mut cmd: Commands,
	) -> eyre::Result<()> {
		let mut comp = query.get_mut(entity)?;
		let mut ec = cmd.entity(entity);
		ec.insert_if_new(KeyHandler::new(config.keys.generate_key_map()));
		comp.control_panel = ec.spawn_child(ControlPanelComponent::default()).id();
		comp.nav_bar = ec.spawn_child(NavbarComponent::default()).id();
		comp.library = ec.spawn_child(LibraryComponent::default()).id();

		Ok(())
	}

	fn render(
		(In(entity), InMut(buf)): RenderInput,
		query: Query<&Self>,
		mut areas: Query<&mut Area>,
	) -> eyre::Result<()> {
		let comp = query.get(entity)?;
		let area = **areas.get(entity)?;

		let [browser_area, control_panel_area] =
			Layout::vertical([Constraint::Fill(1), Constraint::Length(7)]).areas(area);
		let [navbar_area_fixed, navbar_area_dynamic, library_area] = Layout::horizontal([
			Constraint::Length(4),
			Constraint::Percentage(10),
			Constraint::Fill(1),
		])
		.areas(browser_area);
		let control_panel_area = control_panel_area.inner(Margin::new(1, 0));
		let navbar_area = navbar_area_fixed.union(navbar_area_dynamic);

		let control_panel_block = Block::new()
			.borders(Borders::ALL)
			.border_type(BorderType::Rounded);
		control_panel_block.render_ref(control_panel_area, buf);

		let navbar_block = Block::new()
			.borders(Borders::RIGHT)
			.border_type(BorderType::Plain);
		navbar_block.render_ref(navbar_area, buf);

		symbols::line::HORIZONTAL_UP.render(
			Rect::new(navbar_area.right() - 1, control_panel_area.top(), 1, 1),
			buf,
		);
		symbols::line::HORIZONTAL_DOWN.render(
			Rect::new(
				navbar_area.right() - 1,
				control_panel_area.bottom() - 1,
				1,
				1,
			),
			buf,
		);

		let control_panel_area = control_panel_block.inner(control_panel_area);
		let navbar_area = navbar_block.inner(navbar_area);

		**areas.get_mut(comp.control_panel)? = control_panel_area;
		**areas.get_mut(comp.nav_bar)? = navbar_area;
		**areas.get_mut(comp.library)? = library_area;

		Ok(())
	}
}
