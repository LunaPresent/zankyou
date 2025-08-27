mod app_event;
mod cli;
mod component;
mod config;
mod tui;

use std::fs;

use clap::Parser;
use cli::Cli;
use color_eyre::eyre;

use app_event::AppEvent;
use component::RootComponent;
use config::UserConfig;
use tui::{app::App, config::ConfigManager};

#[tokio::main]
async fn main() -> eyre::Result<()> {
	color_eyre::install()?;
	let _ = Cli::parse();
	let config_file = fs::read_to_string(".config/config.toml")?;
	let config: UserConfig = toml::from_str(&config_file)?;
	let app = App::<AppEvent>::new()
		.with_component(ConfigManager::from_value(config))?
		.with_main_component(RootComponent::default())?;
	app.run().await
}
