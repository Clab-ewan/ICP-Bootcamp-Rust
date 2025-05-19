// 1. Processing string data with immutable references
fn get_length(my_string: &String) -> usize {
    // TODO: Return the length of the string
    my_string.len() // Replace this placeholder
}

// 2. Modifying vector data with mutable references
fn add_three_elements(my_vect: &mut Vec<i32>) {
    // TODO: Add three elements (10, 20, and 30) to the vector
    my_vect.push(10);
    my_vect.push(20);
    my_vect.push(30);
}

// 3. Processing multiple data structures
fn calculate_stats(numbers: &Vec<f64>, strings: &Vec<String>) -> (f64, i32) {
    // TODO: Calculate and return the average of the numbers vector and the count of items in the strings vector
    let mut average: f64 = 0.0;
    for i in numbers {
        average += i;
    }
    average = average / numbers.len() as f64;
    (average, strings.len() as i32) // Replace this placeholder
}

// 4. Borrowing rules demonstration
fn fix_borrowing_issues() {
    let mut data = vec![1, 2, 3];
    
    // TODO: The following code has borrowing issues. Uncomment and fix it.
    let ref1 = &mut data;
    ref1.push(4);
    let ref2 = &mut data;
    ref2.push(5);
    println!("Modified data: {:?}", data);
    
    // TODO: Fix another example of borrowing issue
    let ref3 = data.clone();
    let ref4 = &mut data;
    println!("Data length: {}", ref3.len());
    ref4.push(6);
    println!("Modified data: {:?}", data);
}

fn main() {
    // 1. Test immutable reference function
    let test_string = String::from("Hello, Rust borrowing!");
    let length = get_length(&test_string);
    println!("String length: {}", length);
    // Verify the string is still usable after passing as reference
    println!("Original string: {}", test_string);
    
    // 2. Test mutable reference function
    let mut my_vec: Vec<i32> = Vec::new();
    println!("Before function call: {:?}", my_vec);
    add_three_elements(&mut my_vec);
    println!("After function call: {:?}", my_vec);
    
    // 3. Test multiple references
    let numbers = vec![10.0, 20.0, 30.0, 40.0, 50.0];
    let words = vec![String::from("apple"), String::from("banana"), String::from("cherry")];
    let (average, count) = calculate_stats(&numbers, &words);
    println!("Average of numbers: {:.1}, Count of strings: {}", average, count);
    
    // 4. Test the fixed borrowing issues
    fix_borrowing_issues();
}