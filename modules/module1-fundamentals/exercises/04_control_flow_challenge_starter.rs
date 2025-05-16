use std::io;

fn main() {
    // Part 1: FizzBuzz Implementation
    println!("=== FizzBuzz Challenge ===");
    
    // TODO: Implement the FizzBuzz algorithm for numbers 1 to 20
    for i in 1..20 {
        // TODO: Check if i is divisible by both 3 and 5
        if i%3 == 0 && i%5 == 0 {
            println!("FizzBuzz");
        } else if i%3 == 0 {
            println!("Fizz");
        } else if i%5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
    
    // Part 2: Menu-driven Calculator
    println!("\n=== Calculator ===");
    
    // TODO: Create a variable to control the calculator loop
    let mut running = true;
    
    // TODO: Implement the calculator loop
    while running {
        // TODO: Show the menu options
        println!("Choose an operation:");
        println!("1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Divide");
        println!("5. Exit");
        
        // TODO: Get the user's choice
        let mut choice = String::new();
        // TODO: Read user input
        io::stdin().read_line(&mut choice).expect("Something went wrong");
        
        // TODO: Convert choice to a number (with error handling)
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };
        
        // TODO: Exit if the user chose option 5
        if choice == 5 {
            running = false;
            break;
        }
        
        // TODO: Get the two input numbers from the user
        // First number
        // TODO: Read first number
        let mut number1 = String::new();
        println!("Enter first number");
        io::stdin().read_line(&mut number1).expect("Something went wrong");
        let num1: u32 = match number1.trim().parse() {
            Ok(n1) => n1,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        
        // Second number
        // TODO: Read second number
        let mut number2 = String::new();
        println!("Enter first number");
        io::stdin().read_line(&mut number2).expect("Something went wrong");
        let num2: u32 = match number2.trim().parse() {
            Ok(n2) => n2,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };
        
        // TODO: Perform the selected operation using match or if statements
        match choice {
           1 => println!("Result: {} + {} = {}", num1, num2, num1 + num2),
           2 => println!("Result: {} - {} = {}", num1, num2, num1 - num2),
           3 => println!("Result: {} * {} = {}", num1, num2, num1 * num2),
           4 => if num2 == 0 {
            println!("Division by 0 not accepted");
           } else {
            println!("Result: {} / {} = {}", num1, num2, num1 / num2);
           },
           _ => println!("Invalid option. Please try again."),
        }
        
        // TODO: Ask if the user wants to perform another calculation
        println!("Do you want to perform another calculation? (y/n): ");
        // TODO: Read user's response
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Something went wrong");
        let choice: char = match choice.trim().parse() {
            Ok(response) => response,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };
        
        // TODO: Set running to false if the user doesn't want to continue
        if choice == 'n' {
            running = false;
        } else if choice != 'y'{
            println!("Invalid choice");
            running = false;
        }
    }
    
    println!("Thank you for using the calculator!");
}