use std::env;
use std::fs;
use std::process;
use std::error;
use minigrep_lib::{search, search_case_in};

fn main(){

	let arguments: Vec<String> = env::args().collect();

	let config_args = match Config::build(&arguments){
				Ok(val) => val,
				Err(e) => {println!("Problem parsing arguments: {e}");
					  process::exit(1);}
			  };

	println!("Searching for {}", config_args.query);
	println!("In file {}", config_args.file_path);

	match run(config_args){
		Ok(val) => val,
		Err(e) => println!("{e}")   
	};
}

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

struct Config{
	query: String,
	file_path: String,
	ignore_case: bool
}

impl Config{
	fn build(args: &[String]) -> Result<Config, &/*'static*/ str>{  //testing the lifetimes
		if args.len()<3{
			return Err("not enough arguments")
		}

		let query = args[1].clone();
		let file_path = args[2].clone();
		let ignore_case = env::var("IGNORE_CASE").is_ok();	

		Ok(Config{ query: query, file_path: file_path, ignore_case: ignore_case}) 
	}
}

