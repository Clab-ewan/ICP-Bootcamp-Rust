// 1. Find the first word in a string
fn first_word(s: &str) -> &str {
    // TODO: Return the first word in the string (up to the first space or the entire string if no spaces)
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// 2. Calculate the sum of elements in an array slice
fn sum_slice(numbers: &[i32]) -> i32 {
    // TODO: Calculate and return the sum of all elements in the slice
    numbers.iter().sum()
}

// 3. Find the middle element(s) of a slice
fn middle_elements<T>(slice: &[T]) -> &[T]  {
    // TODO: Return the middle element if length is odd, or the two middle elements if length is even
    // Hint: For a generic implementation, you'll need to handle both cases
    let middle = (slice.len() - 1) / 2;
    if slice.len() % 2 == 1 {
        // For odd length, return a single-element slice containing the middle element
        &slice[middle..middle + 1]
    } else {
        // Even length - return a slice containing the two middle elements
        &slice[middle..middle + 2]
    }
}

// 4. Extract a subslice based on a condition (e.g., all positive numbers)
fn extract_positive(numbers: &[i32]) -> &[i32] {
    // TODO: Find the first continuous run of positive numbers in the slice and return it as a slice
    // If the slice starts with a positive number, return from start until the first non-positive
    // If the slice starts with a non-positive, find the first positive and return from there until the next non-positive
    // If no positives are found, return an empty slice
    let mut start: usize = 0;
    let mut end: usize = 0;
    if numbers[0] >= 0 {
        while numbers[end] >= 0 && end < numbers.len() {
            end += 1;
        }
        return &numbers[start..end];
    } else {
        while numbers[start] <= 0 && start < numbers.len(){
            start += 1;
        }
        end = start;
        while numbers[end] >= 0 && end < numbers.len() {
            end += 1;
        }
        if end == 0 && start == 0 {
            return &[];
        }
        else {
            return &numbers[start..end];
        }
    }    
}

fn main() {
    // 1. Test first_word function
    let sentence = String::from("Hello Rust slices world");
    let first = first_word(&sentence);
    println!("First word: {}", first);
    
    let empty_str = String::from("");
    let first_empty = first_word(&empty_str);
    println!("First word in empty string: '{}'", first_empty);
    
    // 2. Test sum_slice function
    let numbers = [1, 2, 3, 4, 5];
    let sum = sum_slice(&numbers);
    println!("Sum of all elements: {}", sum);
    
    let partial_sum = sum_slice(&numbers[0..3]);
    println!("Sum of first three elements: {}", partial_sum);
    
    // 3. Test middle_elements function
    let vec1 = vec![1, 2, 3, 4, 5]; // Odd length
    let mid1 = middle_elements(&vec1);
    println!("Middle element(s) of odd-length vector: {:?}", mid1);
    
    let vec2 = vec![1, 2, 3, 4]; // Even length
    let mid2 = middle_elements(&vec2);
    println!("Middle element(s) of even-length vector: {:?}", mid2);
    
    // 4. Test extract_positive function
    let mixed_numbers = [3, 5, 2, -1, -5, 8, 10, -3];
    let positive_run = extract_positive(&mixed_numbers);
    println!("First run of positive numbers: {:?}", positive_run);
    
    let negative_start = [-2, -5, 3, 4, 5, -1, 7];
    let positive_run2 = extract_positive(&negative_start);
    println!("First run of positive numbers (starting negative): {:?}", positive_run2);
}