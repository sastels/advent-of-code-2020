use std::fmt;

pub struct Ship {
    pub name: String,
    pub direction: i32,
    pub waypoint_lat: i32,
    pub waypoint_long: i32,
    pub position_lat: i32,
    pub position_long: i32,
}

impl fmt::Display for Ship {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} : waypoint: ({}, {}) position ({}, {}) direction: {}",
            self.name,
            self.waypoint_lat,
            self.waypoint_long,
            self.position_lat,
            self.position_long,
            self.direction
        )
    }
}

impl Ship {
    pub fn new(name: &str) -> Self {
        Ship {
            name: name.to_string(),
            direction: 90,
            position_lat: 0,
            position_long: 0,
            waypoint_lat: 10,
            waypoint_long: 1,
        }
    }

    pub fn execute_command(&mut self, command: &str) {
        let action = command.chars().next().unwrap();
        let value: i32 = command[1..].parse().unwrap();
        match action {
            'N' => self.position_long += value,
            'S' => self.position_long -= value,
            'E' => self.position_lat += value,
            'W' => self.position_lat -= value,
            'R' => self.direction += value,
            'L' => self.direction -= value,
            'F' => match self.direction {
                0 => self.position_long += value,
                90 => self.position_lat += value,
                180 => self.position_long -= value,
                270 => self.position_lat -= value,
                _ => panic!(),
            },
            _ => panic!(),
        }
        self.direction = ((self.direction % 360) + 360) % 360;
    }

    pub fn rotate_waypoint_r_90(&mut self) {
        let new_lat = self.waypoint_long;
        let new_long = -self.waypoint_lat;
        self.waypoint_lat = new_lat;
        self.waypoint_long = new_long;
    }

    pub fn rotate_waypoint_l_90(&mut self) {
        let new_lat = -self.waypoint_long;
        let new_long = self.waypoint_lat;
        self.waypoint_lat = new_lat;
        self.waypoint_long = new_long;
    }

    pub fn execute_command_b(&mut self, command: &str) {
        let action = command.chars().next().unwrap();
        let value: i32 = command[1..].parse().unwrap();
        match action {
            'N' => self.waypoint_long += value,
            'S' => self.waypoint_long -= value,
            'E' => self.waypoint_lat += value,
            'W' => self.waypoint_lat -= value,
            'R' => {
                for _ in 0..value / 90 {
                    let new_lat = self.waypoint_long;
                    let new_long = -self.waypoint_lat;
                    self.waypoint_lat = new_lat;
                    self.waypoint_long = new_long;
                }
            }
            'L' => {
                for _ in 0..value / 90 {
                    let new_lat = -self.waypoint_long;
                    let new_long = self.waypoint_lat;
                    self.waypoint_lat = new_lat;
                    self.waypoint_long = new_long;
                }
            }
            'F' => {
                self.position_lat += self.waypoint_lat * value;
                self.position_long += self.waypoint_long * value;
            }
            _ => panic!(),
        }
        self.direction = ((self.direction % 360) + 360) % 360;
    }
}

pub fn solve_a(data: &[String]) -> i32 {
    let mut ship = Ship::new("Bluenose");
    for command in data {
        ship.execute_command(command);
    }
    ship.position_lat.abs() + ship.position_long.abs()
}

pub fn solve_b(data: &[String]) -> i32 {
    let mut ship = Ship::new("Bluenose");
    for command in data {
        ship.execute_command_b(command);
    }
    ship.position_lat.abs() + ship.position_long.abs()
}
