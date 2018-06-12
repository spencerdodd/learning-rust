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
}
