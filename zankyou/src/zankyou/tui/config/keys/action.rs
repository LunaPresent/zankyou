pub trait Action {
	type AppEvent;

	fn into_app_event(&self) -> Self::AppEvent;
}
