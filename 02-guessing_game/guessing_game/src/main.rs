extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!(" ---------------------------------");
    println!("////// guess the number game \\\\\\\\\\\\");
    println!("-----------------------------------");

    // generate random number
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // loop around the guessing
    loop {
        println!("[*] please input your guess below:");
        // create mutable variable to hold guess
        let mut guess = String::new();
        // read user's guess from standard input
        // returns new Stdin object
        // reads a line from stdin into the object
        //- referred to by the address of the mutable `guess`
        //- variable
        io::stdin().read_line(&mut guess)
            // catch any errors with an error message
            .expect("[!] failed to read line!");

        // now cast it to u32 (default int type)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("[*] you guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("[!] too small!"),
            Ordering::Greater => println!("[!] too big!"),
            Ordering::Equal => {
                println!("[+] correct, you win!");
                break;
            }
        }
    }
}