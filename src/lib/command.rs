use macros::{CommandCategory, Subcommand};

#[derive(Debug, Clone, Eq, PartialEq, CommandCategory)]
pub enum Command {
	Album(AlbumCommand),
}

#[derive(Debug, Clone, Eq, PartialEq, Subcommand)]
pub enum AlbumCommand {
	List,
	ListTracks { id: usize },
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		let command = "album list".parse::<Command>().unwrap();
		assert_eq!(command, Command::Album(AlbumCommand::List));

		let command = "album list-tracks id=5".parse::<Command>().unwrap();
		assert_eq!(command, Command::Album(AlbumCommand::ListTracks { id: 5 }));
	}
}
