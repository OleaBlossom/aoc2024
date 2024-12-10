advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<i32> {
    let lines: Vec<&str> = input.lines().collect();
    let mut total_calibration_result: i32 = 0;

    for line in lines {
        let mut values: Vec<&str> = line.split(":").collect();
        let remaining_numbers: Vec<i32> = values
            .pop()
            .unwrap()
            .split(" ")
            .filter(|x| *x != "")
            .map(|i| i.parse::<i32>().unwrap())
            .collect();
        let expected_value: i32 = values.pop().unwrap().parse::<i32>().unwrap();

        let sum: i32 = remaining_numbers.iter().sum();
        let product: i32 = remaining_numbers.iter().product();

        if sum == expected_value || product == expected_value {
            total_calibration_result += expected_value;
            continue;
        }

        let possible_to_solve = expected_value > sum && expected_value < product;
        if !possible_to_solve {
            continue;
        }

        for i in 0..remaining_numbers.len() {
            let mut test_value =
                remaining_numbers.get(i).unwrap() * remaining_numbers.get(i - 1).unwrap();
            for j in 0..remaining_numbers.len() {
                if !(i == j || i == j - 1) {
                    test_value += remaining_numbers.get(j).unwrap();
                }
            }

            if test_value == expected_value {
                total_calibration_result += expected_value;
                continue;
            }
        }
    }
    Some(total_calibration_result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
