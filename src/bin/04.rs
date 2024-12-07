advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;

    let size: i32 = input.lines().nth(1).unwrap().len() as i32;
    let layers = size / 2;

    for line in input.lines() {
        total += line.matches("XMAS").count();
        total += line.matches("SAMX").count();
    }

    println!("{}", total);

    None
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
