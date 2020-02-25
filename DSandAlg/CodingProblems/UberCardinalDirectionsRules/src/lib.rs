// https://www.reddit.com/r/CodingProblems/comments/f5umwr/day_3_20200218_problem_of_the_day_asked_by_uber/

use std::collections::{HashMap, VecDeque};

#[derive(Clone, Copy)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

struct Rule {
    lhs: u8,
    rhs: u8,
    direction: Direction,
}

fn get_rhs_position(position: u8, direction: Direction) -> u8 {
    // SS: 'position' 'direction' 'unknown'
    let offset = match direction {
        Direction::N => 4,
        Direction::NE => 3,
        Direction::E => 2,
        Direction::SE => 1,
        Direction::S => 0,
        Direction::SW => 7,
        Direction::W => 6,
        Direction::NW => 5,
    };

    let pos = (position + offset) % 8;
    pos
}

// SS: could be expressed in rhs above...
fn get_lhs_position(position: u8, direction: Direction) -> u8 {
    // SS: 'unknown' 'direction' 'position'
    let offset = match direction {
        Direction::N => 4,
        Direction::NE => 5,
        Direction::E => 6,
        Direction::SE => 7,
        Direction::S => 0,
        Direction::SW => 1,
        Direction::W => 2,
        Direction::NW => 3,
    };

    let pos = (position + offset) % 8;
    pos
}

fn check_validity(pos1: u8, pos2: u8) -> bool {
    let lower = (pos1 + 7) % 8;
    let upper = (pos1 + 1) % 8;
    pos2 >= lower && pos2 <= upper
}

fn validate_rules(rules: &[Rule]) -> bool {
    if rules.is_empty() {
        true
    } else {
        let mut s = HashMap::new();
        let r = &rules[0];

        // SS: insert 1st rule to set directions
        s.insert(r.rhs, 4);
        let position = get_lhs_position(4, r.direction);
        s.insert(r.lhs, position);

        let mut queue = VecDeque::new();
        rules.iter().skip(1).for_each(|r| {
            queue.push_back(r);
        });

        while queue.is_empty() == false {
            let r = queue.pop_front().unwrap();
            if s.contains_key(&r.lhs) {
                let rhs_pos = get_rhs_position(s[&r.lhs], r.direction);

                // SS: check if r.rhs already exists
                if s.contains_key(&r.rhs) {
                    // SS: does rule cause a contradiction?
                    let valid = check_validity(s[&r.rhs], rhs_pos);
                    if valid == false {
                        return false;
                    }
                } else {
                    s.insert(r.rhs, rhs_pos);
                }
            } else if s.contains_key(&r.rhs) {
                let lhs_pos = get_lhs_position(s[&r.rhs], r.direction);

                // SS: check if r.rhs already exists
                if s.contains_key(&r.lhs) {
                    // SS: does rule cause a contradiction?
                    let valid = check_validity(s[&r.lhs], lhs_pos);
                    if valid == false {
                        return false;
                    }
                } else {
                    s.insert(r.lhs, lhs_pos);
                }
            } else {
                queue.push_back(r);
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::Direction::{N, NE, NW};
    use crate::{get_lhs_position, get_rhs_position, validate_rules, Rule};

    #[test]
    fn test_get_position() {
        // Arrange

        // Act/Assert

        // SS: A N B, B=south(4) => A=north(0)
        let position = get_lhs_position(4, N);
        assert_eq!(position, 0);

        // SS: B NE C, C=west(6) => B=southeast(3)
        let position = get_lhs_position(6, NE);
        assert_eq!(position, 3);

        // SS: B NE C, B=southeast(3) => C=west(6)
        let position = get_rhs_position(3, NE);
        assert_eq!(position, 6);

        // SS: B N C, B=west(6) => C=east(2)
        let position = get_rhs_position(6, N);
        assert_eq!(position, 2);

        // SS: B N C, C=west(6) => B=east(2)
        let position = get_lhs_position(6, N);
        assert_eq!(position, 2);
    }

    #[test]
    fn test1() {
        // Arrange
        let rules = [
            Rule {
                lhs: 0,
                rhs: 1,
                direction: N,
            },
            Rule {
                lhs: 1,
                rhs: 2,
                direction: NE,
            },
            Rule {
                lhs: 2,
                rhs: 0,
                direction: N,
            },
        ];

        // Act
        let valid = validate_rules(&rules);

        // Assert
        assert_eq!(false, valid);
    }

    #[test]
    fn test2() {
        // Arrange
        let rules = [
            Rule {
                lhs: 0,
                rhs: 1,
                direction: NW,
            },
            Rule {
                lhs: 0,
                rhs: 1,
                direction: N,
            },
        ];

        // Act
        let valid = validate_rules(&rules);

        // Assert
        assert_eq!(true, valid);
    }
}
