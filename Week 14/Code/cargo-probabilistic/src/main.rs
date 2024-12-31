use std::collections::HashMap;
use rand::Rng;

struct Robot {
    position: (f64, f64),
    target: (f64, f64),
    sensor_uncertainty: f64, // Standard deviation for sensor noise
    movement_uncertainty: f64, // Standard deviation for movement noise
}

impl Robot {
    fn new(start: (f64, f64), target: (f64, f64), sensor_uncertainty: f64, movement_uncertainty: f64) -> Self {
        Self {
            position: start,
            target,
            sensor_uncertainty,
            movement_uncertainty,
        }
    }

    fn sense_position(&self) -> (f64, f64) {
        let mut rng = rand::thread_rng();
        let noise_x: f64 = rng.gen_range(-self.sensor_uncertainty..=self.sensor_uncertainty);
        let noise_y: f64 = rng.gen_range(-self.sensor_uncertainty..=self.sensor_uncertainty);
        (self.position.0 + noise_x, self.position.1 + noise_y)
    }

    fn move_towards_target(&mut self) {
        let mut rng = rand::thread_rng();

        let mut dx = self.target.0 - self.position.0;
        let mut dy = self.target.1 - self.position.1;

        let distance = (dx.powi(2) + dy.powi(2)).sqrt();

        if distance > 1.0 {
            dx /= distance;
            dy /= distance;
        }

        let noise_x: f64 = rng.gen_range(-self.movement_uncertainty..=self.movement_uncertainty);
        let noise_y: f64 = rng.gen_range(-self.movement_uncertainty..=self.movement_uncertainty);

        self.position.0 += dx + noise_x;
        self.position.1 += dy + noise_y;

        println!("Moved to position: {:.2}, {:.2}", self.position.0, self.position.1);
    }

    fn has_reached_target(&self) -> bool {
        let dx = self.target.0 - self.position.0;
        let dy = self.target.1 - self.position.1;
        (dx.powi(2) + dy.powi(2)).sqrt() < 0.5
    }

    fn run(&mut self) {
        while !self.has_reached_target() {
            let sensed_position = self.sense_position();
            println!("Sensed position: {:.2}, {:.2}", sensed_position.0, sensed_position.1);
            self.move_towards_target();
        }
        println!("Reached target at position: {:.2}, {:.2}", self.position.0, self.position.1);
    }
}

fn main() {
    let mut robot = Robot::new((0.0, 0.0), (10.0, 10.0), 0.5, 0.2);
    robot.run();
}
