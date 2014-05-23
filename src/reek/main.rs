#![crate_type="bin"]

extern crate getopts;

use std::os;
use std::vec::Vec;
use std::from_str::from_str;

use getopts::{optopt, getopts, OptGroup, usage};

mod http;

fn print_usage(opts: &[OptGroup]) {
	println!("{}", usage("reek - simple rusty http server", opts));
}

fn main() {
	let args : Vec<StrBuf> = os::args().iter()
		.map(|x| x.to_strbuf())
		.collect();
	let opts = [
		optopt("p", "port", "The port on which to listen for incoming HTTP connections", "PORT")
	];
	let matches = match getopts(args.tail(), opts) {
		Ok(m) => { m },
		Err(_) => { print_usage(opts); return; }
	};

	let port = if matches.opt_present("p") {
		let portStr = matches.opt_str("p").unwrap();
		match from_str::<u16>(portStr.as_slice()) {
			Some(i) => i,
			None => fail!("Invalid port number: {}", portStr)
		}
	} else {
		8080 as u16
	};

	println!("Listening on port {}", port);
	http::run(port);
}