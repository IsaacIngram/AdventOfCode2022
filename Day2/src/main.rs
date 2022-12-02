use std::fs;

fn main() {

    // Load the file
    let file_path = "input/rounds.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    let mut score: i32 = 0;
    for line in contents.split("\n") {
        let moves: Vec<&str> = line.split_whitespace().collect();
        // Check whether we won, if it was a tie, or if we didn't win, and add points
        if round_won(moves[0], moves[1]) {
            score += 6;
        } else if round_tied(moves[0], moves[1]) {
            score += 3;
        }

        // Check what we played and add points
        if moves[1] == "X" {
            score += 1;
        } else if moves[1] == "Y" {
            score += 2;
        } else if moves[1] == "Z" {
            score += 3;
        }
    }

    println!("Part 1 solution: {}", score);

    let mut part_2_score = 0;
    for line in contents.split("\n") {
        let moves: Vec<&str> = line.split_whitespace().collect();
        // Check if we need to win, tie, or lose
        if moves[1] == "Z" {
            // Win
            part_2_score += 6;
            if moves[0] == "A" {
                part_2_score += 2;
            } else if moves[0] == "B" {
                part_2_score += 3;
            } else if moves[0] == "C" {
                part_2_score += 1;
            }
        } else if moves[1] == "Y" {
            // Tie
            part_2_score += 3;
            if moves[0] == "A" {
                part_2_score += 1;
            } else if moves[0] == "B" {
                part_2_score += 2;
            } else if moves[0] == "C" {
                part_2_score += 3;
            }
        } else if moves[1] == "X" {
            // Lose
            if moves[0] == "A" {
                part_2_score += 3;
            } else if moves[0] == "B" {
                part_2_score += 1;
            } else if moves[0] == "C" {
                part_2_score += 2;
            }
        }
    }

    println!("Part 2 solution: {}", part_2_score);

}

/// Checks if a round has been won based on the response of you and your opponent
fn round_won(opponent: &str, you: &str) -> bool {
    if opponent == "A" {
        return you == "Y";
    } else if opponent == "B" {
        return you == "Z";
    } else if opponent == "C" {
        return you == "X";
    } else {
        return false;
    }
}

fn round_tied(opponent: &str, you: &str) -> bool{
    if opponent == "A" {
        return you == "X";
    } else if opponent == "B" {
        return you == "Y";
    } else if opponent == "C" {
        return you == "Z";
    } else {
        return false;
    }
}