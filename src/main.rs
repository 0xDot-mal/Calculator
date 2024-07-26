use std::io;
fn logo(){
    let my_ascii_art = r#"
    _________        .__               .__          __                
    \_   ___ \_____  |  |   ____  __ __|  | _____ _/  |_  ___________ 
    /    \  \/\__  \ |  | _/ ___\|  |  \  | \__  \\   __\/  _ \_  __ \
    \     \____/ __ \|  |_\  \___|  |  /  |__/ __ \|  | (  <_> )  | \/
     \______  (____  /____/\___  >____/|____(____  /__|  \____/|__|   
            \/     \/          \/                \/                    
    "#;
    println!("{}" , my_ascii_art);
}
fn main() {
    println!("===================================");
    logo();
    println!("===================================");
    println!("===================================");
    println!("== Choose your operator symbol : ==");
    println!("== Addition          =>     +    ==");
    println!("== Subtraction       =>     -    ==");
    println!("== Multiplication    =>     *    ==");
    println!("== Division          =>     /    ==");
    println!("== Modulo            =>     %    ==");
    println!("===================================");

    let mut num_one = String::new();
    let mut num_two = String::new();
    let mut op = String::new();

    println!("Enter a number: ");
    io::stdin().read_line(&mut num_one).expect("Failed to read input");
    let num_one: f64 = num_one.trim().parse().expect("Invalid input. Please enter a number.");

    println!("Enter your operator symbol: ");
    io::stdin().read_line(&mut op).expect("Failed to read input");
    let op = op.trim();

    println!("Enter another number: ");
    io::stdin().read_line(&mut num_two).expect("Failed to read input");
    let num_two: f64 = num_two.trim().parse().expect("Invalid input. Please enter a number.");

    match op {
        "+" => println!("Addition result: {}", num_one + num_two),
        "-" => println!("Subtraction result: {}", num_one - num_two),
        "*" => println!("Multiplication result: {}", num_one * num_two),
        "/" => {
            if num_two != 0.0 {
                println!("Division result: {}", num_one / num_two);
            } else {
                println!("Error: Cannot divide by zero!");
            }
        }
        "%" => println!("Remainder from division result: {}", num_one % num_two),
        _ => println!("Invalid operator symbol! Please choose +, -, *, /, or %."),
    }
}
