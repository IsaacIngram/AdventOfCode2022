use std::fs;

fn main() {

    // Load the file
    let file_path = "input/rucksacks.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    // Counter for the sum of all priorities
    let mut sum_of_priorities: u32 = 0;

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

    // Print the sum for part 1
    println!("Part 1 solution: {}", sum_of_priorities);

    // Reset the sum for part 2
    sum_of_priorities = 0;

    // Iterate through every 3 lines of the file
    let lines: Vec<&str> = contents.split("\n").collect();
    for i in (0..lines.len()-2).rev().step_by(3) {

        // Declare the contents of the elf's rucksacks
        let elf_1 = lines.get(i).unwrap();
        let elf_2 = lines.get(i+1).unwrap();
        let elf_3 = lines.get(i+2).unwrap();

        // Iterate through all items in elf 1's rucksack
        for item in elf_1.chars() {
            // Check if the other two elves have the same item
            if elf_2.contains(item) && elf_3.contains(item) {
                // Increment the sum based on that items priority
                sum_of_priorities += get_priority(item);
                break;
            }
        }
    }

    // Print the output
    println!("Part 2 solution: {}", sum_of_priorities);

}

/// Get the priority of a certain character
fn get_priority(input: char) -> u32{
    match input {
        input if (input >= 'a' && input <= 'z') =>
            u32::from(input) - u32::from('a') + 1,
        input if (input >= 'A' && input <= 'Z') =>
            u32::from(input) - u32::from('A') + 27,
        _ => 0
    }
}
