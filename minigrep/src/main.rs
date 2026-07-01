//!# Minigrep Binary Crate
//!
//! `minigrep` is the binary crate of the `re_minigrep` project. Contains the main function
//! where arguments from the cli is taken, and executed via the run function. The crate also
//! contains the `Config` struct which organizes the required arguments in order to extract 
//! the user's desired output.

use std::env;
use std::fs;
use std::process;
use std::error;
use minigrep_lib::{search, search_case_in};

/// the main function where arguments from the cli is extracted, and organised via the
/// `Config` Sruct, and executed via the `run` function. 

fn main(){

	//let arguments: Vec<String> = env::args().collect();

	let config_args = match Config::build(env::args()){
				Ok(val) => val,
				Err(e) => {eprintln!("Problem parsing arguments: {e}");
					  process::exit(1);}
			  };

	println!("Searching for {}", config_args.query);
	println!("In file {}", config_args.file_path);

	match run(config_args){
		Ok(val) => val,
		Err(e) => eprintln!("{e}")   
	};
}

/// The `run` function takes a `Config` Struct as parameter, reads the file path address,
/// recognises the boolean value of the desired search case, then prints the output  
/// accordingly. 
///
///# Header example
///```
/// some example notes 
///
///```
fn run(config: Config) -> Result<(), Box<dyn error::Error>>{
	let contents = fs::read_to_string(config.file_path)?;	
	let results = match config.ignore_case {
			true => search_case_in(&config.query, &contents),
			false => search(&config.query, &contents)
		     };

	for line in results{
		println!("{line}");
	}

	Ok(())
}

/// Has the ability to contain and organise the required query, file path, and required 
/// search case setting of the user.
///
/// # Some Header example
/// Definition notes.
/// ```
/// some example notes 
/// ```
/// Definition notes
///
/// # Another Header example (if neccessary)
///
/// ```
/// some example notes 
///
/// ```
struct Config{
	query: String,
	file_path: String,
	ignore_case: bool
} 

impl Config{

/// Builds a new `Config` Struct value
///
/// # Some Header example
///
/// ```
/// some example notes 
///
/// ```
	fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str>{ 
		args.next();

		let query = match args.next(){
				Some(val) => val,
				None => return Err("Didn't get a query string")
			   };	

		let file_path = match args.next(){
				Some(val) => val,
				None => return Err("Didn't get a file path")
			   };

		let ignore_case = env::var("IGNORE_CASE").is_ok();	

		Ok(Config{ query: query, file_path: file_path, ignore_case: ignore_case}) 
	}
}

