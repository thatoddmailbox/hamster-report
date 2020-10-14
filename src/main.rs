use chrono::NaiveDateTime;
use csv::Writer;
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

	// open the csv
	let mut wtr = Writer::from_path("output.csv")?;

	wtr.write_record(&["Activity", "Description", "Start", "End", "Time"])?;

	let mut facts_cursor = connection.prepare(
		"
		SELECT activities.name, facts.description, facts.start_time, facts.end_time FROM facts
		INNER JOIN activities ON activities.id = facts.activity_id
		WHERE activities.category_id = ?
		"
	).unwrap().cursor();
	facts_cursor.bind(&[Value::Integer(category_id)])?;
	while let Some(row) = facts_cursor.next().unwrap() {
		let activity_name = row[0].as_string().unwrap();
		let description = row[1].as_string().unwrap();
		let start_string = row[2].as_string().unwrap();
		let end_string = row[3].as_string().unwrap();

		let start = NaiveDateTime::parse_from_str(start_string, "%Y-%m-%d %H:%M:%S").unwrap();
		let end = NaiveDateTime::parse_from_str(end_string, "%Y-%m-%d %H:%M:%S").unwrap();

		let duration_seconds = (end - start).num_seconds();

		wtr.write_record(&[
			activity_name,
			description,
			start_string,
			end_string,
			duration_seconds.to_string().as_str()
		])?;
	}

	wtr.flush()?;

	Ok(())
}
