use std::fs;

fn main() {

    // Load the file
    let file_path = "input/sections.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    // Sum of all assignments that are flawed
    let mut flawed_assignments = 0;

    // Iterate through all assignment pairs
    for line in contents.split("\n") {
        // Parse assignments
        let separated_assignments: Vec<&str> = line.split(",").collect();
        let assignment_1: Vec<&str> = separated_assignments[0].split("-").collect();
        let assignment_2: Vec<&str> = separated_assignments[1].split("-").collect();
        let assignment_1_start = assignment_1[0].parse::<i32>().unwrap();
        let assignment_1_end = assignment_1[1].parse::<i32>().unwrap();
        let assignment_2_start = assignment_2[0].parse::<i32>().unwrap();
        let assignment_2_end = assignment_2[1].parse::<i32>().unwrap();

        // Check if one range fully contains another
        if (assignment_1_start <= assignment_2_start && assignment_1_end >= assignment_2_end) ||
            (assignment_2_start <= assignment_1_start && assignment_2_end >= assignment_1_end) {
            // Increase the number of flawed assignments
            flawed_assignments += 1;
        }
    }

    // Print output
    println!("Part 1 solution: {}", flawed_assignments);

    // Reset counter
    flawed_assignments = 0;

    // Iterate through all assignment pairs
    for line in contents.split("\n") {
        // Parse assignments
        let separated_assignments: Vec<&str> = line.split(",").collect();
        let assignment_1: Vec<&str> = separated_assignments[0].split("-").collect();
        let assignment_2: Vec<&str> = separated_assignments[1].split("-").collect();
        let assignment_1_start = assignment_1[0].parse::<i32>().unwrap();
        let assignment_1_end = assignment_1[1].parse::<i32>().unwrap();
        let assignment_2_start = assignment_2[0].parse::<i32>().unwrap();
        let assignment_2_end = assignment_2[1].parse::<i32>().unwrap();

        // Check if one assignment contains another
        if !(assignment_1_end < assignment_2_start || assignment_2_end < assignment_1_start) {
            flawed_assignments += 1;
        }
    }

    // Print output
    println!("Part 2 solution: {}", flawed_assignments);

}
