use std::fs;

fn main() {

    // Load the file
    let file_path = "input/rucksacks.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    // Counter for the sum of all priorities
    let mut sum_of_priorities: i32 = 0;

    // Iterate through each rucksack
    for line in contents.split("\n") {

        // Calculate the size of each compartment
        let compartment_size = line.len()/2;

        // Separate the compartments
        let compartment1 = &line[0..compartment_size];
        let compartment2 = &line[compartment_size..line.len()];

        // Iterate through each item in compartment 1 and see if it also exists in compartment 2
        for item in compartment1.chars() {
            if compartment2.contains(item) {
                sum_of_priorities+= get_priority(item);
                break;
            }
        }
    }

    // Print the sum
    println!("Part 1 solution: {}", sum_of_priorities);

}

/// Get the priority of a certain character
fn get_priority(input: char) -> i32{
    return match input {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => 0
    }
}
