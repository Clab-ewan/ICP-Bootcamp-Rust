fn main() {
    // TODO: 1. Declare an immutable integer variable
    let my_integer: u8 = 10;

    // TODO: 2. Declare a mutable float variable and modify it later
    let mut my_float: f32 = 2.5;
    
    // TODO: Modify the float value
    let new_float = my_float * my_float;  
    
    // TODO: 3. Declare a boolean variable using type inference
    let is_rust_fun = true;
    
    // TODO: 4. Declare a character variable with explicit type annotation
    let character: char = 'c';
    // TODO: 5. Perform arithmetic operations with the numeric variables
    let sum = my_integer + 10;
    let product = sum * 3;
    
    // TODO: 6. Print all variables and calculation results with appropriate labels
    println!("Integer value: {}", my_integer);
    println!("Original float value: {}", my_float);
    println!("Modified float value: {}", new_float);
    println!("Boolean value: {}", is_rust_fun);
    println!("Character value: {}", character);
    println!("Addition result: {}", sum);
    println!("Multiplication result: {}", product);
}