pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
	let mut vec_lines = Vec::new();

	for line in contents.lines(){
		if line.contains(query) {
			vec_lines.push(line);
		}
	}

	vec_lines
}

#[cfg(test)]
mod tests {
    use super::*;

    	#[test]
	fn once(){
		let query = "duct";	
		let contents ="/
Rust:
safe, fast, productive.
Pick three";

	assert_eq!(vec!["safe, fast, productive."], search(query, contents));
	}
}
