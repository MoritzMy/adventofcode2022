advent_of_code::solution!(5);
pub fn part_one(_input: &str) -> Option<String> {
    let set_amount = 9;
    let mut list_of_sets = create_initial_data();

    for line in _input.split("\n") {
        let line_parts = line.split(" ").collect::<Vec<&str>>();
        let amount = line_parts[1].parse::<u32>().unwrap();
        let from = line_parts[3].parse::<usize>().unwrap();
        let to = line_parts[5].parse::<usize>().unwrap();

        for _i in 0..amount {
            let value = list_of_sets[from-1].pop().unwrap();
            list_of_sets[to-1].push(value);
        }
    }
    let mut result: String = String::new();
    for i in 0..set_amount {
        result = result + &*list_of_sets[i].pop().unwrap().to_string();
    }

    Some(result)
}

pub fn create_initial_data() -> Vec<Vec<char>> {
    let set_amount = 9;
    let mut column = Vec::new();

    for i in 0..set_amount {
        column.push(Vec::new())
    }

    column[0].push('V');
    column[0].push('C');
    column[0].push('D');
    column[0].push('R');
    column[0].push('Z');
    column[0].push('G');
    column[0].push('B');
    column[0].push('W');
    column[1].push('G');
    column[1].push('W');
    column[1].push('F');
    column[1].push('C');
    column[1].push('B');
    column[1].push('S');
    column[1].push('T');
    column[1].push('V');
    column[2].push('C');
    column[2].push('B');
    column[2].push('S');
    column[2].push('N');
    column[2].push('W');
    column[3].push('Q');
    column[3].push('G');
    column[3].push('M');
    column[3].push('N');
    column[3].push('J');
    column[3].push('V');
    column[3].push('C');
    column[3].push('P');
    column[4].push('T');
    column[4].push('S');
    column[4].push('L');
    column[4].push('F');
    column[4].push('D');
    column[4].push('H');
    column[4].push('B');
    column[5].push('J');
    column[5].push('V');
    column[5].push('T');
    column[5].push('W');
    column[5].push('M');
    column[5].push('N');
    column[6].push('P');
    column[6].push('F');
    column[6].push('L');
    column[6].push('C');
    column[6].push('S');
    column[6].push('T');
    column[6].push('G');
    column[7].push('B');
    column[7].push('D');
    column[7].push('Z');
    column[8].push('M');
    column[8].push('N');
    column[8].push('Z');
    column[8].push('W');

    return column;
}
pub fn part_two(input: &str) -> Option<String> {

    let set_amount = 9;
    let mut list_of_sets = create_initial_data();

    for line in input.split("\n") {
        let line_parts = line.split(" ").collect::<Vec<&str>>();
        let amount = line_parts[1].parse::<usize>().unwrap();
        let from = line_parts[3].parse::<usize>().unwrap();
        let to = line_parts[5].parse::<usize>().unwrap();
        let mut from_list = &mut list_of_sets[from - 1];
        let drain: Vec<char> = from_list.drain(from_list.len()-amount..).collect();
        list_of_sets[to - 1].extend(drain);
    }
    let mut result: String = String::new();
    for i in 0..set_amount {
        result = result + &*list_of_sets[i].pop().unwrap().to_string();
    }
        Some(result)
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
