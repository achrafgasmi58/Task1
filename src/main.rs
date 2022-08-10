extern crate rand;
use rand::Rng;
use std::io;
use std::io::Write;

fn main() {
    let random_number = rand::thread_rng().gen_range(1.00, 11.00);
    let mut input_price:f32=0.0;
    let mut input = String::new();
    let mut choice = String::new();
    let mut change:f32=0.0;

    while choice.trim() != "q" {
        choice.clear();

        println!("What do you want to do?");
        println!("(p)rint hello");
        println!("(q)uit the program");

        print!("Your choice: ");
        io::stdout().flush().expect("Cannot flush stdout");

        io::stdin()
            .read_line(&mut choice)
            .expect("Cannot read user input");

        println!();

        println!("You selected: {}", choice);
        if choice.trim() == "p" {

            println!("The product cost: {} $", random_number);
        
            loop {
        
                println!("Hey mate give us ur price");
        
                io::stdin().read_line(&mut input).expect("Not a valid string");
            
                input_price = input.trim().parse().expect("Not a valid number");
        
                if input_price <= random_number {
                   continue;
         
                }
                else {
                    change = input_price-random_number;
                    println!("This is your change {}" , change);
                    break;
                }
            }
            println!("This is your price {}", input_price );
            
        };
    }

}
