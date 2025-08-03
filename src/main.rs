use rand::Rng;
use std::io;

fn main() {
    let limit: u8 = 10;

    let mut rng = rand::rng();
    let mut input = String::new();
    println!("Enter number of choices:");
    let mut num: usize = 0;
    loop{
        io::stdin().read_line(&mut input).expect("Failed to read input");
        match input.trim().parse(){
            Ok(n) => {
                num = n;
                break;
            },
            Err(error) => println!("Failed to read input, please enter valid number: {error}"),
        }
        input.clear();
    }
    println!("Now enter your choices manually. You have chosen to have {} choices.", num);
    let mut choices: Vec<String> = Vec::new();
    for i in 1..=num {
        input.clear();
        println!("Enter choice {}:", i);
        io::stdin().read_line(&mut input).expect("Failed to read input");
        choices.push(input.trim().to_string());
    }
    let mut counter: u8 = 1;
    loop {
        println!("\nNow moving on to making the choice for you.");
        let choice: usize = rng.random_range(0..num);
        let choice: String = choices[choice].clone();
        println!("I choose.. {}", choice);
        counter += 1;
        println!("Are you satisfied with {} as your choice? (y/n)", choice);
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        input = input.trim().to_lowercase();
        if input.starts_with('y') {
            break;
        }
        if counter == limit {
            println!(
                "\n\nNaah.. I have already done this {} times.\nYou are gonna have to go with {} as your choice",
                counter,
                choice,
            );
            break;
        }
        println!("Ahh. Understandable, trying again.");

    }
}
