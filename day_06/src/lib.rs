use ndarray::Array2;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

pub fn load_input(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Could not read from file")
}

pub fn create_map(input: &str) -> Array2<char> {
    let rows: Vec<&str> = input.lines().collect();
    let nrows = rows.len();
    let ncols = rows[0].len();

    let mut array = Array2::from_elem((nrows, ncols), ' ');

    for (i, row) in rows.iter().enumerate() {
        for (j, letter) in row.chars().enumerate() {
            array[[i, j]] = letter;
        }
    }

    array
}

pub fn find_initial_location(room_map: &Array2<char>, target: char) -> Option<Location> {
    for ((i, j), &c) in room_map.indexed_iter() {
        if c == target {
            return Some(Location {
                row: i as i32,
                column: j as i32,
            });
        }
    }
    None
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Location {
    row: i32,
    column: i32,
}

impl Location {
    pub fn update(&mut self, row_update: i32, column_update: i32) {
        self.row += row_update;
        self.column += column_update;
    }
}

pub struct Guard {
    location: Location,
    direction: Direction,
    on_map: bool,
    location_log: HashMap<i32, (Location, Direction)>,
}

impl Guard {
    pub fn new(location: Location, direction: Direction) -> Self {
        let mut guard = Self {
            location,
            direction,
            on_map: true,
            location_log: HashMap::new(),
        };

        let _ = guard.location_log.insert(1, (location, Direction::Up));

        guard
    }

    pub fn turn_right(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Right,
            Direction::Right => self.direction = Direction::Down,
            Direction::Down => self.direction = Direction::Left,
            Direction::Left => self.direction = Direction::Up,
        }
    }

    pub fn move_in_direction(&mut self, room_map: &Array2<char>) {
        match self.direction {
            Direction::Up => {
                if let Some(&cell) = room_map.get([
                    (self.location.row - 1) as usize,
                    self.location.column as usize,
                ]) {
                    if cell == '#' {
                        self.turn_right();
                    } else {
                        self.location.update(-1, 0);
                        self.location_log.insert(
                            self.location_log.len() as i32 + 1,
                            (self.location, self.direction),
                        );
                    }
                } else {
                    self.on_map = false;
                }
            }
            Direction::Right => {
                if let Some(&cell) = room_map.get([
                    self.location.row as usize,
                    (self.location.column + 1) as usize,
                ]) {
                    if cell == '#' {
                        self.turn_right();
                    } else {
                        self.location.update(0, 1);
                        self.location_log.insert(
                            self.location_log.len() as i32 + 1,
                            (self.location, self.direction),
                        );
                    }
                } else {
                    self.on_map = false;
                }
            }
            Direction::Down => {
                if let Some(&cell) = room_map.get([
                    (self.location.row + 1) as usize,
                    self.location.column as usize,
                ]) {
                    if cell == '#' {
                        self.turn_right();
                    } else {
                        self.location.update(1, 0);
                        self.location_log.insert(
                            self.location_log.len() as i32 + 1,
                            (self.location, self.direction),
                        );
                    }
                } else {
                    self.on_map = false;
                }
            }
            Direction::Left => {
                if let Some(&cell) = room_map.get([
                    self.location.row as usize,
                    (self.location.column - 1) as usize,
                ]) {
                    if cell == '#' {
                        self.turn_right();
                    } else {
                        self.location.update(0, -1);
                        self.location_log.insert(
                            self.location_log.len() as i32 + 1,
                            (self.location, self.direction),
                        );
                    }
                } else {
                    self.on_map = false;
                }
            }
        }
    }

    pub fn unique_locations_count(&self) -> usize {
        let mut unique_locations = HashSet::new();
        for &entry in self.location_log.values() {
            unique_locations.insert(entry.0);
        }
        unique_locations.len()
    }

    pub fn check_if_guard_in_loop(&self) -> bool {
        self.location_log
            .values()
            .filter(|(location, direction)| {
                self.location == *location && self.direction == *direction
            })
            .count()
            > 1
    }

    pub fn track_guard(&mut self, room_map: &Array2<char>) -> Result<(), Box<dyn Error>> {
        while self.on_map {
            self.move_in_direction(room_map);
            if self.check_if_guard_in_loop() {
                return Err("Guard in loop".into());
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indicatif::ProgressBar;
    use rayon::prelude::*;
    use std::sync::{
        atomic::{AtomicI32, Ordering},
        Arc,
    };

    #[test]
    fn day_06_part_1_simple() {
        let input = load_input("input_simple.txt");
        let room_map = create_map(&input);
        let initial_location = find_initial_location(&room_map, '^');
        let mut guard = Guard::new(initial_location.unwrap(), Direction::Up);
        let _ = guard.track_guard(&room_map);

        assert_eq!(guard.unique_locations_count(), 41);
    }

    #[test]
    fn day_06_part_1() {
        let input = load_input("input.txt");
        let room_map = create_map(&input);
        let initial_location = find_initial_location(&room_map, '^');
        let mut guard = Guard::new(initial_location.unwrap(), Direction::Up);
        let _ = guard.track_guard(&room_map);

        assert_eq!(guard.unique_locations_count(), 4977);
    }

    #[test]
    fn day_06_part_2_simple_should_fail() {
        let input = load_input("input_simple.txt");
        let mut room_map = create_map(&input);
        if let Some(cell) = room_map.get_mut((6, 3)) {
            *cell = '#';
        };

        let initial_location = find_initial_location(&room_map, '^');
        let mut guard = Guard::new(initial_location.unwrap(), Direction::Up);
        assert!(guard.track_guard(&room_map).is_err());
    }

    #[test]
    fn day_06_part_2_simple() {
        let input = load_input("input_simple.txt");
        let room_map = create_map(&input);
        let initial_location = find_initial_location(&room_map, '^');

        let mut loop_count = 0;

        room_map
            .indexed_iter()
            .filter(|(_, &c)| c == '.')
            .for_each(|((row, column), _)| {
                let mut cloned_map = room_map.clone();
                cloned_map[[row, column]] = '#';

                let mut guard = Guard::new(initial_location.unwrap(), Direction::Up);
                if guard.track_guard(&cloned_map).is_err() {
                    loop_count += 1;
                }
            });

        assert_eq!(loop_count, 6);
    }

    #[test]
    fn day_06_part_2() {
        let input = load_input("input.txt");
        let room_map = create_map(&input);
        let initial_location = find_initial_location(&room_map, '^');

        let loop_count = Arc::new(AtomicI32::new(0));
        let total_dots = room_map.iter().filter(|&&c| c == '.').count();
        let progress_bar = ProgressBar::new(total_dots as u64);

        room_map
            .indexed_iter()
            .filter(|(_, &c)| c == '.')
            .collect::<Vec<_>>()
            .par_iter()
            .for_each(|&((row, column), _)| {
                let mut cloned_map = room_map.clone();
                cloned_map[[row, column]] = '#';
                let loop_count = loop_count.clone();

                let mut guard = Guard::new(initial_location.unwrap(), Direction::Up);
                if guard.track_guard(&cloned_map).is_err() {
                    loop_count.fetch_add(1, Ordering::SeqCst);
                }

                progress_bar.inc(1);
            });

        progress_bar.finish_with_message("Done");
        assert_eq!(loop_count.load(Ordering::SeqCst), 1729);
    }
}
