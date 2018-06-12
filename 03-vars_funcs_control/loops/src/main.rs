fn main() {
	while_loops();
	for_loops();
	countdown_for();
}

fn while_loops() {
	println!("// countdown with a while loop");
	let mut x = 3;
    while x != 0 {
        println!("{}", x);
        x = x - 1;
    }
    println!("LIFTOFF")
}

fn for_loops() {
	println!("// counting with a for loop");
	let a = [10, 20, 30, 40, 50];
	for value in a.iter() {
		println!("{}", value);
	}
}

fn countdown_for() {
	println!("// countdown with a for loop");
	for value in (1..4).rev() {
		println!("{:?}", value);
	}
	println!("LIFTOFF");
}