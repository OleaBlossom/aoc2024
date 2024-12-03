advent_of_code::solution!(2, 2);

pub fn part_one(input: &str) -> Option<i32> {
    let v: Vec<&str> = input.lines().collect();

    let mut total_safe: i32 = 0;

    for line in v.iter().enumerate() {
        println!("{:?}", total_safe);
        let report_values: Vec<i32> = line
            .1
            .split(" ")
            .map(|i| i.parse::<i32>().unwrap())
            .collect();

        let mut sorted_report = report_values.clone();
        sorted_report.sort();
        let mut sorted_desc = sorted_report.clone();
        sorted_desc.reverse();

        println!("{:?}", report_values);

        if report_values == sorted_report || report_values == sorted_desc {
            let vals = report_values.iter();
            let next_vals = report_values.iter().skip(1);

            let checked: Vec<i32> = vals
                .zip(next_vals)
                .map(|(cur, next)| (next - cur).abs())
                .filter(|x| *x > 0 && *x < 4)
                .collect();

            println!("{:?}", checked);
            match report_values.len() - checked.len() == 1 {
                false => continue,
                true => {
                    total_safe += 1;
                }
            }
        } else {
            continue;
        }
    }

    Some(total_safe)
}

pub fn part_two(input: &str) -> Option<i32> {
    let v: Vec<&str> = input.lines().collect();

    let total_safe: i32 = 0;

    for line in v.iter().enumerate() {
        let report_values: Vec<i32> = line
            .1
            .split(" ")
            .map(|i| i.parse::<i32>().unwrap())
            .collect();

        println!("values: {:?}", report_values);

        let next_values: Vec<i32> = report_values.iter().skip(1).cloned().collect();
        // for current_value in report_values.iter().enumerate() {
        for i in 1..report_values.len() {
            println!("current: {:?}", report_values.get(i - 1));
            println!("next: {:?}", next_values.get(i - 1));
        }
    }

    Some(total_safe)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
