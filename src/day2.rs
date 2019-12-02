use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn parse_input(s: &str) -> Vec<usize> {
    s.split(',').map(|n| n.parse().unwrap()).collect()
}

fn run_program(program: &[usize]) -> Vec<usize> {
    let mut p = program.to_vec();
    // The current location in the program
    let mut head = 0;
    loop {
        // Handle each op code
        match p[head] {
            1 => {
                if let [_op, ad1, ad2, ad3] = p[head..head + 4] {
                    p[ad3] = p[ad1] + p[ad2];
                }
            }
            2 => {
                if let [_op, ad1, ad2, ad3] = p[head..head + 4] {
                    p[ad3] = p[ad1] * p[ad2];
                }
            }
            99 => {
                break;
            }
            _ => unreachable!(),
        }

        // Move to the next instruction
        head += 4;
    }

    p
}

#[aoc(day2, part1)]
fn part1(program: &[usize]) -> usize {
    let mut p = program.to_vec();
    p[1] = 12;
    p[2] = 2;

    let res = run_program(&p);

    res[0]
}

#[aoc(day2, part2)]
fn part2(program: &[usize]) -> usize {
    let mut p = program.to_vec();

    for noun in 1..100 {
        for verb in 1..100 {
            p[1] = noun;
            p[2] = verb;

            let res = run_program(&p);

            if res[0] == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day2_input() {
        assert_eq!(parse_input("1,0,0,0,99"), vec![1, 0, 0, 0, 99])
    }

    #[test]
    fn test_run_program() {
        assert_eq!(run_program(&[1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
        assert_eq!(run_program(&[2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
        assert_eq!(
            run_program(&[2, 4, 4, 5, 99, 0]),
            vec![2, 4, 4, 5, 99, 9801]
        );
        assert_eq!(
            run_program(&[1, 1, 1, 4, 99, 5, 6, 0, 99]),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        )
    }
}
