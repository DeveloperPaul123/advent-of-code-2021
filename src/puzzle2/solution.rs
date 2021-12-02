pub fn calculate_movement_location_with_aim(lines: &Vec<String>) -> (u64, u64) {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in lines {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let command = parts[0].trim();
        let amount = parts[1].trim().parse::<u64>().unwrap();
        if command == "forward" {
            horizontal += amount;
            depth += aim * amount;
        } else if command == "up" {
            aim -= amount;
        } else if command == "down" {
            aim += amount;
        }
    }
    (horizontal, depth)
}

pub fn calculate_movement_location(lines: &Vec<String>) -> (u64, u64) {
    let mut horizontal = 0;
    let mut vertical = 0;
    for line in lines {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let command = parts[0].trim();
        let amount = parts[1].trim().parse::<u64>().unwrap();
        if command == "forward" {
            horizontal += amount
        } else if command == "up" {
            vertical -= amount;
        } else if command == "down" {
            vertical += amount;
        }
    }

    (horizontal, vertical)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_movement_location() {
        // sample args from aoc
        let sample_args: Vec<String> = vec![
            String::from("forward 5"),
            String::from("down 5"),
            String::from("forward 8"),
            String::from("up 3"),
            String::from("down 8"),
            String::from("forward 2"),
        ];

        let (horz, depth) = calculate_movement_location(&sample_args);

        assert_eq!(horz, 15);
        assert_eq!(depth, 10);
    }

    #[test]
    fn test_movement_location_with_aim() {
        let sample_args: Vec<String> = vec![
            String::from("forward 5"),
            String::from("down 5"),
            String::from("forward 8"),
            String::from("up 3"),
            String::from("down 8"),
            String::from("forward 2"),
        ];

        let (horz, depth) = calculate_movement_location_with_aim(&sample_args);

        assert_eq!(horz, 15);
        assert_eq!(depth, 60);
    }
}
