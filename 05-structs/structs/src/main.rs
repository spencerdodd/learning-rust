struct User {
	username: String,
	email: String,
	sign_in_count: u64,
	active: bool,
}

fn print_user(u: &User) {
	println!("[*] printing a user");
	println!("    username: {:?}", u.username);
	println!("    email: {:?}", u.email);
	println!("    sign_in_count: {:?}", u.sign_in_count);
	println!("    active: {:?}", u.active);
}

fn modify_email(u: &mut User, email: &str) {
	println!("[*] modifying email for user {:?} to {:?}", u.username, email);
	u.email = String::from(email);
}

fn create_user(u: &str, e: &str) -> User {
	let new_user = User {
		username: String::from(u),
		email: String::from(e),
		sign_in_count: 0,
		active: false,
	};
	new_user
}

fn main() {
	let mut user1 = User {
		username: String::from("coastal"),
		email: String::from("coastal@coastal.com"),
		sign_in_count: 2,
		active: false,
	};
	print_user(&user1);
	let new_email = "someone@example.com";
	modify_email(&mut user1, &new_email);
	print_user(&user1);
	let username = "username";
	let email = "email@domain.com";
	let new_user = create_user(&username, &email);
	print_user(&new_user);
	let new_user2 = User {
		username: String::from("new_user2"),
		email: String::from("new_user2@domain.com"),
		sign_in_count: new_user.sign_in_count,
		active: new_user.active,
	};
	print_user(&new_user2);
	let new_user3 = User {
		username: String::from("new_user3"),
		email: String::from("new_user3@domain.com"),
		..new_user
	};
	print_user(&new_user3);
}





