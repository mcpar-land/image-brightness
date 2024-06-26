use clap::Parser;
use std::{io::Read, path::PathBuf};

#[derive(Debug, Parser)]
#[command()]
struct Cli {
	/// image file (if blank, use stdin)
	file: Option<PathBuf>,
	#[arg(long, short, default_value = "1")]
	sample_rate: usize,
}

fn main() -> Result<(), anyhow::Error> {
	let args = Cli::parse();

	let image_bytes: Vec<u8> = match args.file {
		Some(path) => std::fs::read(&path)?,
		None => std::io::stdin().bytes().collect::<Result<Vec<u8>, _>>()?,
	};

	let res = image_brightness::image_brightness(&image_bytes, args.sample_rate)?;

	println!("{}", res);

	Ok(())
}
