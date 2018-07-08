#[derive(Debug)]
enum IPAddrKind {
	V4(u32, u32, u32, u32),
	V6(String),
}

fn route(ip: &IPAddrKind) {
	println!("[*] routing IP address: {:?}", ip);
}

// messages as an enum of various data types
#[derive(Debug)]
enum Messages {
	Quit,
	Move(u32, u32),
	Write(String),
	ChangeColor(u32, u32, u32),
}

impl Messages {
	fn call(&self) {
		println!("[*] calling: {:?}", &self);
	}
}

// match enum
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("[+] lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("[+] quarter from {:?} state", state);
            25
        },
    }
}

#[derive(Debug)]
enum USState {
    California,
    NewYork,
}

// this one is cool because we match to the state of x (Some or None), and then
// perform an operation on the value of the contained type based on that state
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let home = 		IPAddrKind::V4(127, 0, 0, 1);
    let loopback = 	IPAddrKind::V6(String::from("::1"));

    route(&home);
    route(&loopback);

    let wr = 	Messages::Write(String::from("This is just a string"));
    let mv = 	Messages::Move(0, 1);
    let cc = 	Messages::ChangeColor(255, 255, 255);
    let qt = 	Messages::Quit;

    wr.call();
    mv.call();
    cc.call();
    qt.call();

    // now we work with Option<T> Some and None, analogs to absence of value
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    // Option<T>s are not treated like Ts, so we cannot perform operations with
    // them like we would the normal type.
    // in order to use them, we need to convert them back to T
    // this allows us to use 'null-like' values in our code without worrying
    // about using null implicitly, as the compiler won't allow us

    // enter match
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Quarter(USState::California));
    // matching Option<T>
    // when defining/assigning the variables, we can implicitly or explicitly
    // define the type of the object. 
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    let another_five: Option<i32> = Some(5);
    let another_six: Option<i32> = plus_one(another_five);
    // let fail_five: Option<i32> = Some("five"); // obvious fail

    // matching a subset of the type's value
    // here we only care about acting on 1, 3, 5, and 7. For all of the other
    // values that could match, we simply do nothing
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("[+] one"),
        3 => println!("[+] three"),
        5 => println!("[+] five"),
        7 => println!("[+] seven"),
        _ => (),
    }

}










