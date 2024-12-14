use std::collections::HashSet;

advent_of_code::solution!(5, 1);

pub fn part_one(input: &str) -> Option<i32> {
    let (orderings, updates) = input.split_once("\n\n").unwrap();

    let orderings: HashSet<(i32, i32)> = orderings
        .lines()
        .map(|line| {
            let parts = line.split_once("|").unwrap();
            (
                parts.0.parse::<i32>().unwrap(),
                parts.1.parse::<i32>().unwrap(),
            )
        })
        .collect();

    let updates: Vec<Vec<i32>> = updates
        .lines()
        .map(|line| {
            let parts = line.split(",").map(|i| i.parse::<i32>().unwrap()).collect();

            parts
        })
        .collect();

    let compare = |x: &i32, y: &i32| !orderings.contains(&(*y, *x));

    let mut page_number_total = 0;
    for update in updates {
        if update.is_sorted_by(compare) {
            page_number_total += update.get((update.len() - 1) / 2).unwrap();
        }
    }

    Some(page_number_total)
}

#[allow(unused_variables)]
pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
