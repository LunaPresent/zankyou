mod app_event;
mod cli;
mod component;
mod config;
mod tui;

use clap::Parser;
use cli::Cli;
use color_eyre::eyre;

use app_event::AppEvent;
use tui::app::App;

use crate::component::RootComponent;

#[tokio::main]
async fn main() -> eyre::Result<()> {
	color_eyre::install()?;
	let _ = Cli::parse();
	let app = App::<AppEvent>::new().with_focussed_component(RootComponent::default());
	app.run().await
}
