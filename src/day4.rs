use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
fn parse_input(input: &str) -> (usize, usize) {
    if let [start, end] = input
        .split("-")
        .map(|d| d.parse().unwrap())
        .collect::<Vec<_>>()
        .as_slice()
    {
        return (*start, *end);
    }

    unreachable!()
}

fn is_possible_password_part1(p: usize) -> bool {
    let ps = p.to_string();

    // 6-digit number
    if ps.len() != 6 {
        return false;
    }

    //2 adjacent digits are the same
    let mut adjacent_digits = false;
    //Digints never decrease
    for (a, b) in ps.chars().zip(ps.chars().skip(1)) {
        if a == b {
            adjacent_digits = true;
        }
        if a > b {
            return false;
        }
    }

    adjacent_digits
}

fn is_possible_password_part2(p: usize) -> bool {
    let ps = p.to_string();

    // 6-digit number
    if ps.len() != 6 {
        return false;
    }

    //Exactrly 2 adjacent digits are the same
    let mut adjacent_digits = false;

    let mut last_char_in_group = '\0';

    let mut return_override = false;

    for (a, b) in ps.chars().zip(ps.chars().skip(1)) {
        if a == b {
            if b == last_char_in_group {
                adjacent_digits = false;
            } else {
                last_char_in_group = b;
                adjacent_digits = true;
            }
        } else {
            if adjacent_digits {
                return_override = true;
            }
        }
        if a > b {
            return false;
        }
    }

    adjacent_digits || return_override
}

#[aoc(day4, part1, iter)]
fn part1_iter(limits: &(usize, usize)) -> usize {
    let (a, b) = *limits;

    (a..=b).filter(|&a| is_possible_password_part1(a)).count()
}

#[aoc(day4, part2, iter)]
fn part2_iter(limits: &(usize, usize)) -> usize {
    let (a, b) = *limits;

    (a..=b).filter(|&a| is_possible_password_part2(a)).count()
}

#[aoc(day4, part1, generator)]
fn part1_generator(limits: &(usize, usize)) -> usize {
    let (a, b) = *limits;

    generator(vec![], false)
        .iter()
        .filter(|&&n| n >= a && n <= b)
        .count()
}

fn generator(cur_dig: Vec<usize>, condition_met: bool) -> Vec<usize> {
    if cur_dig.len() == 6 {
        if condition_met {
            let num = cur_dig.iter().fold(0, |acc, x| 10 * acc + x);
            return vec![num];
        } else {
            return Vec::new();
        }
    }

    match cur_dig.last() {
        Some(val) => {
            // Numbers can't decrease as we go on
            let mut k = Vec::new();

            for num in (*val..10).map(|n| {
                let mut tmp = cur_dig.clone();
                tmp.push(n);
                generator(tmp, n == *val || condition_met)
            }) {
                k.extend(num);
            }

            k
        }
        None => {
            // Password can start with any digit
            let mut k = Vec::new();

            for num in (0..10).map(|n| generator(vec![n], false)) {
                k.extend(num);
            }

            k
        }
    }
}

#[aoc(day4, part2, generator)]
fn part2_generator(limits: &(usize, usize)) -> usize {
    let (a, b) = *limits;

    generator_part2(vec![], false, false)
        .iter()
        .filter(|&&n| n >= a && n <= b)
        .count()
}

fn generator_part2(
    cur_dig: Vec<usize>,
    condition_met: bool,
    condition_override: bool,
) -> Vec<usize> {
    if cur_dig.len() == 6 {
        if condition_met || condition_override {
            let num = cur_dig.iter().fold(0, |acc, x| 10 * acc + x);
            return vec![num];
        } else {
            return Vec::new();
        }
    }

    match cur_dig.split_last() {
        Some((val, elements)) => {
            // Numbers can't decrease as we go on
            let mut k = Vec::new();

            for num in (*val..10).map(|n| {
                let mut tmp = cur_dig.clone();
                tmp.push(n);
                // generator(tmp, n == *val || condition_met)
                if n == *val {
                    match elements.last() {
                        Some(val2) => {
                            if n == *val2 {
                                generator_part2(tmp, false, condition_override)
                            } else {
                                generator_part2(tmp, true, condition_override)
                            }
                        }
                        None => generator_part2(tmp, true, condition_override),
                    }
                } else {
                    generator_part2(tmp, false, condition_met || condition_override)
                }
            }) {
                k.extend(num);
            }

            k
        }
        None => {
            // Password can start with any digit
            let mut k = Vec::new();

            for num in (0..10).map(|n| generator_part2(vec![n], false, false)) {
                k.extend(num);
            }

            k
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generator_2() {
        println!("{:?}", generator_part2(vec![], false, false))
    }

    #[test]
    fn test_is_possible_password_part1() {
        assert!(is_possible_password_part1(111111));
        assert!(!is_possible_password_part1(223450));
        assert!(!is_possible_password_part1(123789));
    }

    #[test]
    fn test_is_possible_password_part2() {
        assert!(is_possible_password_part2(112233));
        assert!(!is_possible_password_part2(123444));
        assert!(is_possible_password_part2(111122));
        assert!(is_possible_password_part2(112222));
    }
}
