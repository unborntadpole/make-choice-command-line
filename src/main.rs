use rand::Rng;
use std::io;

fn main() {
    let mut rng = rand::rng();
    let mut input = String::new();
    println!("Enter number of choices:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num: usize = input.trim().parse().expect("Failed to parse input");
    println!("Now enter your choices manually. You have chosen to have {} choices.", num);
    let mut choices: Vec<String> = Vec::new();
    for i in 1..=num {
        input.clear();
        println!("Enter choice {}:", i);
        io::stdin().read_line(&mut input).expect("Failed to read input");
        choices.push(input.trim().to_string());
    }
    loop {
        println!("\nNow moving on to making the choice for you.");
        let choice: usize = rng.random_range(0..num);
        println!("I choose.. {}", choices[choice]);
        println!("Are you satisfied with {} as your choice? (y/n)", choices[choice]);
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        input = input.trim().to_lowercase();
        if input.starts_with('y'){
            break;
        }
        println!("Ahh. Understandable, trying again.");
    }
}
