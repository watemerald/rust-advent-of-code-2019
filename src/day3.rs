use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum MovementDirection {
    U(u32),
    D(u32),
    L(u32),
    R(u32),
}

impl FromStr for MovementDirection {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[..1] {
            "U" => Ok(MovementDirection::U(s[1..].parse().unwrap())),
            "D" => Ok(MovementDirection::D(s[1..].parse().unwrap())),
            "L" => Ok(MovementDirection::L(s[1..].parse().unwrap())),
            "R" => Ok(MovementDirection::R(s[1..].parse().unwrap())),
            _x => unreachable!(),
        }
    }
}

#[aoc_generator(day3)]
fn parse_input(s: &str) -> (Vec<MovementDirection>, Vec<MovementDirection>) {
    let lines: Vec<_> = s.split('\n').collect();

    let wire1 = lines.get(0).unwrap();
    let wire2 = lines.get(1).unwrap();

    (
        wire1
            .split(',')
            .map(|d| MovementDirection::from_str(d).unwrap())
            .collect(),
        wire2
            .split(',')
            .map(|d| MovementDirection::from_str(d).unwrap())
            .collect(),
    )
}

#[aoc(day3, part1)]
fn part1(input: &(Vec<MovementDirection>, Vec<MovementDirection>)) -> i32 {
    let (wire1, wire2) = input;

    let ord1 = visited_coords(wire1);
    let ord2 = visited_coords(wire2);

    let pos1: HashSet<_> = ord1.iter().collect();
    let pos2: HashSet<_> = ord2.iter().collect();

    pos1.intersection(&pos2)
        .filter(|(x, y)| *x != 0 || *y != 0)
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap()
}

#[aoc(day3, part2)]
fn part2(input: &(Vec<MovementDirection>, Vec<MovementDirection>)) -> usize {
    let (wire1, wire2) = input;

    let ord1 = visited_coords(wire1);
    let ord2 = visited_coords(wire2);

    let pos1: HashSet<_> = ord1.iter().collect();
    let pos2: HashSet<_> = ord2.iter().collect();

    pos1.intersection(&pos2)
        .filter(|(x, y)| *x != 0 || *y != 0)
        .map(|&int| {
            ord1.iter().position(|x| x == int).unwrap()
                + ord2.iter().position(|x| x == int).unwrap()
        })
        .min()
        .unwrap()
}

fn visited_coords(dirs: &[MovementDirection]) -> Vec<(i32, i32)> {
    // Positions that dirs has been in
    let mut pos = Vec::new();

    let mut curpos: (i32, i32) = (0, 0);

    pos.push(curpos);

    for d in dirs {
        match d {
            MovementDirection::U(amount) => {
                let amount = *amount as i32;

                for i in curpos.1 + 1..=curpos.1 + amount {
                    pos.push((curpos.0, i));
                }

                curpos = (curpos.0, curpos.1 + amount);
            }
            MovementDirection::D(amount) => {
                let amount = *amount as i32;

                // for i in curpos.1 - amount..curpos.1 {
                for i in (curpos.1 - amount..curpos.1).rev() {
                    pos.push((curpos.0, i));
                }

                curpos = (curpos.0, curpos.1 - amount);
            }
            MovementDirection::L(amount) => {
                let amount = *amount as i32;

                for i in (curpos.0 - amount..curpos.0).rev() {
                    pos.push((i, curpos.1));
                }

                curpos = (curpos.0 - amount, curpos.1);
            }
            MovementDirection::R(amount) => {
                let amount = *amount as i32;

                for i in curpos.0 + 1..=curpos.0 + amount {
                    pos.push((i, curpos.1));
                }

                curpos = (curpos.0 + amount, curpos.1);
            }
        }
    }

    pos
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_movement_from_str() {
        assert_eq!(
            MovementDirection::from_str("R992"),
            Ok(MovementDirection::R(992))
        );
        assert_eq!(
            MovementDirection::from_str("U849"),
            Ok(MovementDirection::U(849))
        );
        assert_eq!(
            MovementDirection::from_str("L230"),
            Ok(MovementDirection::L(230))
        );
        assert_eq!(
            MovementDirection::from_str("D909"),
            Ok(MovementDirection::D(909))
        );
    }

    #[test]
    fn test_day3_part1() {
        assert_eq!(part1(&parse_input("R8,U5,L5,D3\nU7,R6,D4,L4")), 6);

        assert_eq!(
            part1(&parse_input(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
            )),
            159
        );

        assert_eq!(
            part1(&parse_input(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )),
            135
        );
    }

    #[test]
    fn test_day3_part2() {
        assert_eq!(part2(&parse_input("R8,U5,L5,D3\nU7,R6,D4,L4")), 30);

        assert_eq!(
            part2(&parse_input(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
            )),
            610
        );

        assert_eq!(
            part2(&parse_input(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )),
            410
        );
    }
}
