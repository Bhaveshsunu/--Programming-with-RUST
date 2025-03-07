fn calculate_average(temperatures: &[i32]) -> f64 {
    let sum: i32 = temperatures.iter().sum();
    let count = temperatures.len();
    if count > 0 {
        sum as f64 / count as f64
    } else {
        0.0
    }
}

fn main() {
    // Weekly temperature readings for a week (7 days).
    let temperatures = [30, 32, 31, 29, 28, 27, 26];
    
    // Extract a slice of temperatures representing the last three days.
    let last_three_days = &temperatures[4..];  // Slice the last three days (index 4 to end)
    
    // Calculate the average temperature for the last three days.
    let average_temp = calculate_average(last_three_days);
    println!("Average temperature of the last three days: {:.2}°C", average_temp);
    
    // Demonstrate out-of-bounds access and handle it safely
    let out_of_bounds_slice = &temperatures.get(10..);  // This will return None
    
    match out_of_bounds_slice {
        Some(slice) => println!("Slice of temperatures: {:?}", slice),
        None => println!("Error: Attempted to access out-of-bounds slice!"),
    }
}

