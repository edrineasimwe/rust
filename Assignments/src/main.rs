fn main() { 
    let income = 60_000;
    let tax = calculate_income_tax(income);
    println!("Income tax for {} is: {}", income, tax);

    // Process a vector
    let numbers = vec![1, 2, 3, 4, 5];
    let (modified_vec, sum) = process_vector(numbers);
    println!("Modified vector: {:?}", modified_vec);
    println!("Sum of elements: {}", sum);

    // Perform calculations
    let a = 10.0;
    let b = 5.0;

    match calculator(a, b, "add") {
        Ok(result) => println!("Addition result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match calculator(a, b, "subtract") {
        Ok(result) => println!("Subtraction result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match calculator(a, b, "multiply") {
        Ok(result) => println!("Multiplication result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match calculator(a, b, "divide") {
        Ok(result) => println!("Division result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match calculator(a, 0.0, "divide") {
        Ok(result) => println!("Division result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}

fn calculate_income_tax(income: u32) -> u32 {
    let tax;

    if income <= 10000 {
        tax = 0;
    } else if income > 10000 && income <= 50000 {
        tax = ((income - 10000) as f64 * 0.10) as u32;
    } else {
        tax = ((income - 50000) as f64 * 0.20 + 40_000 as f64 * 0.10) as u32;
    }

    tax
}

fn process_vector(vec: Vec<i32>) -> (Vec<i32>, i32) {
    let mut sum = 0;
    let mut modified_vec = Vec::new();

    for item in vec {
        sum += item;
        modified_vec.push(item * 2);
    }

    (modified_vec, sum)
}

fn calculator(a: f64, b: f64, operation: &str) -> Result<f64, String> {
    if operation == "add" {
        Ok(a + b)
    } else if operation == "subtract" {
        Ok(a - b)
    } else if operation == "multiply" {
        Ok(a * b)
    } else if operation == "divide" {
        if b == 0.0 {
            Err("Division by zero is not allowed".to_string())
        } else {
            Ok(a / b)
        }
    } else {
        Err("Invalid operation".to_string())
    }
}
