use std::io;

fn main() {
    // Ask user for input to determine the number of Fibonacci numbers to generate
    println!("Please specify which fibonacci number you would like to generate: ");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice: i64 = choice
        .trim()
        .parse()
        .expect("please give me correct number!");

    // Generate the Fibonacci numbers based on the user's input

    let mut a: i64 = 0;
    let mut b = 1;

    for num in 0..choice {
        if choice == 1 {
            println! {"The Fibonnaci number you are looking for is: 0"};
            break;
        } else if choice == 2 {
            println! {"The Fibonnaci number you are looking for is: 1"};
            break;
        } else {
            let c = a + b;
            a = b;
            b = c;
            if num == choice - 1 {
                println!("The Fibonnaci number you are looking for is: {c}");
            }
        }
    }
}
