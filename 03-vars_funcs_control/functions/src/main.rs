fn main() {
	println!("[*] running main");
    another_function(3, 4);
    weird_expressions();
    let ret = return_function();
    println!("[+] return_function: {}", ret);
    let plus = plus_one(ret);
    println!("[+] plus_one: {}", plus);
    conditional(7);
    conditional_assignment(true);
}

fn another_function(x: i32, y: i32) {
	println!("[+] another function received values: x={}, y={}", x, y);
}

fn weird_expressions() {
	println!("[*] weird_expressions");
	let x = 5;
	let y = {
        let x = 3;
		x + 1
	};
	println!("[+] y: {}", y);
}

fn return_function() -> i32 {
	5
}

fn plus_one(x: i32) -> i32 {
	x + 1
}

fn conditional(x: i32) {
	if x < 10 {
		println!("[*] {} is less than 10", x);
	} else if x > 10 {
		println!("[*] {} is greater than 10", x);
	} else {
		println!("[*] {} is equal to 10", x);
	}
}

fn conditional_assignment(x: bool) {
	let y = if x {
		5
	} else {
		6
	};
	println!("[+] y is {}", y);
}