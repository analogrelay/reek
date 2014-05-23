use std::io::{TcpListener, TcpStream, Listener, Acceptor};

fn client_connected(mut stream: TcpStream) {
	println!("Client {} connected", stream.peer_name().unwrap());
}

pub fn run(port: u16) {
	if port <= 0 {
		fail!("Invalid port: {}", port);
	}

	// Bind to the port
	let listener = TcpListener::bind("127.0.0.1", port).unwrap();
	let mut acceptor = listener.listen();
	for stream in acceptor.incoming() {
		match stream {
			Err(e) => { fail!("Connection failed!") }
			Ok(stream) => spawn(proc() {
				client_connected(stream);
			})
		}
	}
	drop(acceptor);
}