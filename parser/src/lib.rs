use std::fs::read_to_string;


pub fn read_puzzle(input_path: &str) -> (Map, Guard) {
    let mut map = Map::new();
    let mut guard = Guard::new();

    let file_content = read_to_string(input_path).unwrap();
    let lines: Vec<&str> = file_content
    .lines()
    .collect();

    for (line_index, line) in lines.into_iter().enumerate() {
        map.rows += 1;
        map.columns = line.len();
        for (col_index, character) in line.chars().into_iter().enumerate() {
            if character == '#' {
                map.obstacles.push(Point {
                    x: col_index,
                    y: line_index
                });
            }
            if character == '>' {
                guard.position = Point {
                    x: col_index,
                    y: line_index
                };
                guard.direction = Direction::RIGHT;
            }
            if character == '<' {
                guard.position = Point {
                    x: col_index,
                    y: line_index
                };
                guard.direction = Direction::LEFT;
            }
            if character == '^' {
                guard.position = Point {
                    x: col_index,
                    y: line_index
                };
                guard.direction = Direction::UP;
            }
            if character == 'v' {
                guard.position = Point {
                    x: col_index,
                    y: line_index
                };
                guard.direction = Direction::DOWN;
            }
        }
    }
    (map, guard)
}


#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Point {
    x: usize,
    y: usize
}


#[derive(PartialEq, Debug)]
pub struct Map {
    obstacles: Vec<Point>,
    rows: usize,
    columns: usize
}

impl Map {
    fn new() -> Map {
        Map {
            obstacles: Vec::new(),
            rows: 0,
            columns: 0
        }
    }
}


#[derive(PartialEq, Debug)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

