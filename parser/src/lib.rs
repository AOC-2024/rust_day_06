use std::fs::read_to_string;


pub fn read_puzzle(input_path: &str) -> (Map, Guard) {
    let mut map = Map::new();
    let mut guard = Guard::new();

    let file_content = read_to_string(input_path).unwrap();
    let lines: Vec<&str> = file_content
    .lines()
    .collect();

    for (line_index, line) in lines.into_iter().enumerate() {
        for (col_index, character) in line.chars().into_iter().enumerate() {
            if character == '#' {
                map.obstacles.push(Point {
                    x: line_index,
                    y: col_index
                });
            }
            if character == '>' {
                guard.position = Point {
                    x: line_index,
                    y: col_index
                };
                guard.direction = Direction::RIGHT;
            }
            if character == '<' {
                guard.position = Point {
                    x: line_index,
                    y: col_index
                };
                guard.direction = Direction::LEFT;
            }
            if character == '^' {
                guard.position = Point {
                    x: line_index,
                    y: col_index
                };
                guard.direction = Direction::UP;
            }
            if character == 'v' {
                guard.position = Point {
                    x: line_index,
                    y: col_index
                };
                guard.direction = Direction::DOWN;
            }
        }
    }

    (map, guard)
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Guard {
    position: Point,
    direction: Direction
}

impl Guard {
    fn new() -> Guard {
        Guard {
            position: Point {
                x: 0,
                y: 0
            },
            direction: Direction::UP
        }
    }
}


#[derive(PartialEq)]
#[derive(Debug)]
pub struct Point {
    x: usize,
    y: usize
}


#[derive(PartialEq)]
#[derive(Debug)]
pub struct Map {
    obstacles: Vec<Point>
}

impl Map {
    fn new() -> Map {
        Map {
            obstacles: Vec::new()
        }
    }
}


#[derive(PartialEq)]
#[derive(Debug)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_read_puzzle_guard_right() {
        let first_point = Point {
            x: 0,
            y: 4
        };
        let second_point = Point {
            x: 1,
            y: 9   
        };
        let guard = Guard {
            position: Point {
                x: 1,
                y: 3
            },
            direction: Direction::RIGHT
        };
        assert_eq!(read_puzzle("resources/puzzle_guard_right.txt"), (Map {
            obstacles: vec![first_point, second_point]
        }, guard));
    }

    #[test]
    fn should_read_puzzle_guard_left() {
        let first_point = Point {
            x: 0,
            y: 4
        };
        let second_point = Point {
            x: 1,
            y: 9   
        };
        let guard = Guard {
            position: Point {
                x: 1,
                y: 3
            },
            direction: Direction::LEFT
        };
        assert_eq!(read_puzzle("resources/puzzle_guard_left.txt"), (Map {
            obstacles: vec![first_point, second_point]
        }, guard));
    }

    #[test]
    fn should_read_puzzle_guard_up() {
        let first_point = Point {
            x: 0,
            y: 4
        };
        let second_point = Point {
            x: 1,
            y: 9   
        };
        let guard = Guard {
            position: Point {
                x: 1,
                y: 3
            },
            direction: Direction::UP
        };
        assert_eq!(read_puzzle("resources/puzzle_guard_up.txt"), (Map {
            obstacles: vec![first_point, second_point]
        }, guard));
    }

    
    #[test]
    fn should_read_puzzle_guard_down() {
        let first_point: Point = Point {
            x: 0,
            y: 4
        };
        let second_point = Point {
            x: 1,
            y: 9   
        };
        let guard = Guard {
            position: Point {
                x: 1,
                y: 3
            },
            direction: Direction::DOWN
        };
        assert_eq!(read_puzzle("resources/puzzle_guard_down.txt"), (Map {
            obstacles: vec![first_point, second_point]
        }, guard));
    }
}