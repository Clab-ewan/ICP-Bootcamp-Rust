// TODO: 1. Define a function that adds two integers and returns the result
fn add(a: i32, b: i32) -> i32 {
    // TODO: Return the sum of a and b
    return a + b; // Replace this placeholder
}

// TODO: 2. Define a function that calculates the area of a rectangle
fn calculate_rectangle_area(width: f64, height: f64) -> f64 {
    // TODO: Return the area (width × height)
    return width*height; // Replace this placeholder
}

// TODO: 3. Define a function that checks if a number is prime
fn is_prime(number: u32) -> bool {
    // TODO: Implement the prime number check logic
    //       A number is prime if it's greater than 1 and 
    //       only divisible by 1 and itself
    // Handle cases less than 2 (not prime by definition)
    if number < 2 {
        return false;
    }

    if number == 2 {
        return true;
    }
    
    let limit: u32 = number - 1;
    
    // Start from 3 and check only odd numbers
    for i in (3..limit).step_by(2) {
        if number % i == 0 {
            return false;
        }
    }
    
    // If no divisors found, number is prime
    return true;
}

// TODO: 4. Define a function that converts Fahrenheit to Celsius
// Formula: C = (F - 32) * 5/9
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    // TODO: Implement the temperature conversion logic
    (fahrenheit - 32.0) * 5.0/9.0  // Replace this placeholder
}

fn main() {
    // TODO: Call the addition function with different values and print the results
    let sum1 = add(1, 3);
    let sum2 = add(16, 98);
    
    // TODO: Calculate and print the area of rectangles with different dimensions
    let area1 = calculate_rectangle_area(2.0, 9.9);
    let area2 = calculate_rectangle_area(16.5, 5.6);
    
    // TODO: Test your prime number checker with several numbers
    let prime_check1 = is_prime(13);
    let prime_check2 = is_prime(289);
    
    // TODO: Convert and print some temperatures from Fahrenheit to Celsius
    let celsius1 = fahrenheit_to_celsius(89.8);
    let celsius2 = fahrenheit_to_celsius(45.5);
    
    // TODO: Print all results with appropriate labels
    println!("Sum of 16 and 98 is: {}", sum2);
    println!("Area of rectangle with width 16.5 and height 5.6 is: {} square units", area2);
    println!("Is 13 a prime number? {}", prime_check1);
    println!("{}°F is equivalent to {}°C", 289, celsius1);
}