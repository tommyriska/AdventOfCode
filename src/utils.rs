use std::fs;

/// Print the answer for a given day
///
/// # Example
///
/// ## Answer for day one, part 1
/// printAnswer(1, 1, "The answer is 42")
/// printAnswer(1, 2, "The answer is 1337")
///
pub fn printAnswer(day: u32, part: u32, answer: &str) {
    // Print the result
    println!("========== DAY {} - PART {} ==========", day, part);
    println!("{}", answer);
    println!("======================================");
}
/// Read a file into a string vector
pub fn get_input_as_lines(filepath: &str) -> Vec<String> {
    // Read the input file and convert it to a string
    let input = fs::read_to_string(filepath).unwrap();
    // Split the input string into lines
    input.lines().map(String::from).collect()
}