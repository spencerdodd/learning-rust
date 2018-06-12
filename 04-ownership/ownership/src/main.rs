fn main() {
    s();
    //copy(); // will error out due to use of s1 after copy
    clone();
    valid_copy();
    valid_change();
}

fn s() {
	let mut s = String::from("hello");
	s.push_str(", world!"); // string append
	println!("{:?}", s);
}

/*
fn copy() {
	let s1 = String::from("hello");
	let s2 = s1;
	println!("{:?}, world!", s1);
} */

// copy the heap data, along with the stack data (pointer to data)
fn clone() {
	let s1 = String::from("hello");
	let s2 = s1.clone();
	println!("s1: {:?}, s2: {:?}", s1, s2);
}

// integers are of a known size and therefore are copied fully
// on the stack (and live on the stack), so there is no
// difference in computational expense for the "shallow" copy
// and "deep" copy

// all scalar types are "Copy" type, meaning they can be used
// after they have been re-assigned to a new variable
// tuples can also be "Copy", but only if all variables it
// contains are "Copy"
fn valid_copy() {
	let x = 5;
	let y = x;
	println!("x: {:?}, y: {:?}", x, y);
}

// this will allow us to modify a variable that we pass by
// reference, in order to maintain ownership in the original
// function without having to pass the variable back from
// the called function
fn valid_change() {
	let mut s = String::from("hello");
	change(&mut s);
	println!("{:?}", s);
}

// we can mutate a value by reference by defining it as mutable, and
// passing the pointer to the mutating function with the &mut tag
fn change(some_string: &mut String) {
	some_string.push_str(", world");
}






