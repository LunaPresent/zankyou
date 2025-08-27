use bevy_ecs::resource::Resource;

/// Global resource that holds the last reported cursor position.
/// This resource is automatically added to the world at the start of execution
#[derive(Debug, Resource)]
pub struct CursorPos {
	pub x: u16,
	pub y: u16,
}

impl Default for CursorPos {
	fn default() -> Self {
		Self {
			x: u16::MAX,
			y: u16::MAX,
		}
	}
}
