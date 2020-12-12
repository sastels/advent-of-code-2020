use std::fmt;

pub struct Ship {
    pub name: String,
    pub direction: i32,
    pub latitude: i32,
    pub longitude: i32,
}

impl fmt::Display for Ship {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} : lat {}  long {} direction: {}",
            self.name, self.latitude, self.longitude, self.direction
        )
    }
}

impl Ship {
    pub fn new(name: &str) -> Self {
        Ship {
            name: name.to_string(),
            direction: 90,
            latitude: 0,
            longitude: 0,
        }
    }

    pub fn execute_command(&mut self, command: &str) {
        let action = command.chars().nth(0).unwrap();
        let value: i32 = command[1..].parse().unwrap();

        match action {
            'N' => self.longitude += value,
            'S' => self.longitude -= value,
            'E' => self.latitude += value,
            'W' => self.latitude -= value,
            'R' => self.direction += value,
            'L' => self.direction -= value,
            'F' => match self.direction {
                0 => self.longitude += value,
                90 => self.latitude += value,
                180 => self.longitude -= value,
                270 => self.latitude -= value,
                _ => panic!(),
            },
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
    ship.latitude.abs() + ship.longitude.abs()
}

pub fn solve_b(_data: &[String]) -> usize {
    0
}
