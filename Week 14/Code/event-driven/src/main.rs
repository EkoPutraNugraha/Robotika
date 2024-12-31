use std::collections::VecDeque;

#[derive(Debug)]
enum Event {
    ObstacleDetected,
    TargetChanged((usize, usize)),
    Idle,
}

struct Robot {
    position: (usize, usize),
    target: (usize, usize),
    event_queue: VecDeque<Event>,
}

impl Robot {
    fn new(start: (usize, usize), target: (usize, usize)) -> Self {
        Self {
            position: start,
            target,
            event_queue: VecDeque::new(),
        }
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::ObstacleDetected => {
                println!("Obstacle detected. Adjusting path...");
                // Implement obstacle avoidance logic here
            }
            Event::TargetChanged(new_target) => {
                println!("Target changed to: {:?}", new_target);
                self.target = new_target;
            }
            Event::Idle => {
                println!("No significant changes. Maintaining current state.");
            }
        }
    }

    fn move_towards_target(&mut self) {
        if self.position != self.target {
            if self.position.0 < self.target.0 {
                self.position.0 += 1;
            } else if self.position.0 > self.target.0 {
                self.position.0 -= 1;
            }

            if self.position.1 < self.target.1 {
                self.position.1 += 1;
            } else if self.position.1 > self.target.1 {
                self.position.1 -= 1;
            }

            println!("Moving to position: {:?}", self.position);
        } else {
            println!("Reached target: {:?}", self.target);
        }
    }

    fn run(&mut self) {
        while let Some(event) = self.event_queue.pop_front() {
            self.handle_event(event);
            self.move_towards_target();
        }
    }

    fn add_event(&mut self, event: Event) {
        self.event_queue.push_back(event);
    }
}

fn main() {
    let mut robot = Robot::new((0, 0), (5, 5));

    robot.add_event(Event::Idle);
    robot.add_event(Event::ObstacleDetected);
    robot.add_event(Event::TargetChanged((2, 2)));

    robot.run();
}
