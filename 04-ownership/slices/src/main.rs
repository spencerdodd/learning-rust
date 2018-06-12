fn main() {
    let s = String::from("first second third");
    println!("[*] full string: {:?}", s);
    let first = first_word(&s);
    println!("[+] first word: {:?}", first);
}

// since String literals and string slices are of the same
// base type, by using &str as our input type, we can take
// in both String literals and string slices with the same
// function
fn first_word(s: &str) -> &str {
	let string_bytes = s.as_bytes();
	for (i, &item) in string_bytes.iter().enumerate() {
		if item == b' ' {
			return &s[..i];
		}
	}
	return s
}
