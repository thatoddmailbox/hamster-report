use std::boxed::Box;
use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;
use sqlite;

#[derive(Debug, StructOpt)]
struct Opt {
	database: PathBuf,
	category: Option<String>
}

fn main() -> Result<(), Box<dyn Error>> {
	let opt = Opt::from_args();

	if !opt.database.is_file() {
		Err("Provided database path either does not exist or is not a file.")?;
	}

	let connection = sqlite::open(opt.database)?;

	if let None = opt.category {
		println!("You must specify a category!");
		println!("Valid categories:");
		connection.iterate("SELECT name FROM categories", |row| {
			println!("* {}", row[0].1.unwrap());
			true
		})?;

		return Ok(());
	}

	Ok(())
}
