use std::collections::HashMap;
use std::ffi::c_char;
advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;

    let get_game_outcome = create_result_map_part_one();

    for line in input.split("\n") {
        sum += get_game_outcome.get(line).unwrap();
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;

    let get_game_outcome = create_result_map_part_two();

    for line in input.split("\n") {
        sum += get_game_outcome.get(line).unwrap();
    }
    Some(sum)
}
pub fn create_result_map_part_one() -> HashMap<&'static str, u32> {
    let mut get_game_outcome = HashMap::new();
    get_game_outcome.insert("A X", 4);
    get_game_outcome.insert("A Y", 8);
    get_game_outcome.insert("A Z", 3);
    get_game_outcome.insert("B X", 1);
    get_game_outcome.insert("B Y", 5);
    get_game_outcome.insert("B Z", 9);
    get_game_outcome.insert("C X", 7);
    get_game_outcome.insert("C Y", 2);
    get_game_outcome.insert("C Z", 6);
    return get_game_outcome

}
pub fn create_result_map_part_two() -> HashMap<&'static str, u32> {
    let mut get_game_outcome = HashMap::new();
    get_game_outcome.insert("A X", 3);
    get_game_outcome.insert("A Y", 4);
    get_game_outcome.insert("A Z", 8);
    get_game_outcome.insert("B X", 1);
    get_game_outcome.insert("B Y", 5);
    get_game_outcome.insert("B Z", 9);
    get_game_outcome.insert("C X", 2);
    get_game_outcome.insert("C Y", 6);
    get_game_outcome.insert("C Z", 7);
    return get_game_outcome
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
