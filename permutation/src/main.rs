fn main() {
    println!("Hello, world!");
    println!("{:?}", permutation("Hello", "lleHo"))
}

fn permutation(str1: &str, str2: &str) -> bool {
	if str1.len() != str2.len() {
		return false;
	}

	let mut str1chars: Vec<char> = str1.chars().collect();
	let mut str2chars: Vec<char> = str2.chars().collect();

	str1chars.sort();
	str2chars.sort();

	return str1chars == str2chars;
}