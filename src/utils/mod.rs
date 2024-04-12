use whoami;

pub fn print_welcome_message() {
    let username = whoami::username();
    println!("Hello {}! This is the Monkey programming language!", username);
    println!("Feel free to type in commands");
}