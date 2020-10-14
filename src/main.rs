use std::boxed::Box;
use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;
use sqlite;
use sqlite::Value;

#[derive(Debug, StructOpt)]
struct Opt {
	#[structopt(short, long)]
	database: PathBuf,

	#[structopt(short, long)]
	category: Option<String>
	// from: String,
	// to: String
}

fn main() -> Result<(), Box<dyn Error>> {
	let opt = Opt::from_args();

	if !opt.database.is_file() {
		Err("Provided database path either does not exist or is not a file.")?;
	}

	let connection = sqlite::open(opt.database)?;

	let category_name = match opt.category {
		Some(a) => a,
		None => {
			println!("You must specify a category with the --category flag!");
			println!("Valid categories:");
			connection.iterate("SELECT name FROM categories", |row| {
				println!("* {}", row[0].1.unwrap());
				true
			})?;

			std::process::exit(0);
		}
	};

	// find the ID of the specified category
	let mut category_id_cursor = connection.prepare("SELECT id FROM categories WHERE name = ?").unwrap().cursor();
	category_id_cursor.bind(&[Value::String(category_name)])?;
	let row = category_id_cursor.next()?;
	let category_id = row.unwrap()[0].as_integer().unwrap();

	let mut facts_cursor = connection.prepare(
		"
		SELECT activities.name, facts.description, facts.start_time, facts.end_time FROM facts
		INNER JOIN activities ON activities.id = facts.activity_id
		WHERE activities.category_id = ?
		"
	).unwrap().cursor();
	facts_cursor.bind(&[Value::Integer(category_id)])?;
	while let Some(row) = facts_cursor.next().unwrap() {
		println!("{:#?}", row);
	}

	Ok(())
}
