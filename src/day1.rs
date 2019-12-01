use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input(s: &str) -> Vec<i32> {
    s.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
fn part1(n: &[i32]) -> i32 {
    n.iter().map(|k| (k / 3) - 2).sum()
}

#[aoc(day1, part2)]
fn part2(n: &[i32]) -> i32 {
    n.iter()
        .map(|&k| {
            let mut n = k;
            let mut ret = 0;

            loop {
                let tmp = (n / 3) - 2;
                if tmp < 0 {
                    break;
                }
                ret += tmp;
                n = tmp;
            }

            ret
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day1_part1() {
        assert_eq!(part1(&[12]), 2);
        assert_eq!(part1(&[14]), 2);
        assert_eq!(part1(&[1969]), 654);
        assert_eq!(part1(&[100756]), 33583);
    }

    #[test]
    fn test_day1_part2() {
        assert_eq!(part2(&[12]), 2);
        assert_eq!(part2(&[1969]), 966);
        assert_eq!(part2(&[100756]), 50346);
    }
}
