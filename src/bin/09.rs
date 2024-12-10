advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    let mut result: Vec<String> = Vec::new();
    let mut total: i32 = 0;
    let mut pos: i32 = 0;
    for i in 0..input.len() {
        let is_even = i.rem_euclid(2) == 1;
        let n = input.get(i..i + 1).unwrap().parse::<i32>().unwrap();

        let mut count = n;
        while count > 0 {
            if is_even {
                result.push(".".to_string());
            } else {
                result.push(pos.to_string());
            }
            count -= 1;
        }

        if is_even {
            pos += 1;
        }
    }

    let count_dots: Vec<String> = result.clone();
    let count = count_dots.iter().filter(|&x| x.eq(".")).count();

    println!("{:?}", result.join(""));

    let mut pos = result.len() - 1;
    for i in 0..result.len() - count {
        if result[i].eq(".") {
            while result[pos].eq(".") {
                pos -= 1;
            }
            result.swap(i, pos);
            pos -= 1;
        }

        if !result[i].eq(".") {
            let r: i32 = result[i].parse::<i32>().unwrap() * i as i32;
            total += r;
        }
    }

    println!("{:?}", result.join(""));
    Some(total)
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
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
