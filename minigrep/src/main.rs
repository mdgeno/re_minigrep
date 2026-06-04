use std::env;
use std::fs;
use std::process;

fn main(){

	let arguments: Vec<String> = env::args().collect();

	let config_args = match Config::build(&arguments){
				Ok(val) => val,
				Err(e) => {println!("Problem parsing arguments: {e}");
					  process::exit(1);}
			  };

	println!("Searching for {}", config_args.query);
	println!("In file {}", config_args.file_path);

	run(config_args);
}

fn run(config: Config){
	let contents = fs::read_to_string(config.file_path).expect("should have been able to read the file");

	println!("{contents}");
}

struct Config{
	query: String,
	file_path: String
}

impl Config{
	fn build(args: &[String]) -> Result<Config, &'static str>{
		if args.len()<3{
			return Err("not enough arguments")
		}

		let query = args[1].clone();
		let file_path = args[2].clone();

		Ok(Config{ query: query, file_path: file_path}) 
	}
}

