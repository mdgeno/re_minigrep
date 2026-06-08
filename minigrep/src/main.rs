use std::env;
use std::fs;
use std::process;
use std::error;

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
	match fs::read_to_string(config.file_path){
			Ok(val) => println!("{val}"),
			Err(e) => println!("{e}")
	};	

	Ok(())
}

struct Config{
	query: String,
	file_path: String
}

impl Config{
	fn build(args: &[String]) -> Result<Config, &/*'static*/ str>{  //testing the lifetimes
		if args.len()<3{
			return Err("not enough arguments")
		}

		let query = args[1].clone();
		let file_path = args[2].clone();

		Ok(Config{ query: query, file_path: file_path}) 
	}
}

