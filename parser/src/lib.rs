use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ParseCommandError {
	EmptyString,
	NoSubcommand,

	UnknownCategory(String),
	UnknownSubcommand(String),

	InvalidArgument(String),
	DuplicateArgument(String),
	UnexpectedArgument(String),
	MissingArgument(String),
}

#[derive(Debug, Clone)]
pub struct ArgumentParser {
	required: &'static [&'static str],
	optional: &'static [&'static str],
}

#[derive(Debug, Clone)]
pub struct Arguments<'a> {
	values: HashMap<&'a str, &'a str>,
}

impl<'a> From<HashMap<&'a str, &'a str>> for Arguments<'a> {
	fn from(value: HashMap<&'a str, &'a str>) -> Self {
		Self { values: value }
	}
}

impl Arguments<'_> {
	pub fn get<T: FromStr>(&self, name: &str) -> Result<T, ParseCommandError> {
		if let Some(&value) = self.values.get(name) {
			value
				.parse()
				.map_err(|_| ParseCommandError::InvalidArgument(name.to_string()))
		} else {
			Err(ParseCommandError::MissingArgument(name.to_string()))
		}
	}
}

impl ArgumentParser {
	pub fn new() -> Self {
		Self {
			required: &[],
			optional: &[],
		}
	}

	pub fn required(mut self, values: &'static [&'static str]) -> Self {
		self.required = values;
		self
	}

	pub fn optional(mut self, values: &'static [&'static str]) -> Self {
		self.optional = values;
		self
	}

	pub fn parse(self, string: &str) -> Result<Arguments, ParseCommandError> {
		let mut result = HashMap::new();

		for arg in string.trim().split(' ').filter(|s| !s.is_empty()) {
			let Some((key, value)) = arg.split_once('=') else {
				return Err(ParseCommandError::InvalidArgument(arg.to_string()));
			};

			if result.contains_key(key) {
				return Err(ParseCommandError::DuplicateArgument(key.to_string()));
			}

			result.insert(key, value);
		}

		for required in self.required {
			if !result.contains_key(required) {
				return Err(ParseCommandError::MissingArgument(required.to_string()));
			}
		}

		for key in result.keys() {
			if !self.required.contains(key) && !self.optional.contains(key) {
				return Err(ParseCommandError::UnexpectedArgument(key.to_string()));
			}
		}

		Ok(result.into())
	}
}