#[derive(PartialEq, Debug)]
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

    pub fn itinerary(&mut self, map: &Map) -> Vec<Point> {
        let mut intinerary = Vec::new();
        intinerary.push(Point {
            x: self.position.x,
            y: self.position.y
        });
        
        loop {
            if self.will_get_out_next_step(&map) {
                break;
            }
            self.check_direction_change(&map);
            self.next_move();
            intinerary.push(Point {
                x: self.position.x,
                y: self.position.y
            });
        }

        return intinerary;
    }

    fn will_get_out_next_step(&self, map: &Map) -> bool {
        match self.direction {
            Direction::DOWN => self.position.y == map.rows - 1,
            Direction::UP => self.position.y == 0,
            Direction::RIGHT => self.position.x == map.columns - 1,
            Direction::LEFT => self.position.x == 0 
        }
    }

    fn next_move(&mut self) {
        match self.direction {
            Direction::DOWN => self.position.y += 1,
            Direction::UP => self.position.y -= 1,
            Direction::RIGHT => self.position.x += 1,
            Direction::LEFT => self.position.x -= 1  
        };
    }

    fn check_direction_change(&mut self, map: &Map) {
        match self.direction {
            Direction::DOWN => {
                if map.obstacles.iter().any(|obstacle| {
                    obstacle.x == self.position.x && self.position.y + 1 == obstacle.y 
                }).to_owned() {
                    self.direction = Direction::LEFT
                }
            },
            Direction::UP => {
                if map.obstacles.iter().any(|obstacle| {
                    obstacle.x == self.position.x && self.position.y - 1 == obstacle.y 
                }).to_owned() {
                    self.direction = Direction::RIGHT
                }
            },
            Direction::RIGHT => {
                if map.obstacles.iter().any(|obstacle| {
                    obstacle.x == self.position.x + 1 && self.position.y== obstacle.y 
                }).to_owned() {
                    self.direction = Direction::DOWN
                }
            },
            Direction::LEFT => {
                if map.obstacles.iter().any(|obstacle| {
                    obstacle.x == self.position.x - 1 && self.position.y == obstacle.y 
                }).to_owned() {
                    self.direction = Direction::UP
                }
            }  
        };
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_have_itinerary_when_guard_get_out_with_one_obstacle_left_to_down_rotate() {
        let mut guard = Guard {
            position: Point {
                x: 0,
                y: 0
            },
            direction: Direction::RIGHT
        };
        let empty_map = Map {
            obstacles: vec![Point {
                x: 2,
                y: 0
            }],
            rows: 3,
            columns: 3
        };

        assert_eq!(guard.itinerary(&empty_map), vec![
            Point {
                x: 1,
                y: 0
            },
            Point {
                x: 1,
                y: 1
            },
            Point {
                x: 1,
                y: 2
            }
        ]);
    }

    #[test]
    fn should_have_itinerary_when_guard_get_out_without_obstacle_right() {
        let mut guard = Guard {
            position: Point {
                x: 0,
                y: 0
            },
            direction: Direction::RIGHT
        };
        let map = Map {
            obstacles: Vec::new(),
            rows: 1,
            columns: 2
        };

        assert_eq!(guard.itinerary(&map), vec![
            Point {
                x: 1,
                y: 0
            }
        ]);
    }

    #[test]
    fn should_have_itinerary_when_guard_get_out_without_obstacle_left() {
        let mut guard = Guard {
            position: Point {
                x: 1,
                y: 0
            },
            direction: Direction::LEFT
        };
        let map = Map {
            obstacles: Vec::new(),
            rows: 1,
            columns: 2
        };

        assert_eq!(guard.itinerary(&map), vec![
            Point {
                x: 0,
                y: 0
            }
        ]);
    }

    #[test]
    fn should_have_itinerary_when_guard_get_out_without_obstacle_up() {
        let mut guard = Guard {
            position: Point {
                x: 1,
                y: 1
            },
            direction: Direction::UP
        };
        let map = Map {
            obstacles: Vec::new(),
            rows: 2,
            columns: 2
        };

        assert_eq!(guard.itinerary(&map), vec![
            Point {
                x: 1,
                y: 0
            }
        ]);
    }

    #[test]
    fn should_have_itinerary_when_guard_get_out_without_obstacle_down() {
        let mut guard = Guard {
            position: Point {
                x: 1,
                y: 0
            },
            direction: Direction::DOWN
        };
        let map = Map {
            obstacles: Vec::new(),
            rows: 2,
            columns: 2
        };

        assert_eq!(guard.itinerary(&map), vec![
            Point {
                x: 1,
                y: 1
            }
        ]);
    }

    #[test]
    fn should_have_empty_itinerary_when_empty_map() {
        let mut guard = Guard::new();
        let empty_map = Map::new();

        assert_eq!(guard.itinerary(&empty_map), Vec::new());
    }

    #[test]
    fn should_read_puzzle_guard_right() {
        let first_point = Point {
            x: 4,
            y: 0
        };
        let second_point = Point {
            x: 9,
            y: 1   
        };
        let guard = Guard {
            position: Point {
                x: 3,
                y: 1
            },
            direction: Direction::RIGHT
        };
        assert_eq!(read_puzzle("resources/puzzle_guard_right.txt"), (Map {
            obstacles: vec![first_point, second_point],
            rows: 2,
            columns: 10
        }, guard));
    }

    #[test]
    fn should_read_puzzle_guard_left() {
        let first_point = Point {
            x: 4,
            y: 0
        };
        let second_point = Point {
            x: 9,
            y: 1   
        };
        let guard = Guard {
            position: Point {
                x: 3,
                y: 1
            },
            direction: Direction::LEFT
        };
        assert_eq!(read_puzzle("resources/puzzle_guard_left.txt"), (Map {
            obstacles: vec![first_point, second_point],
            rows: 2,
            columns: 10
        }, guard));
    }

    #[test]
    fn should_read_puzzle_guard_up() {
        let first_point = Point {
            x: 4,
            y: 0
        };
        let second_point = Point {
            x: 9,
            y: 1   
        };
        let guard = Guard {
            position: Point {
                x: 3,
                y: 1
            },
            direction: Direction::UP
        };
        assert_eq!(read_puzzle("resources/puzzle_guard_up.txt"), (Map {
            obstacles: vec![first_point, second_point],
            rows: 2,
            columns: 10
        }, guard));
    }

    
    #[test]
    fn should_read_puzzle_guard_down() {
        let first_point: Point = Point {
            x: 4,
            y: 0
        };
        let second_point = Point {
            x: 9,
            y: 1   
        };
        let guard = Guard {
            position: Point {
                x: 3,
                y: 1
            },
            direction: Direction::DOWN
        };
        assert_eq!(read_puzzle("resources/puzzle_guard_down.txt"), (Map {
            obstacles: vec![first_point, second_point],
            rows: 2,
            columns: 10
        }, guard));
    }
}