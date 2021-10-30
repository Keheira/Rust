use std::io;
use std:: process;

fn main() {
    println!("--- Immutable example ---");
    let name = "Pascal";

    println!("{}",name);

    println!("--- Mutable example ---");
    let mut name2 = "John";

    println!("{}",name2);

    name2 = "Bob";

    println!("{}",name2);

    println!("--- Mutiple Outputs ---");
    println!("{} and {}", name, name2);

    println!("--- Call a function ---");
    let first = "John".to_string();
    let last = "Doe".to_string();
    say_name(first,last); // pass by value

    println!("--- Ownership and Borrowing --");
    let first_name = "Pascal".to_string();
    say_first_name(&first_name); // pass my reference

    println!("--- Stdin ---");
    println!("Enter your name: ");
    let mut input = String::new();

    io::stdin().read_line(&mut input); // mutable reference;
    println!("Hello {}",input);

    println!("--- Pattern Matching ---");
    print!("Enter a number: ");
    let mut num = String::new();
    io::stdin().read_line(&mut num);

    println!("--- Error Handling ---");
    print!("Enter a number: ");
    let mut first = String::new();
    io::stdin().read_line(&mut first).unwrap();
    let mut a:u32 = 0;

    match first.trim().parse(){
        Ok(val) => {
            a = val;
        }, Err(_err) => {
            println!("This is not a valid number");
        }
    }

    println!("--- Exit a program ---");
    print!("Enter a number: ");
    let mut second = String::new();
    io::stdin().read_line(&mut second).unwrap();
    let mut b:u32 = 0;

    match second.trim().parse(){
        Ok(val) => {
            b = val;
        }, Err(_err) => {
            println!("This is not a valid number");
            process::exit(1); //anything but 0
        }
    }

    println!("--- Loop ---");
    loop {
        println!("I'm am endless loop");
    }
}

// notice no return key
fn say_name(first: String, last: String) {
    println!("{} {}",first,last);
}

fn say_first_name(first: &String) {
    println!("{}",first);
}
