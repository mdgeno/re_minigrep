pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
	vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

	#[test]
	fn once(){
		let query = "duct";	
		let contents ="/
Rust:
safe, fast, productive.
Pick three";

	assert_eq!(vec!["safe, fast, productive"], search(query, contents));
	}
}
