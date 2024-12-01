advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let v: Vec<&str> = input.lines().collect();
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for line in v.iter().enumerate() {
        let values: Vec<i32> = line
            .1
            .split("   ")
            .map(|i| i.parse::<i32>().unwrap())
            .collect();
        left.push(values[0]);
        right.push(values[1]);
    }

    left.sort();
    right.sort();

    let mut total: i32 = 0;

    for i in 1..=left.len() {
        let left: i32 = *left.get(i - 1).unwrap();
        let right: i32 = *right.get(i - 1).unwrap();
        let diff: i32 = left - right;
        total += diff.abs();
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<i32> {
    let v: Vec<&str> = input.lines().collect();
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for line in v.iter().enumerate() {
        let values: Vec<i32> = line
            .1
            .split("   ")
            .map(|i| i.parse::<i32>().unwrap())
            .collect();
        left.push(values[0]);
        right.push(values[1]);
    }

    let mut total: i32 = 0;

    let mut unique = left.clone();
    unique.sort();
    unique.dedup();

    for n in unique {
        let n_left: Vec<i32> = left.clone().into_iter().filter(|x| *x == n).collect();
        let n_right: Vec<i32> = right.clone().into_iter().filter(|x| *x == n).collect();
        total += n * n_left.len() as i32 * n_right.len() as i32;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
