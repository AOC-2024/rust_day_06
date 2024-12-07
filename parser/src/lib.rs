use std::{collections::HashSet, fs::read_to_string};


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


#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize
}


#[derive(PartialEq, Debug, Clone)]
pub struct Map {
    pub obstacles: Vec<Point>,
    pub rows: usize,
    pub columns: usize
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


#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

#[derive(PartialEq, Debug, Clone, Copy)]
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

    pub fn itinerary(&mut self, map: &Map) -> Option<Vec<Point>> {
        let mut itinerary = Vec::new();
        let mut visited_states = std::collections::HashSet::new();
    
        let initial_state = (self.position, self.direction);
        visited_states.insert(initial_state);
    
        itinerary.push(self.position);
    
        loop {
            

            if self.will_get_out_next_step(map) {
                return Some(itinerary);
            }
    
            let new_direction = Guard::next_direction(&self, &map).unwrap_or_else(|| self.direction);
            self.direction = new_direction;
    
            self.position = Guard::next_point(self.direction, self.position, self, &map);
    
            let current_state = (self.position, self.direction);
            if !visited_states.insert(current_state) {
                return None; 
            }
    
            itinerary.push(self.position);
        }
    }

    pub fn find_all_blocking_obstructions(&mut self, map: &Map) -> HashSet<Point> {
        let mut possible_obstructions = HashSet::new();
        let initial_position = self.clone().position;
        let initial_direction = self.clone().direction;
        let initial_itinerary = self.itinerary(map);

    
        if initial_itinerary.is_none() {
            return possible_obstructions;
        }
    

        let unique_points: HashSet<_> = initial_itinerary.unwrap().into_iter().collect();
    
        for point in &unique_points {
            let mut temp_map = map.clone();
            // Add an obstacle at the current point in the itinerary
            temp_map.obstacles.push(*point);

            let mut initial_guard = Guard {
                position: initial_position,
                direction: initial_direction
            };
    
            // Check if placing this obstacle blocks the guard
            if initial_guard.itinerary(&temp_map).is_none() {
                possible_obstructions.insert(*point);
            }
        }
    
        possible_obstructions
    }

    fn will_get_out_next_step(&self, map: &Map) -> bool {
        match self.direction {
            Direction::DOWN => self.position.y == map.rows - 1,
            Direction::UP => self.position.y == 0,
            Direction::RIGHT => self.position.x == map.columns - 1,
            Direction::LEFT => self.position.x == 0 
        }
    }

    fn next_point(direction: Direction, current_point: Point, guard: &Guard, map: &Map) -> Point {
        let mut point = current_point.clone();
        match direction {
            Direction::DOWN => {
                if !map.obstacles.iter().any(|obstacle| {
                    obstacle.x == guard.position.x && guard.position.y + 1 == obstacle.y 
                }).to_owned() {
                    point.y += 1
                }
            },
            Direction::UP => {
                if !map.obstacles.iter().any(|obstacle| {
                    obstacle.x == guard.position.x && guard.position.y - 1 == obstacle.y 
                }).to_owned() {
                    point.y -= 1
                }
            },
            Direction::RIGHT => {
                if !map.obstacles.iter().any(|obstacle| {
                    obstacle.x == guard.position.x + 1 && guard.position.y== obstacle.y 
                }).to_owned() {
                    point.x += 1
                }
            },
            Direction::LEFT => {
                if !map.obstacles.iter().any(|obstacle| {
                    obstacle.x == guard.position.x - 1 && guard.position.y == obstacle.y 
                }).to_owned() {
                    point.x -= 1
                }
            }  
        };
        point
    }

    fn next_direction(guard: &Guard, map: &Map) -> Option<Direction> {
        match guard.direction {
            Direction::DOWN => {
                if map.obstacles.iter().any(|obstacle| {
                    obstacle.x == guard.position.x && guard.position.y + 1 == obstacle.y 
                }).to_owned() {
                   return Some(Direction::LEFT)
                }
                None
            },
            Direction::UP => {
                if map.obstacles.iter().any(|obstacle| {
                    obstacle.x == guard.position.x && guard.position.y - 1 == obstacle.y 
                }).to_owned() {
                    return Some(Direction::RIGHT)
                }
                None
            },
            Direction::RIGHT => {
                if map.obstacles.iter().any(|obstacle| {
                    obstacle.x == guard.position.x + 1 && guard.position.y== obstacle.y 
                }).to_owned() {
                    return Some(Direction::DOWN)
                }
                None
            },
            Direction::LEFT => {
                if map.obstacles.iter().any(|obstacle| {
                    obstacle.x == guard.position.x - 1 && guard.position.y == obstacle.y 
                }).to_owned() {
                    return Some(Direction::UP)
                }
                None
            }  
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_none_when_guard_blocked() {
        let (map, mut guard) = read_puzzle("resources/puzzle_blocking.txt");
        assert_eq!(guard.itinerary(&map), None);
    }

    #[test]
    fn should_have_itinerary_when_guard_get_out_with_one_obstacle_left_to_down_rotate() {
        let mut guard = Guard {
            position: Point {
                x: 0,
                y: 0
            },
            direction: Direction::RIGHT
        };
        let map = Map {
            obstacles: vec![Point {
                x: 2,
                y: 0
            }],
            rows: 3,
            columns: 3
        };

        assert_eq!(guard.itinerary(&map), Some(vec![
            Point {
                x: 0,
                y: 0
            },
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
        ]));
    }

    #[test]
    fn should_have_itinerary_when_guard_have_to_turn_two_times() {
        let mut guard = Guard {
            position: Point {
                x: 1,
                y: 0
            },
            direction: Direction::RIGHT
        };
        let map = Map {
            obstacles: vec![
                Point {
                    x: 2,
                    y: 0
                },
                Point {
                    x: 1,
                    y: 1
                }
            ],
            rows: 3,
            columns: 3
        };

        assert_eq!(guard.itinerary(&map), Some(vec![
            Point {
                x: 1,
                y: 0
            },
            Point {
                x: 1,
                y: 0
            },
            Point {
                x: 0,
                y: 0
            }
        ]));
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

        assert_eq!(guard.itinerary(&map), Some(vec![
            Point {
                x: 0,
                y: 0
            },
            Point {
                x: 1,
                y: 0
            }
        ]));
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

        assert_eq!(guard.itinerary(&map), Some(vec![
            Point {
                x: 1,
                y: 0
            },
            Point {
                x: 0,
                y: 0
            }
        ]));
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

        assert_eq!(guard.itinerary(&map), Some(vec![
            Point {
                x: 1,
                y: 1
            },
            Point {
                x: 1,
                y: 0
            }
        ]));
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

        assert_eq!(guard.itinerary(&map), Some(vec![
            Point {
                x: 1,
                y: 0
            },
            Point {
                x: 1,
                y: 1
            }
        ]));
    }

    #[test]
    fn should_have_empty_itinerary_when_empty_map() {
        let mut guard = Guard::new();
        let empty_map = Map::new();

        assert_eq!(guard.itinerary(&empty_map), Some(vec![Point{
            x: 0,
            y:0
        }]));
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