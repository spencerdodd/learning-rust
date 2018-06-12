#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width >= rect.width && self.height >= rect.height
    }

    fn square(sz: u32) -> Rectangle {
        Rectangle { width: sz, height: sz }
    }
}

fn main() {
     let rect1 = Rectangle { width: 30, height: 50 };
     let rect2 = Rectangle { width: 10, height: 40 };
     let rect3 = Rectangle { width: 60, height: 45 };
     println!("[*] area of rect 1: {:?}", rect1.area());
     println!("[*] area of rect 2: {:?}", rect2.area());
     println!("[*] area of rect 3: {:?}", rect3.area());
     println!("[+] can rect1 hold rect2? {}", rect1.can_hold(&rect2));
     println!("[!] can rect1 hold rect3? {}", rect1.can_hold(&rect3));
     let sz: u32 = 5;
     let sqr1: Rectangle = Rectangle::square(sz);
     println!("[+] area of square size {}: {:?}", sz, sqr1.area());
 }