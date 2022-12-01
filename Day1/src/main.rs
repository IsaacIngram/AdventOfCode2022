use std::fs;

fn main() {

    // Load the file
    let file_path = "resources/input.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    // An array of elves calorie amounts
    let mut elves = Vec::new();

    // The current sum to be added to by each elf
    let mut current_sum: i32 = 0;

    // Iterate through each line and add the calories to the array of elves
    for line in contents.split("\n") {
        if line != "" {
            current_sum += line.parse::<i32>().unwrap();
        } else {
            // Add the current sum to the elves array and reset the current sum
            elves.push(current_sum);
            current_sum = 0;
        }
    }

    // Sort the array
    elves.sort();

    // Get the greatest object
    println!("Part 1 Solution: {}", elves.last().unwrap());

    // Get the last 3 elements
    let length = elves.len();
    let sum = elves[length-1] + elves[length-2] + elves[length-3];

    println!("Part 2 Solution: {}", sum)

}
