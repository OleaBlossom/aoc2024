advent_of_code::solution!(10, 1);

pub fn part_one(input: &str) -> Option<i32> {
    let mut v: Vec<i32> = input.lines().map(|i| i.parse::<i32>().unwrap()).collect();

    v.sort();

    let diffs: Vec<i32> = vec_diff(&v);

    let ones: Vec<i32> = diffs.clone().into_iter().filter(|x| *x == 1).collect();
    let threes: Vec<i32> = diffs.clone().into_iter().filter(|x| *x == 3).collect();

    let total: i32 = (ones.len() as i32 + 1) * (threes.len() as i32 + 1);

    return Some(total);
}

fn vec_diff(input: &[i32]) -> Vec<i32> {
    let vals = input.iter();
    let next_vals = input.iter().skip(1);

    vals.zip(next_vals).map(|(cur, next)| next - cur).collect()
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
        assert_eq!(result, Some(220));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
