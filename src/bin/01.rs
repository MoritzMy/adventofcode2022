advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {

    let mut result:u32 = 0;
    let mut calories = create_calorie_list(input);

    for entry in calories {
        if entry > result {
            result = entry;
        }
    }
    Some(result);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result:u32 = 0;
    let mut calories = create_calorie_list(input);

    calories.sort();

    for i in 1..4 {
        result += calories[calories.len() - i];
    }

    Some(result);
    None
}
pub fn create_calorie_list(input: &str) -> Vec<u32>{
    let mut elf:usize = 0;
    let mut calories = Vec::new();
    let mut sum_of_current_elf:u32 = 0;
    for line in input.split("\n") {
        if  line == "" {
            calories.push(sum_of_current_elf);
            elf += 1;
            sum_of_current_elf = 0;
        }
        else if calories.len() == elf {
            sum_of_current_elf += line.parse::<u32>().unwrap();
        }
    }
    return calories;

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
