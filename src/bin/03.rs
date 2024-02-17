advent_of_code::solution!(3);

pub fn part_one(_input: &str) -> Option<u32> {
    let mut sum = 0;

    for line in _input.split("\n") {
        let halves = line.split_at(line.len() / 2);
        for letter in halves.0.chars() {
            if halves.1.contains(letter) {
                sum += return_priority_value(letter);
                break;
            }
        }
    }
    Some(sum)
}

pub fn part_two(_input: &str) -> Option<u32> {
    let mut sum = 0;
    let all_elves = _input.split("\n").collect::<Vec<&str>>();

    for i in 0..(&all_elves.len() / 3) {
        let first_elf = all_elves.get((0 + i * 3)).unwrap();
        let second_elf = all_elves.get((1 + i * 3)).unwrap();
        let third_elf = all_elves.get(2 + i * 3).unwrap();
        for letter in first_elf.chars() {
            if second_elf.contains(letter) && third_elf.contains(letter) {
                sum += return_priority_value(letter);
                break;
            }
        }
    }
    Some(sum)
}

pub fn return_priority_value(letter: char) -> u32 {
    let min_uppercase_char = 'A' as u32;
    let min_lowercase_char = 'a' as u32;
    let mut sum = 0;
    if letter.is_lowercase() {
        sum += (letter as u32) - min_lowercase_char + 1;
    } else {
        sum += (letter as u32) - min_uppercase_char + 27;
    }
    return sum;
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
