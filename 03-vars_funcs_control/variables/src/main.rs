fn main() {
    // variable mutability
    let mut x = 5;
    println!("[+] x is {}", x);
    x = 6;
    println!("[+] x is {}", x);

    // variable shadowing
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("[+] y is {}", y);

    println!("[+] the value of x is: {}", x);

    // typing variables
    let z: u32 = "42".parse().expect("[!] this is not a number");
    println!("[+] variable z is {}", z);

    //////////////////
    // scalar types //
    //////////////////

    // integers

    // decimal
    let a = 98_222; 
    println!("[+] a is {}", a);
    // hex
    let b = 0xff;
    println!("[+] b is {}", b);
    // octal
    let c = 0o77;
    println!("[+] c is {}", c);
    // binary
    let d = 0b1111_0000;
    println!("[+] d is {}", d);
    // byte
    let e = b'A';
    println!("[+] e is {}", e);

    // floating points
    let f1 = 2.0;       // f64
    let f2: f32 = 3.0;  // f32
    let f3 = f1 / f2;

    println!("[+] {}/{} = {}", f1, f2, f3);
    println!("[+] {}/{} = {}", f1, f2, f1/f2); // we can evaluate in print expression

    // mathematical operations
    // addition
    let sum = 5 + 10;
    // subtraction
    let diff = 10 - 5;
    // multiplication
    let prod = 5 * 10;
    // division
    let quot = 10 / 5;
    // remainder
    let modu = 10 % 5;
    println!("[+] 5 + 10 = {}", sum);
    println!("[+] 10 - 5 = {}", diff);
    println!("[+] 5 * 10 = {}", prod);
    println!("[+] 10 / 5 = {}", quot);
    println!("[+] 10 % 5 = {}", modu);

    // booleans
    let t = true;
    let f: bool = false;
    println!("[+] t: {}, f: {}", t, f);

    // characters
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';
    println!("c: {}, z: {}, heart_eyed_cat: {}", c, z, heart_eyed_cat);

    ////////////////////
    // compound types //
    ////////////////////

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // decouple tuple into individual values
    let (t1, t2, t3) = tup;
    println!("[+] tup: ({}, {}, {})", t1, t2, t3);
    // index-based element access
    let t1i = tup.0;
    let t2i = tup.1;
    let t3i = tup.2;
    println!("[+] tup: ({}, {}, {})", t1i, t2i, t3i);

    // arrays
    let a = [1, 2, 3, 4, 5];
    let months = ["jan", "feb", "mar", "apr", "may", "jun", 
                  "jul", "aug", "sep", "oct", "nov", "dec"];
    println!("[+] array: ({}, {}, {}, {}, {})", a[0], a[1], a[2], a[3], a[4]);
    println!("[+] months: ({}, {}, {}, ..., etc", months[0], months[1], months[2]);
    //let invalid = months[25]; this will build, but throw a runtime error
}
