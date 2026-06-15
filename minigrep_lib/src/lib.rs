pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
	let mut vec_lines = Vec::new();

	for line in contents.lines(){
		if line.contains(query) {
			vec_lines.push(line);
		}
	}

	vec_lines
}

pub fn search_case_in<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
	let mut vec_lines = Vec::new();
	let query = query.to_lowercase();
	
	for line in contents.lines(){
		if line.to_lowercase().contains(&query) {
			vec_lines.push(line);
		}
	}

	vec_lines
}

#[cfg(test)]
mod tests {
    use super::*;
/*
    	#[test]
	fn once(){
		let query = "duct";	
		let contents ="/
Rust:
safe, fast, productive.
Pick three";

	assert_eq!(vec!["safe, fast, productive."], search(query, contents));
	}*/

	#[test]
	fn case_sensitive(){
		let query = "duct";
		let contents = "/
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
	
		assert_eq!(vec!["safe, fast, productive."], search(query, contents));
	}

	#[test]
	fn case_insensitive(){
		let query = "rUsT";
		let contents = "/
Rust:
safe, fast, productive.
Pick three.
Trust me.";

		assert_eq!(vec!["Rust:", "Trust me."], search_case_in(query, contents));
	}
}
