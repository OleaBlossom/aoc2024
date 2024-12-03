use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<i64> {
    let re: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut v = vec![];
    for (_, [x, y]) in re.captures_iter(input).map(|c| c.extract()) {
        v.push(x.parse::<i64>().unwrap() * y.parse::<i64>().unwrap());
    }

    Some(v.iter().sum())
}

pub fn part_two(input: &str) -> Option<i64> {
    let re: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut v = vec![];

    let lines: Vec<&str> = input.split("do()").collect();

    for (_, line) in lines.iter().enumerate() {
        let l: Vec<&str> = line.split("don't").collect();

        for (_, [x, y]) in re.captures_iter(l.get(0).unwrap()).map(|c| c.extract()) {
            v.push(x.parse::<i64>().unwrap() * y.parse::<i64>().unwrap());
        }
    }

    Some(v.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
