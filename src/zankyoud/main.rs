use {
	interprocess::local_socket::{
		GenericNamespaced, ListenerOptions,
		tokio::{Stream, prelude::*},
	},
	std::io,
	tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
	zankyou::Command,
};

#[tokio::main]
async fn main() -> io::Result<()> {
	let printname = "zankyoud.sock";
	let name = printname.to_ns_name::<GenericNamespaced>()?;

	let opts = ListenerOptions::new().name(name);
	let listener = opts.create_tokio()?;

	loop {
		let conn = match listener.accept().await {
			Ok(c) => c,
			Err(e) => {
				eprintln!("There was an error with an incoming connection: {e}");
				continue;
			}
		};

		tokio::spawn(async move {
			if let Err(e) = handle_conn(conn).await {
				eprintln!("Error while handling connection: {e}");
			}
		});
	}
}

async fn handle_conn(conn: Stream) -> io::Result<()> {
	let mut recver = BufReader::new(&conn);
	let mut sender = &conn;

	let mut buf = String::with_capacity(128);
	let _ = recver.read_line(&mut buf).await?;

	let result = match buf.parse::<Command>() {
		Ok(command) => format!("{:?}", command),
		Err(error) => format!("{:?}", error),
	};

	sender.write_all(&result.into_bytes()).await
}
