use interprocess::local_socket::{GenericNamespaced, Name, Stream, ToNsName, prelude::*};
use std::io::{self, BufRead, BufReader, Write};

const NAME: &str = "zankyoud.sock";

pub fn send_bytes(data: &[u8]) -> io::Result<String> {
	let name = name()?;

	let mut connection = BufReader::new(Stream::connect(name)?);
	connection.get_mut().write_all(data)?;

	let mut buffer = String::new();
	connection.read_line(&mut buffer)?;

	Ok(buffer)
}

pub fn send_line(line: &str) -> io::Result<String> {
	let value = format!("{line}\n");
	let bytes = value.into_bytes();

	send_bytes(&bytes)
}

pub fn name() -> io::Result<Name<'static>> {
	NAME.to_ns_name::<GenericNamespaced>()
}
