use std::collections::HashMap;

fn main() {
    println!("{:?}", unique_chars("123"));
}

fn unique_chars(input: &str) -> bool {
	let mut hashmap: HashMap<char, usize> = HashMap::new();
	for char in input.chars() {
		let count = hashmap.entry(char).or_insert(0);
		*count += 1;
		if *count > 1 {
			return false;
		}
	}
	return true;
}
