advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut counter:u32 = 0;

    for line in input.split("\n") {
        let parts = line.split(",").collect::<Vec<&str>>();
        let left_side = parts[0].split("-").collect::<Vec<&str>>();
        let right_side = parts[1].split("-").collect::<Vec<&str>>();
        let left_min = left_side[0].parse::<u32>().unwrap();
        let left_max = left_side[1].parse::<u32>().unwrap();
        let right_min = right_side[0].parse::<u32>().unwrap();
        let right_max = right_side[1].parse::<u32>().unwrap();

        if ((left_min >= right_min && left_max <= right_max) || (right_min >= left_min && right_max <= left_max)) {
            counter += 1;
        }

    }
    Some(counter)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut counter:u32 = 0;

    for line in input.split("\n") {
        let parts = line.split(",").collect::<Vec<&str>>();
        let left_side = parts[0].split("-").collect::<Vec<&str>>();
        let right_side = parts[1].split("-").collect::<Vec<&str>>();
        let left_min = left_side[0].parse::<u32>().unwrap();
        let left_max = left_side[1].parse::<u32>().unwrap();
        let right_min = right_side[0].parse::<u32>().unwrap();
        let right_max = right_side[1].parse::<u32>().unwrap();

        if ((left_min >= right_min && left_min <= right_max) || (left_max >= right_min && left_max <= right_max) || (right_min >= left_min && right_min <= left_max) || (right_max >= left_min && right_max<= left_max)) {
            counter += 1;
        }

    }
    Some(counter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
